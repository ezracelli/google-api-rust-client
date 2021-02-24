#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing a spot in the text that is dynamically replaced with content that can change over time, like a page number."]
pub struct AutoText {
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. An AutoText may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this AutoText, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this AutoText."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this auto text."]
    pub _type: ::std::option::Option<AutoTextTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of this auto text."]
pub enum AutoTextTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "An unspecified auto text type."]
    TypeUnspecified,
    #[serde(rename = "PAGE_NUMBER")]
    #[doc = "Type for auto text that represents the current page number."]
    PageNumber,
    #[serde(rename = "PAGE_COUNT")]
    #[doc = "Type for auto text that represents the total number of pages in the document."]
    PageCount,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the background of a document."]
pub struct Background {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background color."]
    pub color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base Background have been changed in this suggestion. For any field set to true, the Backgound has a new suggested value."]
pub struct BackgroundSuggestionState {
    #[serde(rename = "backgroundColorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the current background color has been modified in this suggestion."]
    pub background_color_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BatchUpdateDocument."]
pub struct BatchUpdateDocumentRequest {
    #[serde(rename = "requests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of updates to apply to the document."]
    pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Request>>>,
    #[serde(rename = "writeControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides control over how write requests are executed."]
    pub write_control: ::std::option::Option<::std::boxed::Box<WriteControl>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message from a BatchUpdateDocument request."]
pub struct BatchUpdateDocumentResponse {
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the document to which the updates were applied to."]
    pub document_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty."]
    pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Response>>>,
    #[serde(rename = "writeControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated write control after applying the request."]
    pub write_control: ::std::option::Option<::std::boxed::Box<WriteControl>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The document body. The body typically contains the full document contents except for headers, footers and footnotes."]
pub struct Body {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the body. The indexes for the body's content begin at zero."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the bullet of a paragraph."]
pub struct Bullet {
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the list this paragraph belongs to."]
    pub list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nestingLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The nesting level of this paragraph in the list."]
    pub nesting_level: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The paragraph specific text style applied to this bullet."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base Bullet have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct BulletSuggestionState {
    #[serde(rename = "listIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to the list_id."]
    pub list_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "nestingLevelSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to the nesting_level."]
    pub nesting_level_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "textStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
    pub text_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TextStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A solid color."]
pub struct Color {
    #[serde(rename = "rgbColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The RGB color value."]
    pub rgb_color: ::std::option::Option<::std::boxed::Box<RgbColor>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing a column break. A column break makes the subsequent text start at the top of the next column."]
pub struct ColumnBreak {
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A ColumnBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this ColumnBreak, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this ColumnBreak. Similar to text content, like text runs and footnote references, the text style of a column break can affect content layout as well as the styling of text inserted adjacent to it."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a Footer. The new footer is applied to the SectionStyle at the location of the SectionBreak if specificed, otherwise it is applied to the DocumentStyle. If a footer of the specified type already exists, a 400 bad request error is returned."]
pub struct CreateFooterRequest {
    #[serde(rename = "sectionBreakLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the SectionBreak immediately preceding the section whose SectionStyle this footer should belong to. If this is unset or refers to the first section break in the document, the footer applies to the document style."]
    pub section_break_location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of footer to create."]
    pub _type: ::std::option::Option<CreateFooterRequestTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of footer to create."]
pub enum CreateFooterRequestTypeEnum {
    #[serde(rename = "HEADER_FOOTER_TYPE_UNSPECIFIED")]
    #[doc = "The header/footer type is unspecified."]
    HeaderFooterTypeUnspecified,
    #[serde(rename = "DEFAULT")]
    #[doc = "A default header/footer."]
    Default,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of creating a footer."]
pub struct CreateFooterResponse {
    #[serde(rename = "footerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the created footer."]
    pub footer_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a Footnote segment and inserts a new FootnoteReference to it at the given location. The new Footnote segment will contain a space followed by a newline character."]
pub struct CreateFootnoteRequest {
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the footnote reference at the end of the document body. Footnote references cannot be inserted inside a header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the footnote reference at a specific index in the document. The footnote reference must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Footnote references cannot be inserted inside an equation, header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of creating a footnote."]
pub struct CreateFootnoteResponse {
    #[serde(rename = "footnoteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the created footnote."]
    pub footnote_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a Header. The new header is applied to the SectionStyle at the location of the SectionBreak if specificed, otherwise it is applied to the DocumentStyle. If a header of the specified type already exists, a 400 bad request error is returned."]
pub struct CreateHeaderRequest {
    #[serde(rename = "sectionBreakLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the SectionBreak which begins the section this header should belong to. If `section_break_location' is unset or if it refers to the first section break in the document body, the header applies to the DocumentStyle"]
    pub section_break_location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of header to create."]
    pub _type: ::std::option::Option<CreateHeaderRequestTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of header to create."]
pub enum CreateHeaderRequestTypeEnum {
    #[serde(rename = "HEADER_FOOTER_TYPE_UNSPECIFIED")]
    #[doc = "The header/footer type is unspecified."]
    HeaderFooterTypeUnspecified,
    #[serde(rename = "DEFAULT")]
    #[doc = "A default header/footer."]
    Default,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of creating a header."]
pub struct CreateHeaderResponse {
    #[serde(rename = "headerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the created header."]
    pub header_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a NamedRange referencing the given range."]
pub struct CreateNamedRangeRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the NamedRange. Names do not need to be unique. Names must be at least 1 character and no more than 256 characters, measured in UTF-16 code units."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range to apply the name to."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of creating a named range."]
pub struct CreateNamedRangeResponse {
    #[serde(rename = "namedRangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the created named range."]
    pub named_range_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates bullets for all of the paragraphs that overlap with the given range. The nesting level of each paragraph will be determined by counting leading tabs in front of each paragraph. To avoid excess space between the bullet and the corresponding paragraph, these leading tabs are removed by this request. This may change the indices of parts of the text. If the paragraph immediately before paragraphs being updated is in a list with a matching preset, the paragraphs being updated are added to that preceding list."]
pub struct CreateParagraphBulletsRequest {
    #[serde(rename = "bulletPreset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kinds of bullet glyphs to be used."]
    pub bullet_preset: ::std::option::Option<CreateParagraphBulletsRequestBulletPresetEnum>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range to apply the bullet preset to."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The kinds of bullet glyphs to be used."]
pub enum CreateParagraphBulletsRequestBulletPresetEnum {
    #[serde(rename = "BULLET_GLYPH_PRESET_UNSPECIFIED")]
    #[doc = "The bullet glyph preset is unspecified."]
    BulletGlyphPresetUnspecified,
    #[serde(rename = "BULLET_DISC_CIRCLE_SQUARE")]
    #[doc = "A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletDiscCircleSquare,
    #[serde(rename = "BULLET_DIAMONDX_ARROW3D_SQUARE")]
    #[doc = "A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletDiamondxArrow3DSquare,
    #[serde(rename = "BULLET_CHECKBOX")]
    #[doc = "A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels."]
    BulletCheckbox,
    #[serde(rename = "BULLET_ARROW_DIAMOND_DISC")]
    #[doc = "A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
    BulletArrowDiamondDisc,
    #[serde(rename = "BULLET_STAR_CIRCLE_SQUARE")]
    #[doc = "A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletStarCircleSquare,
    #[serde(rename = "BULLET_ARROW3D_CIRCLE_SQUARE")]
    #[doc = "A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletArrow3DCircleSquare,
    #[serde(rename = "BULLET_LEFTTRIANGLE_DIAMOND_DISC")]
    #[doc = "A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
    BulletLefttriangleDiamondDisc,
    #[serde(rename = "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE")]
    #[doc = "A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletDiamondxHollowdiamondSquare,
    #[serde(rename = "BULLET_DIAMOND_CIRCLE_SQUARE")]
    #[doc = "A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
    BulletDiamondCircleSquare,
    #[serde(rename = "NUMBERED_DECIMAL_ALPHA_ROMAN")]
    #[doc = "A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
    NumberedDecimalAlphaRoman,
    #[serde(rename = "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS")]
    #[doc = "A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by parenthesis."]
    NumberedDecimalAlphaRomanParens,
    #[serde(rename = "NUMBERED_DECIMAL_NESTED")]
    #[doc = "A numbered list with `DECIMAL` numeric glyphs separated by periods, where each nesting level uses the previous nesting level's glyph as a prefix. For example: '1.', '1.1.', '2.', '2.2.'."]
    NumberedDecimalNested,
    #[serde(rename = "NUMBERED_UPPERALPHA_ALPHA_ROMAN")]
    #[doc = "A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
    NumberedUpperalphaAlphaRoman,
    #[serde(rename = "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL")]
    #[doc = "A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DECIMAL` numeric glyphs for the first 3 list nesting levels, followed by periods."]
    NumberedUpperromanUpperalphaDecimal,
    #[serde(rename = "NUMBERED_ZERODECIMAL_ALPHA_ROMAN")]
    #[doc = "A numbered list with `ZERODECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
    NumberedZerodecimalAlphaRoman,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The crop properties of an image. The crop rectangle is represented using fractional offsets from the original content's four edges. - If the offset is in the interval (0, 1), the corresponding edge of crop rectangle is positioned inside of the image's original bounding rectangle. - If the offset is negative or greater than 1, the corresponding edge of crop rectangle is positioned outside of the image's original bounding rectangle. - If all offsets and rotation angle are 0, the image is not cropped."]
pub struct CropProperties {
    #[serde(rename = "angle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The clockwise rotation angle of the crop rectangle around its center, in radians. Rotation is applied after the offsets."]
    pub angle: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "offsetBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset specifies how far inwards the bottom edge of the crop rectangle is from the bottom edge of the original content as a fraction of the original content's height."]
    pub offset_bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "offsetLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset specifies how far inwards the left edge of the crop rectangle is from the left edge of the original content as a fraction of the original content's width."]
    pub offset_left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "offsetRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset specifies how far inwards the right edge of the crop rectangle is from the right edge of the original content as a fraction of the original content's width."]
    pub offset_right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "offsetTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset specifies how far inwards the top edge of the crop rectangle is from the top edge of the original content as a fraction of the original content's height."]
    pub offset_top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base CropProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct CropPropertiesSuggestionState {
    #[serde(rename = "angleSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to angle."]
    pub angle_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "offsetBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to offset_bottom."]
    pub offset_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "offsetLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to offset_left."]
    pub offset_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "offsetRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to offset_right."]
    pub offset_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "offsetTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to offset_top."]
    pub offset_top_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes content from the document."]
pub struct DeleteContentRangeRequest {
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range of content to delete. Deleting text that crosses a paragraph boundary may result in changes to paragraph styles, lists, positioned objects and bookmarks as the two paragraphs are merged. Attempting to delete certain ranges can result in an invalid document structure in which case a 400 bad request error is returned. Some examples of invalid delete requests include: * Deleting one code unit of a surrogate pair. * Deleting the last newline character of a Body, Header, Footer, Footnote, TableCell or TableOfContents. * Deleting the start or end of a Table, TableOfContents or Equation without deleting the entire element. * Deleting the newline character before a Table, TableOfContents or SectionBreak without deleting the element. * Deleting individual rows or cells of a table. Deleting the content within a table cell is allowed."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a Footer from the document."]
pub struct DeleteFooterRequest {
    #[serde(rename = "footerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the footer to delete. If this footer is defined on DocumentStyle, the reference to this footer is removed, resulting in no footer of that type for the first section of the document. If this footer is defined on a SectionStyle, the reference to this footer is removed and the footer of that type is now continued from the previous section."]
    pub footer_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a Header from the document."]
pub struct DeleteHeaderRequest {
    #[serde(rename = "headerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the header to delete. If this header is defined on DocumentStyle, the reference to this header is removed, resulting in no header of that type for the first section of the document. If this header is defined on a SectionStyle, the reference to this header is removed and the header of that type is now continued from the previous section."]
    pub header_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a NamedRange."]
pub struct DeleteNamedRangeRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the range(s) to delete. All named ranges with the given name will be deleted."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namedRangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the named range to delete."]
    pub named_range_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes bullets from all of the paragraphs that overlap with the given range. The nesting level of each paragraph will be visually preserved by adding indent to the start of the corresponding paragraph."]
pub struct DeleteParagraphBulletsRequest {
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range to delete bullets from."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a PositionedObject from the document."]
pub struct DeletePositionedObjectRequest {
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the positioned object to delete."]
    pub object_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a column from a table."]
pub struct DeleteTableColumnRequest {
    #[serde(rename = "tableCellLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reference table cell location from which the column will be deleted. The column this cell spans will be deleted. If this is a merged cell that spans multiple columns, all columns that the cell spans will be deleted. If no columns remain in the table after this deletion, the whole table is deleted."]
    pub table_cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deletes a row from a table."]
pub struct DeleteTableRowRequest {
    #[serde(rename = "tableCellLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reference table cell location from which the row will be deleted. The row this cell spans will be deleted. If this is a merged cell that spans multiple rows, all rows that the cell spans will be deleted. If no rows remain in the table after this deletion, the whole table is deleted."]
    pub table_cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A magnitude in a single direction in the specified units."]
pub struct Dimension {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The magnitude."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The units for magnitude."]
    pub unit: ::std::option::Option<DimensionUnitEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The units for magnitude."]
pub enum DimensionUnitEnum {
    #[serde(rename = "UNIT_UNSPECIFIED")]
    #[doc = "The units are unknown."]
    UnitUnspecified,
    #[serde(rename = "PT")]
    #[doc = "A point, 1/72 of an inch."]
    Pt,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google Docs document."]
pub struct Document {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The main body of the document."]
    pub body: ::std::option::Option<::std::boxed::Box<Body>>,
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of the document."]
    pub document_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The style of the document."]
    pub document_style: ::std::option::Option<::std::boxed::Box<DocumentStyle>>,
    #[serde(rename = "footers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The footers in the document, keyed by footer ID."]
    pub footers:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Footer>>>,
    #[serde(rename = "footnotes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The footnotes in the document, keyed by footnote ID."]
    pub footnotes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Footnote>>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The headers in the document, keyed by header ID."]
    pub headers:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Header>>>,
    #[serde(rename = "inlineObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The inline objects in the document, keyed by object ID."]
    pub inline_objects: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<InlineObject>>,
    >,
    #[serde(rename = "lists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The lists in the document, keyed by list ID."]
    pub lists: ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<List>>>,
    #[serde(rename = "namedRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The named ranges in the document, keyed by name."]
    pub named_ranges:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<NamedRanges>>>,
    #[serde(rename = "namedStyles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The named styles of the document."]
    pub named_styles: ::std::option::Option<::std::boxed::Box<NamedStyles>>,
    #[serde(rename = "positionedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The positioned objects in the document, keyed by object ID."]
    pub positioned_objects: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<PositionedObject>>,
    >,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The revision ID of the document. Can be used in update requests to specify which revision of a document to apply updates to and how the request should behave if the document has been edited since that revision. Only populated if the user has edit access to the document. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the document has not changed. Conversely, a changed ID (for the same document and user) usually means the document has been updated; however, a changed ID can also be due to internal factors such as ID format changes."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedDocumentStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The suggested changes to the style of the document, keyed by suggestion ID."]
    pub suggested_document_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedDocumentStyle>>,
    >,
    #[serde(rename = "suggestedNamedStylesChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The suggested changes to the named styles of the document, keyed by suggestion ID."]
    pub suggested_named_styles_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedNamedStyles>>,
    >,
    #[serde(rename = "suggestionsViewMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE."]
    pub suggestions_view_mode: ::std::option::Option<DocumentSuggestionsViewModeEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the document."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE."]
pub enum DocumentSuggestionsViewModeEnum {
    #[serde(rename = "DEFAULT_FOR_CURRENT_ACCESS")]
    #[doc = "The SuggestionsViewMode applied to the returned document depends on the user's current access level. If the user only has view access, PREVIEW_WITHOUT_SUGGESTIONS is applied. Otherwise, SUGGESTIONS_INLINE is applied. This is the default suggestions view mode."]
    DefaultForCurrentAccess,
    #[serde(rename = "SUGGESTIONS_INLINE")]
    #[doc = "The returned document has suggestions inline. Suggested changes will be differentiated from base content within the document. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
    SuggestionsInline,
    #[serde(rename = "PREVIEW_SUGGESTIONS_ACCEPTED")]
    #[doc = "The returned document is a preview with all suggested changes accepted. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
    PreviewSuggestionsAccepted,
    #[serde(rename = "PREVIEW_WITHOUT_SUGGESTIONS")]
    #[doc = "The returned document is a preview with all suggested changes rejected if there are any suggestions in the document."]
    PreviewWithoutSuggestions,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The style of the document."]
pub struct DocumentStyle {
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background of the document. Documents cannot have a transparent background color."]
    pub background: ::std::option::Option<::std::boxed::Box<Background>>,
    #[serde(rename = "defaultFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the default footer. If not set, there is no default footer. This property is read-only."]
    pub default_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the default header. If not set, there is no default header. This property is read-only."]
    pub default_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evenPageFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footer used only for even pages. The value of use_even_page_header_footer determines whether to use the default_footer_id or this value for the footer on even pages. If not set, there is no even page footer. This property is read-only."]
    pub even_page_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evenPageHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header used only for even pages. The value of use_even_page_header_footer determines whether to use the default_header_id or this value for the header on even pages. If not set, there is no even page header. This property is read-only."]
    pub even_page_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstPageFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footer used only for the first page. If not set then a unique footer for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_footer_id or this value for the footer on the first page. If not set, there is no first page footer. This property is read-only."]
    pub first_page_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstPageHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header used only for the first page. If not set then a unique header for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_header_id or this value for the header on the first page. If not set, there is no first page header. This property is read-only."]
    pub first_page_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marginBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bottom page margin. Updating the bottom page margin on the document style clears the bottom page margin on all section styles."]
    pub margin_bottom: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of space between the bottom of the page and the contents of the footer."]
    pub margin_footer: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of space between the top of the page and the contents of the header."]
    pub margin_header: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The left page margin. Updating the left page margin on the document style clears the left page margin on all section styles. It may also cause columns to resize in all sections."]
    pub margin_left: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The right page margin. Updating the right page margin on the document style clears the right page margin on all section styles. It may also cause columns to resize in all sections."]
    pub margin_right: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top page margin. Updating the top page margin on the document style clears the top page margin on all section styles."]
    pub margin_top: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "pageNumberStart")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page number from which to start counting the number of pages."]
    pub page_number_start: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of a page in the document."]
    pub page_size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "useCustomHeaderFooterMargins")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether DocumentStyle margin_header, SectionStyle margin_header and DocumentStyle margin_footer, SectionStyle margin_footer are respected. When false, the default values in the Docs editor for header and footer margin are used. This property is read-only."]
    pub use_custom_header_footer_margins: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useEvenPageHeaderFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether to use the even page header / footer IDs for the even pages."]
    pub use_even_page_header_footer: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useFirstPageHeaderFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether to use the first page header / footer IDs for the first page."]
    pub use_first_page_header_footer: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct DocumentStyleSuggestionState {
    #[serde(rename = "backgroundSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in background have been changed in this suggestion."]
    pub background_suggestion_state:
        ::std::option::Option<::std::boxed::Box<BackgroundSuggestionState>>,
    #[serde(rename = "defaultFooterIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to default_footer_id."]
    pub default_footer_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "defaultHeaderIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to default_header_id."]
    pub default_header_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "evenPageFooterIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to even_page_footer_id."]
    pub even_page_footer_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "evenPageHeaderIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to even_page_header_id."]
    pub even_page_header_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "firstPageFooterIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to first_page_footer_id."]
    pub first_page_footer_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "firstPageHeaderIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to first_page_header_id."]
    pub first_page_header_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_bottom."]
    pub margin_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginFooterSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_footer."]
    pub margin_footer_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginHeaderSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_header."]
    pub margin_header_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_left."]
    pub margin_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_right."]
    pub margin_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_top."]
    pub margin_top_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pageNumberStartSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to page_number_start."]
    pub page_number_start_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pageSizeSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in size have been changed in this suggestion."]
    pub page_size_suggestion_state: ::std::option::Option<::std::boxed::Box<SizeSuggestionState>>,
    #[serde(rename = "useCustomHeaderFooterMarginsSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to use_custom_header_footer_margins."]
    pub use_custom_header_footer_margins_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useEvenPageHeaderFooterSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to use_even_page_header_footer."]
    pub use_even_page_header_footer_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useFirstPageHeaderFooterSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to use_first_page_header_footer."]
    pub use_first_page_header_footer_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of an embedded drawing."]
pub struct EmbeddedDrawingProperties {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base EmbeddedDrawingProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct EmbeddedDrawingPropertiesSuggestionState {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An embedded object in the document."]
pub struct EmbeddedObject {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the embedded object. The `title` and `description` are both combined to display alt text."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "embeddedDrawingProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of an embedded drawing."]
    pub embedded_drawing_properties:
        ::std::option::Option<::std::boxed::Box<EmbeddedDrawingProperties>>,
    #[serde(rename = "embeddedObjectBorder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border of the embedded object."]
    pub embedded_object_border: ::std::option::Option<::std::boxed::Box<EmbeddedObjectBorder>>,
    #[serde(rename = "imageProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of an image."]
    pub image_properties: ::std::option::Option<::std::boxed::Box<ImageProperties>>,
    #[serde(rename = "linkedContentReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to the external linked source content. For example, it contains a reference to the source Sheets chart when the embedded object is a linked chart. If unset, then the embedded object is not linked."]
    pub linked_content_reference: ::std::option::Option<::std::boxed::Box<LinkedContentReference>>,
    #[serde(rename = "marginBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bottom margin of the embedded object."]
    pub margin_bottom: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The left margin of the embedded object."]
    pub margin_left: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The right margin of the embedded object."]
    pub margin_right: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top margin of the embedded object."]
    pub margin_top: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The visible size of the image after cropping."]
    pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the embedded object. The `title` and `description` are both combined to display alt text."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A border around an EmbeddedObject."]
pub struct EmbeddedObjectBorder {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the border."]
    pub color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "dashStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dash style of the border."]
    pub dash_style: ::std::option::Option<EmbeddedObjectBorderDashStyleEnum>,
    #[serde(rename = "propertyState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property state of the border property."]
    pub property_state: ::std::option::Option<EmbeddedObjectBorderPropertyStateEnum>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the border."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The dash style of the border."]
pub enum EmbeddedObjectBorderDashStyleEnum {
    #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
    #[doc = "Unspecified dash style."]
    DashStyleUnspecified,
    #[serde(rename = "SOLID")]
    #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
    Solid,
    #[serde(rename = "DOT")]
    #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
    Dot,
    #[serde(rename = "DASH")]
    #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
    Dash,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The property state of the border property."]
pub enum EmbeddedObjectBorderPropertyStateEnum {
    #[serde(rename = "RENDERED")]
    #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered in the document. This is the default value."]
    Rendered,
    #[serde(rename = "NOT_RENDERED")]
    #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered in the document."]
    NotRendered,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base EmbeddedObjectBorder have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct EmbeddedObjectBorderSuggestionState {
    #[serde(rename = "colorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to color."]
    pub color_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dashStyleSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to dash_style."]
    pub dash_style_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "propertyStateSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to property_state."]
    pub property_state_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "widthSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to width."]
    pub width_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base EmbeddedObject have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct EmbeddedObjectSuggestionState {
    #[serde(rename = "descriptionSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to description."]
    pub description_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "embeddedDrawingPropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in embedded_drawing_properties have been changed in this suggestion."]
    pub embedded_drawing_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<EmbeddedDrawingPropertiesSuggestionState>>,
    #[serde(rename = "embeddedObjectBorderSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in embedded_object_border have been changed in this suggestion."]
    pub embedded_object_border_suggestion_state:
        ::std::option::Option<::std::boxed::Box<EmbeddedObjectBorderSuggestionState>>,
    #[serde(rename = "imagePropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in image_properties have been changed in this suggestion."]
    pub image_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<ImagePropertiesSuggestionState>>,
    #[serde(rename = "linkedContentReferenceSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in linked_content_reference have been changed in this suggestion."]
    pub linked_content_reference_suggestion_state:
        ::std::option::Option<::std::boxed::Box<LinkedContentReferenceSuggestionState>>,
    #[serde(rename = "marginBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_bottom."]
    pub margin_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_left."]
    pub margin_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_right."]
    pub margin_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "marginTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to margin_top."]
    pub margin_top_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sizeSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in size have been changed in this suggestion."]
    pub size_suggestion_state: ::std::option::Option<::std::boxed::Box<SizeSuggestionState>>,
    #[serde(rename = "titleSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to title."]
    pub title_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment."]
pub struct EndOfSegmentLocation {
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body."]
    pub segment_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing an equation."]
pub struct Equation {
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A Equation may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A document footer."]
pub struct Footer {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the footer. The indexes for a footer's content begin at zero."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
    #[serde(rename = "footerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footer."]
    pub footer_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A document footnote."]
pub struct Footnote {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the footnote. The indexes for a footnote's content begin at zero."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
    #[serde(rename = "footnoteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footnote."]
    pub footnote_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing a footnote reference. A footnote reference is the inline content rendered with a number and is used to identify the footnote."]
pub struct FootnoteReference {
    #[serde(rename = "footnoteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footnote that contains the content of this footnote reference."]
    pub footnote_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "footnoteNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rendered number of this footnote."]
    pub footnote_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A FootnoteReference may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this FootnoteReference, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this FootnoteReference."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A document header."]
pub struct Header {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the header. The indexes for a header's content begin at zero."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
    #[serde(rename = "headerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header."]
    pub header_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing a horizontal line."]
pub struct HorizontalRule {
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A HorizontalRule may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this HorizontalRule, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this HorizontalRule. Similar to text content, like text runs and footnote references, the text style of a horizontal rule can affect content layout as well as the styling of text inserted adjacent to it."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of an image."]
pub struct ImageProperties {
    #[serde(rename = "angle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The clockwise rotation angle of the image, in radians."]
    pub angle: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "brightness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The brightness effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect."]
    pub brightness: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "contentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URI to the image with a default lifetime of 30 minutes. This URI is tagged with the account of the requester. Anyone with the URI effectively accesses the image as the original requester. Access to the image may be lost if the document's sharing settings change."]
    pub content_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contrast")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contrast effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect."]
    pub contrast: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "cropProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The crop properties of the image."]
    pub crop_properties: ::std::option::Option<::std::boxed::Box<CropProperties>>,
    #[serde(rename = "sourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source URI is the URI used to insert the image. The source URI can be empty."]
    pub source_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transparency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transparency effect of the image. The value should be in the interval [0.0, 1.0], where 0 means no effect and 1 means completely transparent."]
    pub transparency: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base ImageProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct ImagePropertiesSuggestionState {
    #[serde(rename = "angleSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to angle."]
    pub angle_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "brightnessSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to brightness."]
    pub brightness_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "contentUriSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to content_uri."]
    pub content_uri_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "contrastSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to contrast."]
    pub contrast_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "cropPropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in crop_properties have been changed in this suggestion."]
    pub crop_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<CropPropertiesSuggestionState>>,
    #[serde(rename = "sourceUriSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to source_uri."]
    pub source_uri_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "transparencySuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to transparency."]
    pub transparency_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that appears inline with text. An InlineObject contains an EmbeddedObject such as an image."]
pub struct InlineObject {
    #[serde(rename = "inlineObjectProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of this inline object."]
    pub inline_object_properties: ::std::option::Option<::std::boxed::Box<InlineObjectProperties>>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this inline object."]
    pub object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInlineObjectPropertiesChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested changes to the inline object properties, keyed by suggestion ID."]
    pub suggested_inline_object_properties_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedInlineObjectProperties>>,
    >,
    #[serde(rename = "suggestedInsertionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement that contains an InlineObject."]
pub struct InlineObjectElement {
    #[serde(rename = "inlineObjectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the InlineObject this element contains."]
    pub inline_object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. An InlineObjectElement may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this InlineObject, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this InlineObjectElement. Similar to text content, like text runs and footnote references, the text style of an inline object element can affect content layout as well as the styling of text inserted adjacent to it."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Properties of an InlineObject."]
pub struct InlineObjectProperties {
    #[serde(rename = "embeddedObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The embedded object of this inline object."]
    pub embedded_object: ::std::option::Option<::std::boxed::Box<EmbeddedObject>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base InlineObjectProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct InlineObjectPropertiesSuggestionState {
    #[serde(rename = "embeddedObjectSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in embedded_object have been changed in this suggestion."]
    pub embedded_object_suggestion_state:
        ::std::option::Option<::std::boxed::Box<EmbeddedObjectSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts an InlineObject containing an image at the given location."]
pub struct InsertInlineImageRequest {
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the text at the end of a header, footer or the document body. Inline images cannot be inserted inside a footnote."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the image at a specific index in the document. The image must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Inline images cannot be inserted inside a footnote or equation."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "objectSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size that the image should appear as in the document. This property is optional and the final size of the image in the document is determined by the following rules: * If neither width nor height is specified, then a default size of the image is calculated based on its resolution. * If one dimension is specified then the other dimension is calculated to preserve the aspect ratio of the image. * If both width and height are specified, the image is scaled to fit within the provided dimensions while maintaining its aspect ratio."]
    pub object_size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image URI. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI can be at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.content_uri field."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of inserting an inline image."]
pub struct InsertInlineImageResponse {
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the created InlineObject."]
    pub object_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of inserting an embedded Google Sheets chart."]
pub struct InsertInlineSheetsChartResponse {
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object ID of the inserted chart."]
    pub object_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts a page break followed by a newline at the specified location."]
pub struct InsertPageBreakRequest {
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the page break at the end of the document body. Page breaks cannot be inserted inside a footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the page break at a specific index in the document. The page break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Page breaks cannot be inserted inside a table, equation, footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts a section break at the given location. A newline character will be inserted before the section break."]
pub struct InsertSectionBreakRequest {
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts a newline and a section break at the end of the document body. Section breaks cannot be inserted inside a footnote, header or footer. Because section breaks can only be inserted inside the body, the segment ID field must be empty."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts a newline and a section break at a specific index in the document. The section break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Section breaks cannot be inserted inside a table, equation, footnote, header, or footer. Since section breaks can only be inserted inside the body, the segment ID field must be empty."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "sectionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of section to insert."]
    pub section_type: ::std::option::Option<InsertSectionBreakRequestSectionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of section to insert."]
pub enum InsertSectionBreakRequestSectionTypeEnum {
    #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
    #[doc = "The section type is unspecified."]
    SectionTypeUnspecified,
    #[serde(rename = "CONTINUOUS")]
    #[doc = "The section starts immediately after the last paragraph of the previous section."]
    Continuous,
    #[serde(rename = "NEXT_PAGE")]
    #[doc = "The section starts on the next page."]
    NextPage,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts an empty column into a table."]
pub struct InsertTableColumnRequest {
    #[serde(rename = "insertRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to insert new column to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left."]
    pub insert_right: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tableCellLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reference table cell location from which columns will be inserted. A new column will be inserted to the left (or right) of the column where the reference cell is. If the reference cell is a merged cell, a new column will be inserted to the left (or right) of the merged cell."]
    pub table_cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts a table at the specified location. A newline character will be inserted before the inserted table."]
pub struct InsertTableRequest {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of columns in the table."]
    pub columns: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the table at the end of the given header, footer or document body. A newline character will be inserted before the inserted table. Tables cannot be inserted inside a footnote."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the table at a specific model index. A newline character will be inserted before the inserted table, therefore the table start index will be at the specified location index + 1. The table must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between an existing table and its preceding paragraph). Tables cannot be inserted inside a footnote or equation."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of rows in the table."]
    pub rows: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts an empty row into a table."]
pub struct InsertTableRowRequest {
    #[serde(rename = "insertBelow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to insert new row below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell."]
    pub insert_below: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tableCellLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reference table cell location from which rows will be inserted. A new row will be inserted above (or below) the row where the reference cell is. If the reference cell is a merged cell, a new row will be inserted above (or below) the merged cell."]
    pub table_cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inserts text at the specified location."]
pub struct InsertTextRequest {
    #[serde(rename = "endOfSegmentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the text at the end of a header, footer, footnote or the document body."]
    pub end_of_segment_location: ::std::option::Option<::std::boxed::Box<EndOfSegmentLocation>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts the text at a specific index in the document. Text must be inserted inside the bounds of an existing Paragraph. For instance, text cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). The text must be inserted in the preceding paragraph."]
    pub location: ::std::option::Option<::std::boxed::Box<Location>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to be inserted. Inserting a newline character will implicitly create a new Paragraph at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text style for the inserted text will match the text immediately before the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to another portion of a document or an external URL resource."]
pub struct Link {
    #[serde(rename = "bookmarkId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of a bookmark in this document."]
    pub bookmark_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of a heading in this document."]
    pub heading_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An external URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to the external linked source content."]
pub struct LinkedContentReference {
    #[serde(rename = "sheetsChartReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to the linked chart."]
    pub sheets_chart_reference: ::std::option::Option<::std::boxed::Box<SheetsChartReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base LinkedContentReference have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct LinkedContentReferenceSuggestionState {
    #[serde(rename = "sheetsChartReferenceSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in sheets_chart_reference have been changed in this suggestion."]
    pub sheets_chart_reference_suggestion_state:
        ::std::option::Option<::std::boxed::Box<SheetsChartReferenceSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A List represents the list attributes for a group of paragraphs that all belong to the same list. A paragraph that is part of a list has a reference to the list's ID in its bullet."]
pub struct List {
    #[serde(rename = "listProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of the list."]
    pub list_properties: ::std::option::Option<::std::boxed::Box<ListProperties>>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this list."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedListPropertiesChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested changes to the list properties, keyed by suggestion ID."]
    pub suggested_list_properties_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedListProperties>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of a list which describe the look and feel of bullets belonging to paragraphs associated with a list."]
pub struct ListProperties {
    #[serde(rename = "nestingLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes the properties of the bullets at the associated level. A list has at most nine levels of nesting with nesting level 0 corresponding to the top-most level and nesting level 8 corresponding to the most nested level. The nesting levels are returned in ascending order with the least nested returned first."]
    pub nesting_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestingLevel>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct ListPropertiesSuggestionState {
    #[serde(rename = "nestingLevelsSuggestionStates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the corresponding NestingLevel in nesting_levels have been changed in this suggestion. The nesting level suggestion states are returned in ascending order of the nesting level with the least nested returned first."]
    pub nesting_levels_suggestion_states:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestingLevelSuggestionState>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A particular location in the document."]
pub struct Location {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id."]
    pub index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body."]
    pub segment_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Merges cells in a Table."]
pub struct MergeTableCellsRequest {
    #[serde(rename = "tableRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The table range specifying which cells of the table to merge. Any text in the cells being merged will be concatenated and stored in the \"head\" cell of the range. This is the upper-left cell of the range when the content direction is left to right, and the upper-right cell of the range otherwise. If the range is non-rectangular (which can occur in some cases where the range covers cells that are already merged or where the table is non-rectangular), a 400 bad request error is returned."]
    pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of Ranges with the same named range ID. Named ranges allow developers to associate parts of a document with an arbitrary user-defined label so their contents can be programmatically read or edited at a later time. A document can contain multiple named ranges with the same name, but every named range has a unique ID. A named range is created with a single Range, and content inserted inside a named range generally expands that range. However, certain document changes can cause the range to be split into multiple ranges. Named ranges are not private. All applications and collaborators that have access to the document can see its named ranges."]
pub struct NamedRange {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the named range."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namedRangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the named range."]
    pub named_range_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ranges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ranges that belong to this named range."]
    pub ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Range>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of all the NamedRanges in the document that share a given name."]
pub struct NamedRanges {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name that all the named ranges share."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namedRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The NamedRanges that share the same name."]
    pub named_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedRange>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A named style. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from this named style when they have the same named style type."]
pub struct NamedStyle {
    #[serde(rename = "namedStyleType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this named style."]
    pub named_style_type: ::std::option::Option<NamedStyleNamedStyleTypeEnum>,
    #[serde(rename = "paragraphStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The paragraph style of this named style."]
    pub paragraph_style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this named style."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of this named style."]
pub enum NamedStyleNamedStyleTypeEnum {
    #[serde(rename = "NAMED_STYLE_TYPE_UNSPECIFIED")]
    #[doc = "The type of named style is unspecified."]
    NamedStyleTypeUnspecified,
    #[serde(rename = "NORMAL_TEXT")]
    #[doc = "Normal text."]
    NormalText,
    #[serde(rename = "TITLE")]
    #[doc = "Title."]
    Title,
    #[serde(rename = "SUBTITLE")]
    #[doc = "Subtitle."]
    Subtitle,
    #[serde(rename = "HEADING_1")]
    #[doc = "Heading 1."]
    Heading1,
    #[serde(rename = "HEADING_2")]
    #[doc = "Heading 2."]
    Heading2,
    #[serde(rename = "HEADING_3")]
    #[doc = "Heading 3."]
    Heading3,
    #[serde(rename = "HEADING_4")]
    #[doc = "Heading 4."]
    Heading4,
    #[serde(rename = "HEADING_5")]
    #[doc = "Heading 5."]
    Heading5,
    #[serde(rename = "HEADING_6")]
    #[doc = "Heading 6."]
    Heading6,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggestion state of a NamedStyle message."]
pub struct NamedStyleSuggestionState {
    #[serde(rename = "namedStyleType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The named style type that this suggestion state corresponds to. This field is provided as a convenience for matching the NamedStyleSuggestionState with its corresponding NamedStyle."]
    pub named_style_type: ::std::option::Option<NamedStyleSuggestionStateNamedStyleTypeEnum>,
    #[serde(rename = "paragraphStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in paragraph style have been changed in this suggestion."]
    pub paragraph_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<ParagraphStyleSuggestionState>>,
    #[serde(rename = "textStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
    pub text_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TextStyleSuggestionState>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The named style type that this suggestion state corresponds to. This field is provided as a convenience for matching the NamedStyleSuggestionState with its corresponding NamedStyle."]
pub enum NamedStyleSuggestionStateNamedStyleTypeEnum {
    #[serde(rename = "NAMED_STYLE_TYPE_UNSPECIFIED")]
    #[doc = "The type of named style is unspecified."]
    NamedStyleTypeUnspecified,
    #[serde(rename = "NORMAL_TEXT")]
    #[doc = "Normal text."]
    NormalText,
    #[serde(rename = "TITLE")]
    #[doc = "Title."]
    Title,
    #[serde(rename = "SUBTITLE")]
    #[doc = "Subtitle."]
    Subtitle,
    #[serde(rename = "HEADING_1")]
    #[doc = "Heading 1."]
    Heading1,
    #[serde(rename = "HEADING_2")]
    #[doc = "Heading 2."]
    Heading2,
    #[serde(rename = "HEADING_3")]
    #[doc = "Heading 3."]
    Heading3,
    #[serde(rename = "HEADING_4")]
    #[doc = "Heading 4."]
    Heading4,
    #[serde(rename = "HEADING_5")]
    #[doc = "Heading 5."]
    Heading5,
    #[serde(rename = "HEADING_6")]
    #[doc = "Heading 6."]
    Heading6,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The named styles. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from these named styles."]
pub struct NamedStyles {
    #[serde(rename = "styles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The named styles. There is an entry for each of the possible named style types."]
    pub styles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedStyle>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The suggestion state of a NamedStyles message."]
pub struct NamedStylesSuggestionState {
    #[serde(rename = "stylesSuggestionStates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the corresponding NamedStyle in styles have been changed in this suggestion. The order of these named style suggestion states match the order of the corresponding named style within the named styles suggestion."]
    pub styles_suggestion_states:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedStyleSuggestionState>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties describing the look and feel of a list bullet at a given level of nesting."]
pub struct NestingLevel {
    #[serde(rename = "bulletAlignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alignment of the bullet within the space allotted for rendering the bullet."]
    pub bullet_alignment: ::std::option::Option<NestingLevelBulletAlignmentEnum>,
    #[serde(rename = "glyphFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The format string used by bullets at this level of nesting. The glyph format contains one or more placeholders, and these placeholder are replaced with the appropriate values depending on the glyph_type or glyph_symbol. The placeholders follow the pattern `%[nesting_level]`. Furthermore, placeholders can have prefixes and suffixes. Thus, the glyph format follows the pattern `%[nesting_level]`. Note that the prefix and suffix are optional and can be arbitrary strings. For example, the glyph format `%0.` indicates that the rendered glyph will replace the placeholder with the corresponding glyph for nesting level 0 followed by a period as the suffix. So a list with a glyph type of UPPER_ALPHA and glyph format `%0.` at nesting level 0 will result in a list with rendered glyphs `A.` `B.` `C.` The glyph format can contain placeholders for the current nesting level as well as placeholders for parent nesting levels. For example, a list can have a glyph format of `%0.` at nesting level 0 and a glyph format of `%0.%1.` at nesting level 1. Assuming both nesting levels have DECIMAL glyph types, this would result in a list with rendered glyphs `1.` `2.` ` 2.1.` ` 2.2.` `3.` For nesting levels that are ordered, the string that replaces a placeholder in the glyph format for a particular paragraph depends on the paragraph's order within the list."]
    pub glyph_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "glyphSymbol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A custom glyph symbol used by bullets when paragraphs at this level of nesting are unordered. The glyph symbol replaces placeholders within the glyph_format. For example, if the glyph_symbol is the solid circle corresponding to Unicode U+25cf code point and the glyph_format is `%0`, the rendered glyph would be the solid circle."]
    pub glyph_symbol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "glyphType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of glyph used by bullets when paragraphs at this level of nesting are ordered. The glyph type determines the type of glyph used to replace placeholders within the glyph_format when paragraphs at this level of nesting are ordered. For example, if the nesting level is 0, the glyph_format is `%0.` and the glyph type is DECIMAL, then the rendered glyph would replace the placeholder `%0` in the glyph format with a number corresponding to list item's order within the list."]
    pub glyph_type: ::std::option::Option<NestingLevelGlyphTypeEnum>,
    #[serde(rename = "indentFirstLine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of indentation for the first line of paragraphs at this level of nesting."]
    pub indent_first_line: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "indentStart")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of indentation for paragraphs at this level of nesting. Applied to the side that corresponds to the start of the text, based on the paragraph's content direction."]
    pub indent_start: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "startNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of the first list item at this nesting level. A value of 0 is treated as a value of 1 for lettered lists and roman numeraled lists, i.e. for values of both 0 and 1, lettered and roman numeraled lists will begin at `a` and `i` respectively. This value is ignored for nesting levels with unordered glyphs."]
    pub start_number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of bullets at this level of nesting."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alignment of the bullet within the space allotted for rendering the bullet."]
pub enum NestingLevelBulletAlignmentEnum {
    #[serde(rename = "BULLET_ALIGNMENT_UNSPECIFIED")]
    #[doc = "The bullet alignment is unspecified."]
    BulletAlignmentUnspecified,
    #[serde(rename = "START")]
    #[doc = "The bullet is aligned to the start of the space allotted for rendering the bullet. Left-aligned for LTR text, right-aligned otherwise."]
    Start,
    #[serde(rename = "CENTER")]
    #[doc = "The bullet is aligned to the center of the space allotted for rendering the bullet."]
    Center,
    #[serde(rename = "END")]
    #[doc = "The bullet is aligned to the end of the space allotted for rendering the bullet. Right-aligned for LTR text, left-aligned otherwise."]
    End,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of glyph used by bullets when paragraphs at this level of nesting are ordered. The glyph type determines the type of glyph used to replace placeholders within the glyph_format when paragraphs at this level of nesting are ordered. For example, if the nesting level is 0, the glyph_format is `%0.` and the glyph type is DECIMAL, then the rendered glyph would replace the placeholder `%0` in the glyph format with a number corresponding to list item's order within the list."]
pub enum NestingLevelGlyphTypeEnum {
    #[serde(rename = "GLYPH_TYPE_UNSPECIFIED")]
    #[doc = "The glyph type is unspecified or unsupported."]
    GlyphTypeUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "An empty string."]
    None,
    #[serde(rename = "DECIMAL")]
    #[doc = "A number, like `1`, `2`, or `3`."]
    Decimal,
    #[serde(rename = "ZERO_DECIMAL")]
    #[doc = "A number where single digit numbers are prefixed with a zero, like `01`, `02`, or `03`. Numbers with more than one digit are not prefixed with a zero."]
    ZeroDecimal,
    #[serde(rename = "UPPER_ALPHA")]
    #[doc = "An uppercase letter, like `A`, `B`, or `C`."]
    UpperAlpha,
    #[serde(rename = "ALPHA")]
    #[doc = "A lowercase letter, like `a`, `b`, or `c`."]
    Alpha,
    #[serde(rename = "UPPER_ROMAN")]
    #[doc = "An uppercase Roman numeral, like `I`, `II`, or `III`."]
    UpperRoman,
    #[serde(rename = "ROMAN")]
    #[doc = "A lowercase Roman numeral, like `i`, `ii`, or `iii`."]
    Roman,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base NestingLevel have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct NestingLevelSuggestionState {
    #[serde(rename = "bulletAlignmentSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to bullet_alignment."]
    pub bullet_alignment_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "glyphFormatSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to glyph_format."]
    pub glyph_format_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "glyphSymbolSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to glyph_symbol."]
    pub glyph_symbol_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "glyphTypeSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to glyph_type."]
    pub glyph_type_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "indentFirstLineSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to indent_first_line."]
    pub indent_first_line_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "indentStartSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to indent_start."]
    pub indent_start_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startNumberSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to start_number."]
    pub start_number_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "textStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
    pub text_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TextStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of object IDs."]
pub struct ObjectReferences {
    #[serde(rename = "objectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object IDs."]
    pub object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A color that can either be fully opaque or fully transparent."]
pub struct OptionalColor {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, this will be used as an opaque color. If unset, this represents a transparent color."]
    pub color: ::std::option::Option<::std::boxed::Box<Color>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement representing a page break. A page break makes the subsequent text start at the top of the next page."]
pub struct PageBreak {
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A PageBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this PageBreak, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this PageBreak. Similar to text content, like text runs and footnote references, the text style of a page break can affect content layout as well as the styling of text inserted adjacent to it."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A StructuralElement representing a paragraph. A paragraph is a range of content that is terminated with a newline character."]
pub struct Paragraph {
    #[serde(rename = "bullet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bullet for this paragraph. If not present, the paragraph does not belong to a list."]
    pub bullet: ::std::option::Option<::std::boxed::Box<Bullet>>,
    #[serde(rename = "elements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the paragraph broken down into its component parts."]
    pub elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParagraphElement>>>,
    #[serde(rename = "paragraphStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of this paragraph."]
    pub paragraph_style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
    #[serde(rename = "positionedObjectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the positioned objects tethered to this paragraph."]
    pub positioned_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedBulletChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested changes to this paragraph's bullet."]
    pub suggested_bullet_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedBullet>>,
    >,
    #[serde(rename = "suggestedParagraphStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested paragraph style changes to this paragraph, keyed by suggestion ID."]
    pub suggested_paragraph_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedParagraphStyle>>,
    >,
    #[serde(rename = "suggestedPositionedObjectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the positioned objects that are suggested to be attached to this paragraph, keyed by suggestion ID."]
    pub suggested_positioned_object_ids: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<ObjectReferences>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A border around a paragraph."]
pub struct ParagraphBorder {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the border."]
    pub color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "dashStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dash style of the border."]
    pub dash_style: ::std::option::Option<ParagraphBorderDashStyleEnum>,
    #[serde(rename = "padding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The padding of the border."]
    pub padding: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the border."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The dash style of the border."]
pub enum ParagraphBorderDashStyleEnum {
    #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
    #[doc = "Unspecified dash style."]
    DashStyleUnspecified,
    #[serde(rename = "SOLID")]
    #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
    Solid,
    #[serde(rename = "DOT")]
    #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
    Dot,
    #[serde(rename = "DASH")]
    #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
    Dash,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement describes content within a Paragraph."]
pub struct ParagraphElement {
    #[serde(rename = "autoText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An auto text paragraph element."]
    pub auto_text: ::std::option::Option<::std::boxed::Box<AutoText>>,
    #[serde(rename = "columnBreak")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column break paragraph element."]
    pub column_break: ::std::option::Option<::std::boxed::Box<ColumnBreak>>,
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-base end index of this paragraph element, exclusive, in UTF-16 code units."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "equation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An equation paragraph element."]
    pub equation: ::std::option::Option<::std::boxed::Box<Equation>>,
    #[serde(rename = "footnoteReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A footnote reference paragraph element."]
    pub footnote_reference: ::std::option::Option<::std::boxed::Box<FootnoteReference>>,
    #[serde(rename = "horizontalRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A horizontal rule paragraph element."]
    pub horizontal_rule: ::std::option::Option<::std::boxed::Box<HorizontalRule>>,
    #[serde(rename = "inlineObjectElement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An inline object paragraph element."]
    pub inline_object_element: ::std::option::Option<::std::boxed::Box<InlineObjectElement>>,
    #[serde(rename = "pageBreak")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page break paragraph element."]
    pub page_break: ::std::option::Option<::std::boxed::Box<PageBreak>>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based start index of this paragraph element, in UTF-16 code units."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "textRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text run paragraph element."]
    pub text_run: ::std::option::Option<::std::boxed::Box<TextRun>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Styles that apply to a whole paragraph. Inherited paragraph styles are represented as unset fields in this message. A paragraph style's parent depends on where the paragraph style is defined: * The ParagraphStyle on a Paragraph inherits from the paragraph's corresponding named style type. * The ParagraphStyle on a named style inherits from the normal text named style. * The ParagraphStyle of the normal text named style inherits from the default paragraph style in the Docs editor. * The ParagraphStyle on a Paragraph element that is contained in a table may inherit its paragraph style from the table style. If the paragraph style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor."]
pub struct ParagraphStyle {
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text alignment for this paragraph."]
    pub alignment: ::std::option::Option<ParagraphStyleAlignmentEnum>,
    #[serde(rename = "avoidWidowAndOrphan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to avoid widows and orphans for the paragraph. If unset, the value is inherited from the parent."]
    pub avoid_widow_and_orphan: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderBetween")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border between this paragraph and the next and previous paragraphs. If unset, the value is inherited from the parent. The between border is rendered when the adjacent paragraph has the same border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
    pub border_between: ::std::option::Option<::std::boxed::Box<ParagraphBorder>>,
    #[serde(rename = "borderBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border at the bottom of this paragraph. If unset, the value is inherited from the parent. The bottom border is rendered when the paragraph below has different border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
    pub border_bottom: ::std::option::Option<::std::boxed::Box<ParagraphBorder>>,
    #[serde(rename = "borderLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border to the left of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
    pub border_left: ::std::option::Option<::std::boxed::Box<ParagraphBorder>>,
    #[serde(rename = "borderRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border to the right of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
    pub border_right: ::std::option::Option<::std::boxed::Box<ParagraphBorder>>,
    #[serde(rename = "borderTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The border at the top of this paragraph. If unset, the value is inherited from the parent. The top border is rendered when the paragraph above has different border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
    pub border_top: ::std::option::Option<::std::boxed::Box<ParagraphBorder>>,
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited."]
    pub direction: ::std::option::Option<ParagraphStyleDirectionEnum>,
    #[serde(rename = "headingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The heading ID of the paragraph. If empty, then this paragraph is not a heading. This property is read-only."]
    pub heading_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "indentEnd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of indentation for the paragraph on the side that corresponds to the end of the text, based on the current paragraph direction. If unset, the value is inherited from the parent."]
    pub indent_end: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "indentFirstLine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of indentation for the first line of the paragraph. If unset, the value is inherited from the parent."]
    pub indent_first_line: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "indentStart")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of indentation for the paragraph on the side that corresponds to the start of the text, based on the current paragraph direction. If unset, the value is inherited from the parent."]
    pub indent_start: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "keepLinesTogether")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether all lines of the paragraph should be laid out on the same page or column if possible. If unset, the value is inherited from the parent."]
    pub keep_lines_together: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keepWithNext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether at least a part of this paragraph should be laid out on the same page or column as the next paragraph if possible. If unset, the value is inherited from the parent."]
    pub keep_with_next: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lineSpacing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent."]
    pub line_spacing: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "namedStyleType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated."]
    pub named_style_type: ::std::option::Option<ParagraphStyleNamedStyleTypeEnum>,
    #[serde(rename = "shading")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shading of the paragraph. If unset, the value is inherited from the parent."]
    pub shading: ::std::option::Option<::std::boxed::Box<Shading>>,
    #[serde(rename = "spaceAbove")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of extra space above the paragraph. If unset, the value is inherited from the parent."]
    pub space_above: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "spaceBelow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of extra space below the paragraph. If unset, the value is inherited from the parent."]
    pub space_below: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "spacingMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The spacing mode for the paragraph."]
    pub spacing_mode: ::std::option::Option<ParagraphStyleSpacingModeEnum>,
    #[serde(rename = "tabStops")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the tab stops for this paragraph. The list of tab stops is not inherited. This property is read-only."]
    pub tab_stops: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TabStop>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The text alignment for this paragraph."]
pub enum ParagraphStyleAlignmentEnum {
    #[serde(rename = "ALIGNMENT_UNSPECIFIED")]
    #[doc = "The paragraph alignment is inherited from the parent."]
    AlignmentUnspecified,
    #[serde(rename = "START")]
    #[doc = "The paragraph is aligned to the start of the line. Left-aligned for LTR text, right-aligned otherwise."]
    Start,
    #[serde(rename = "CENTER")]
    #[doc = "The paragraph is centered."]
    Center,
    #[serde(rename = "END")]
    #[doc = "The paragraph is aligned to the end of the line. Right-aligned for LTR text, left-aligned otherwise."]
    End,
    #[serde(rename = "JUSTIFIED")]
    #[doc = "The paragraph is justified."]
    Justified,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited."]
pub enum ParagraphStyleDirectionEnum {
    #[serde(rename = "CONTENT_DIRECTION_UNSPECIFIED")]
    #[doc = "The content direction is unspecified."]
    ContentDirectionUnspecified,
    #[serde(rename = "LEFT_TO_RIGHT")]
    #[doc = "The content goes from left to right."]
    LeftToRight,
    #[serde(rename = "RIGHT_TO_LEFT")]
    #[doc = "The content goes from right to left."]
    RightToLeft,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated."]
pub enum ParagraphStyleNamedStyleTypeEnum {
    #[serde(rename = "NAMED_STYLE_TYPE_UNSPECIFIED")]
    #[doc = "The type of named style is unspecified."]
    NamedStyleTypeUnspecified,
    #[serde(rename = "NORMAL_TEXT")]
    #[doc = "Normal text."]
    NormalText,
    #[serde(rename = "TITLE")]
    #[doc = "Title."]
    Title,
    #[serde(rename = "SUBTITLE")]
    #[doc = "Subtitle."]
    Subtitle,
    #[serde(rename = "HEADING_1")]
    #[doc = "Heading 1."]
    Heading1,
    #[serde(rename = "HEADING_2")]
    #[doc = "Heading 2."]
    Heading2,
    #[serde(rename = "HEADING_3")]
    #[doc = "Heading 3."]
    Heading3,
    #[serde(rename = "HEADING_4")]
    #[doc = "Heading 4."]
    Heading4,
    #[serde(rename = "HEADING_5")]
    #[doc = "Heading 5."]
    Heading5,
    #[serde(rename = "HEADING_6")]
    #[doc = "Heading 6."]
    Heading6,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The spacing mode for the paragraph."]
pub enum ParagraphStyleSpacingModeEnum {
    #[serde(rename = "SPACING_MODE_UNSPECIFIED")]
    #[doc = "The spacing mode is inherited from the parent."]
    SpacingModeUnspecified,
    #[serde(rename = "NEVER_COLLAPSE")]
    #[doc = "Paragraph spacing is always rendered."]
    NeverCollapse,
    #[serde(rename = "COLLAPSE_LISTS")]
    #[doc = "Paragraph spacing is skipped between list elements."]
    CollapseLists,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct ParagraphStyleSuggestionState {
    #[serde(rename = "alignmentSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to alignment."]
    pub alignment_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "avoidWidowAndOrphanSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to avoid_widow_and_orphan."]
    pub avoid_widow_and_orphan_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderBetweenSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_between."]
    pub border_between_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_bottom."]
    pub border_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_left."]
    pub border_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_right."]
    pub border_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_top."]
    pub border_top_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "directionSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to direction."]
    pub direction_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "headingIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to heading_id."]
    pub heading_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "indentEndSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to indent_end."]
    pub indent_end_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "indentFirstLineSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to indent_first_line."]
    pub indent_first_line_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "indentStartSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to indent_start."]
    pub indent_start_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keepLinesTogetherSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to keep_lines_together."]
    pub keep_lines_together_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keepWithNextSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to keep_with_next."]
    pub keep_with_next_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lineSpacingSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to line_spacing."]
    pub line_spacing_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "namedStyleTypeSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to named_style_type."]
    pub named_style_type_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "shadingSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in shading have been changed in this suggestion."]
    pub shading_suggestion_state: ::std::option::Option<::std::boxed::Box<ShadingSuggestionState>>,
    #[serde(rename = "spaceAboveSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to space_above."]
    pub space_above_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "spaceBelowSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to space_below."]
    pub space_below_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "spacingModeSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to spacing_mode."]
    pub spacing_mode_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that is tethered to a Paragraph and positioned relative to the beginning of the paragraph. A PositionedObject contains an EmbeddedObject such as an image."]
pub struct PositionedObject {
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this positioned object."]
    pub object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "positionedObjectProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of this positioned object."]
    pub positioned_object_properties:
        ::std::option::Option<::std::boxed::Box<PositionedObjectProperties>>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedPositionedObjectPropertiesChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested changes to the positioned object properties, keyed by suggestion ID."]
    pub suggested_positioned_object_properties_changes: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<SuggestedPositionedObjectProperties>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The positioning of a PositionedObject. The positioned object is positioned relative to the beginning of the Paragraph it is tethered to."]
pub struct PositionedObjectPositioning {
    #[serde(rename = "layout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The layout of this positioned object."]
    pub layout: ::std::option::Option<PositionedObjectPositioningLayoutEnum>,
    #[serde(rename = "leftOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset of the left edge of the positioned object relative to the beginning of the Paragraph it is tethered to. The exact positioning of the object can depend on other content in the document and the document's styling."]
    pub left_offset: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "topOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset of the top edge of the positioned object relative to the beginning of the Paragraph it is tethered to. The exact positioning of the object can depend on other content in the document and the document's styling."]
    pub top_offset: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The layout of this positioned object."]
pub enum PositionedObjectPositioningLayoutEnum {
    #[serde(rename = "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED")]
    #[doc = "The layout is unspecified."]
    PositionedObjectLayoutUnspecified,
    #[serde(rename = "WRAP_TEXT")]
    #[doc = "The text wraps around the positioned object."]
    WrapText,
    #[serde(rename = "BREAK_LEFT")]
    #[doc = "Breaks text such that the positioned object is on the left and text is on the right."]
    BreakLeft,
    #[serde(rename = "BREAK_RIGHT")]
    #[doc = "Breaks text such that the positioned object is on the right and text is on the left."]
    BreakRight,
    #[serde(rename = "BREAK_LEFT_RIGHT")]
    #[doc = "Breaks text such that there is no text on the left or right of the positioned object."]
    BreakLeftRight,
    #[serde(rename = "IN_FRONT_OF_TEXT")]
    #[doc = "The positioned object is in front of the text."]
    InFrontOfText,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base PositionedObjectPositioning have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct PositionedObjectPositioningSuggestionState {
    #[serde(rename = "layoutSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to layout."]
    pub layout_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "leftOffsetSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to left_offset."]
    pub left_offset_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "topOffsetSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to top_offset."]
    pub top_offset_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Properties of a PositionedObject."]
pub struct PositionedObjectProperties {
    #[serde(rename = "embeddedObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The embedded object of this positioned object."]
    pub embedded_object: ::std::option::Option<::std::boxed::Box<EmbeddedObject>>,
    #[serde(rename = "positioning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The positioning of this positioned object relative to the newline of the Paragraph that references this positioned object."]
    pub positioning: ::std::option::Option<::std::boxed::Box<PositionedObjectPositioning>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base PositionedObjectProperties have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct PositionedObjectPropertiesSuggestionState {
    #[serde(rename = "embeddedObjectSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in embedded_object have been changed in this suggestion."]
    pub embedded_object_suggestion_state:
        ::std::option::Option<::std::boxed::Box<EmbeddedObjectSuggestionState>>,
    #[serde(rename = "positioningSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields in positioning have been changed in this suggestion."]
    pub positioning_suggestion_state:
        ::std::option::Option<::std::boxed::Box<PositionedObjectPositioningSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies a contiguous range of text."]
pub struct Range {
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header, footer or footnote that this range is contained in. An empty segment ID signifies the document's body."]
    pub segment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replaces all instances of text matching a criteria with replace text."]
pub struct ReplaceAllTextRequest {
    #[serde(rename = "containsText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Finds text in the document matching this substring."]
    pub contains_text: ::std::option::Option<::std::boxed::Box<SubstringMatchCriteria>>,
    #[serde(rename = "replaceText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text that will replace the matched text."]
    pub replace_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of replacing text."]
pub struct ReplaceAllTextResponse {
    #[serde(rename = "occurrencesChanged")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of occurrences changed by replacing all text."]
    pub occurrences_changed: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replaces an existing image with a new image. Replacing an image removes some image effects from the existing image in order to mirror the behavior of the Docs editor."]
pub struct ReplaceImageRequest {
    #[serde(rename = "imageObjectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the existing image that will be replaced."]
    pub image_object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageReplaceMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The replacement method."]
    pub image_replace_method: ::std::option::Option<ReplaceImageRequestImageReplaceMethodEnum>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the new image. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI can be at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.source_uri field."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The replacement method."]
pub enum ReplaceImageRequestImageReplaceMethodEnum {
    #[serde(rename = "IMAGE_REPLACE_METHOD_UNSPECIFIED")]
    #[doc = "Unspecified image replace method. This value must not be used."]
    ImageReplaceMethodUnspecified,
    #[serde(rename = "CENTER_CROP")]
    #[doc = "Scales and centers the image to fill the bounds of the original image. The image may be cropped in order to fill the original image's bounds. The rendered size of the image will be the same as that of the original image."]
    CenterCrop,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replaces the contents of the specified NamedRange or NamedRanges with the given replacement content. Note that an individual NamedRange may consist of multiple discontinuous ranges. In this case, only the content in the first range will be replaced. The other ranges and their content will be deleted. In cases where replacing or deleting any ranges would result in an invalid document structure, a 400 bad request error is returned."]
pub struct ReplaceNamedRangeContentRequest {
    #[serde(rename = "namedRangeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the named range whose content will be replaced. If there is no named range with the given ID a 400 bad request error is returned."]
    pub named_range_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namedRangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the NamedRanges whose content will be replaced. If there are multiple named ranges with the given name, then the content of each one will be replaced. If there are no named ranges with the given name, then the request will be a no-op."]
    pub named_range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replaces the content of the specified named range(s) with the given text."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single update to apply to a document."]
pub struct Request {
    #[serde(rename = "createFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creates a footer."]
    pub create_footer: ::std::option::Option<::std::boxed::Box<CreateFooterRequest>>,
    #[serde(rename = "createFootnote")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creates a footnote."]
    pub create_footnote: ::std::option::Option<::std::boxed::Box<CreateFootnoteRequest>>,
    #[serde(rename = "createHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creates a header."]
    pub create_header: ::std::option::Option<::std::boxed::Box<CreateHeaderRequest>>,
    #[serde(rename = "createNamedRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creates a named range."]
    pub create_named_range: ::std::option::Option<::std::boxed::Box<CreateNamedRangeRequest>>,
    #[serde(rename = "createParagraphBullets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creates bullets for paragraphs."]
    pub create_paragraph_bullets:
        ::std::option::Option<::std::boxed::Box<CreateParagraphBulletsRequest>>,
    #[serde(rename = "deleteContentRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes content from the document."]
    pub delete_content_range: ::std::option::Option<::std::boxed::Box<DeleteContentRangeRequest>>,
    #[serde(rename = "deleteFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a footer from the document."]
    pub delete_footer: ::std::option::Option<::std::boxed::Box<DeleteFooterRequest>>,
    #[serde(rename = "deleteHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a header from the document."]
    pub delete_header: ::std::option::Option<::std::boxed::Box<DeleteHeaderRequest>>,
    #[serde(rename = "deleteNamedRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a named range."]
    pub delete_named_range: ::std::option::Option<::std::boxed::Box<DeleteNamedRangeRequest>>,
    #[serde(rename = "deleteParagraphBullets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes bullets from paragraphs."]
    pub delete_paragraph_bullets:
        ::std::option::Option<::std::boxed::Box<DeleteParagraphBulletsRequest>>,
    #[serde(rename = "deletePositionedObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a positioned object from the document."]
    pub delete_positioned_object:
        ::std::option::Option<::std::boxed::Box<DeletePositionedObjectRequest>>,
    #[serde(rename = "deleteTableColumn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a column from a table."]
    pub delete_table_column: ::std::option::Option<::std::boxed::Box<DeleteTableColumnRequest>>,
    #[serde(rename = "deleteTableRow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deletes a row from a table."]
    pub delete_table_row: ::std::option::Option<::std::boxed::Box<DeleteTableRowRequest>>,
    #[serde(rename = "insertInlineImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts an inline image at the specified location."]
    pub insert_inline_image: ::std::option::Option<::std::boxed::Box<InsertInlineImageRequest>>,
    #[serde(rename = "insertPageBreak")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts a page break at the specified location."]
    pub insert_page_break: ::std::option::Option<::std::boxed::Box<InsertPageBreakRequest>>,
    #[serde(rename = "insertSectionBreak")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts a section break at the specified location."]
    pub insert_section_break: ::std::option::Option<::std::boxed::Box<InsertSectionBreakRequest>>,
    #[serde(rename = "insertTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts a table at the specified location."]
    pub insert_table: ::std::option::Option<::std::boxed::Box<InsertTableRequest>>,
    #[serde(rename = "insertTableColumn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts an empty column into a table."]
    pub insert_table_column: ::std::option::Option<::std::boxed::Box<InsertTableColumnRequest>>,
    #[serde(rename = "insertTableRow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts an empty row into a table."]
    pub insert_table_row: ::std::option::Option<::std::boxed::Box<InsertTableRowRequest>>,
    #[serde(rename = "insertText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inserts text at the specified location."]
    pub insert_text: ::std::option::Option<::std::boxed::Box<InsertTextRequest>>,
    #[serde(rename = "mergeTableCells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merges cells in a table."]
    pub merge_table_cells: ::std::option::Option<::std::boxed::Box<MergeTableCellsRequest>>,
    #[serde(rename = "replaceAllText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replaces all instances of the specified text."]
    pub replace_all_text: ::std::option::Option<::std::boxed::Box<ReplaceAllTextRequest>>,
    #[serde(rename = "replaceImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replaces an image in the document."]
    pub replace_image: ::std::option::Option<::std::boxed::Box<ReplaceImageRequest>>,
    #[serde(rename = "replaceNamedRangeContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replaces the content in a named range."]
    pub replace_named_range_content:
        ::std::option::Option<::std::boxed::Box<ReplaceNamedRangeContentRequest>>,
    #[serde(rename = "unmergeTableCells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unmerges cells in a table."]
    pub unmerge_table_cells: ::std::option::Option<::std::boxed::Box<UnmergeTableCellsRequest>>,
    #[serde(rename = "updateDocumentStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the style of the document."]
    pub update_document_style: ::std::option::Option<::std::boxed::Box<UpdateDocumentStyleRequest>>,
    #[serde(rename = "updateParagraphStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the paragraph style at the specified range."]
    pub update_paragraph_style:
        ::std::option::Option<::std::boxed::Box<UpdateParagraphStyleRequest>>,
    #[serde(rename = "updateSectionStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the section style of the specified range."]
    pub update_section_style: ::std::option::Option<::std::boxed::Box<UpdateSectionStyleRequest>>,
    #[serde(rename = "updateTableCellStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the style of table cells."]
    pub update_table_cell_style:
        ::std::option::Option<::std::boxed::Box<UpdateTableCellStyleRequest>>,
    #[serde(rename = "updateTableColumnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the properties of columns in a table."]
    pub update_table_column_properties:
        ::std::option::Option<::std::boxed::Box<UpdateTableColumnPropertiesRequest>>,
    #[serde(rename = "updateTableRowStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the row style in a table."]
    pub update_table_row_style:
        ::std::option::Option<::std::boxed::Box<UpdateTableRowStyleRequest>>,
    #[serde(rename = "updateTextStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updates the text style at the specified range."]
    pub update_text_style: ::std::option::Option<::std::boxed::Box<UpdateTextStyleRequest>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single response from an update."]
pub struct Response {
    #[serde(rename = "createFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of creating a footer."]
    pub create_footer: ::std::option::Option<::std::boxed::Box<CreateFooterResponse>>,
    #[serde(rename = "createFootnote")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of creating a footnote."]
    pub create_footnote: ::std::option::Option<::std::boxed::Box<CreateFootnoteResponse>>,
    #[serde(rename = "createHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of creating a header."]
    pub create_header: ::std::option::Option<::std::boxed::Box<CreateHeaderResponse>>,
    #[serde(rename = "createNamedRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of creating a named range."]
    pub create_named_range: ::std::option::Option<::std::boxed::Box<CreateNamedRangeResponse>>,
    #[serde(rename = "insertInlineImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of inserting an inline image."]
    pub insert_inline_image: ::std::option::Option<::std::boxed::Box<InsertInlineImageResponse>>,
    #[serde(rename = "insertInlineSheetsChart")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of inserting an inline Google Sheets chart."]
    pub insert_inline_sheets_chart:
        ::std::option::Option<::std::boxed::Box<InsertInlineSheetsChartResponse>>,
    #[serde(rename = "replaceAllText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of replacing text."]
    pub replace_all_text: ::std::option::Option<::std::boxed::Box<ReplaceAllTextResponse>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An RGB color."]
pub struct RgbColor {
    #[serde(rename = "blue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The blue component of the color, from 0.0 to 1.0."]
    pub blue: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "green")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The green component of the color, from 0.0 to 1.0."]
    pub green: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "red")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The red component of the color, from 0.0 to 1.0."]
    pub red: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A StructuralElement representing a section break. A section is a range of content which has the same SectionStyle. A section break represents the start of a new section, and the section style applies to the section after the section break. The document body always begins with a section break."]
pub struct SectionBreak {
    #[serde(rename = "sectionStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of the section after this section break."]
    pub section_style: ::std::option::Option<::std::boxed::Box<SectionStyle>>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A SectionBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Properties that apply to a section's column."]
pub struct SectionColumnProperties {
    #[serde(rename = "paddingEnd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The padding at the end of the column."]
    pub padding_end: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The width of the column."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The styling that applies to a section."]
pub struct SectionStyle {
    #[serde(rename = "columnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The section's columns properties. If empty, the section contains one column with the default properties in the Docs editor. A section can be updated to have no more than three columns. When updating this property, setting a concrete value is required. Unsetting this property will result in a 400 bad request error."]
    pub column_properties:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SectionColumnProperties>>>,
    #[serde(rename = "columnSeparatorStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of column separators. This style can be set even when there is one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub column_separator_style: ::std::option::Option<SectionStyleColumnSeparatorStyleEnum>,
    #[serde(rename = "contentDirection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub content_direction: ::std::option::Option<SectionStyleContentDirectionEnum>,
    #[serde(rename = "defaultFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the default footer. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_footer_id. This property is read-only."]
    pub default_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the default header. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_header_id. This property is read-only."]
    pub default_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evenPageFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footer used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the footers on even pages in the section. If it is false, the footers on even pages uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_footer_id. This property is read-only."]
    pub even_page_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evenPageHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the headers on even pages in the section. If it is false, the headers on even pages uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_header_id. This property is read-only."]
    pub even_page_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstPageFooterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the footer used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the footer on the first page of the section. If it is false, the footer on the first page of the section uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_footer_id. This property is read-only."]
    pub first_page_footer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstPageHeaderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the header used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the header on the first page of the section. If it is false, the header on the first page of the section uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_header_id. This property is read-only."]
    pub first_page_header_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marginBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bottom page margin of the section. If unset, uses margin_bottom from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_bottom: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The footer margin of the section. If unset, uses margin_footer from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a footer margin is being respected for this section When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_footer: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The header margin of the section. If unset, uses margin_header from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a header margin is being respected for this section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_header: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The left page margin of the section. If unset, uses margin_left from DocumentStyle. Updating left margin causes columns in this section to resize. Since the margin affects column width, it is applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_left: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The right page margin of the section. If unset, uses margin_right from DocumentStyle. Updating right margin causes columns in this section to resize. Since the margin affects column width, it is applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_right: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "marginTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top page margin of the section. If unset, uses margin_top from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub margin_top: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "pageNumberStart")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page number from which to start counting the number of pages for this section. If unset, page numbering continues from the previous section. If the value is unset in the first SectionBreak, refer to DocumentStyle's page_number_start. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub page_number_start: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sectionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of section."]
    pub section_type: ::std::option::Option<SectionStyleSectionTypeEnum>,
    #[serde(rename = "useFirstPageHeaderFooter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether to use the first page header / footer IDs for the first page of the section. If unset, it inherits from DocumentStyle's use_first_page_header_footer for the first section. If the value is unset for subsequent sectors, it should be interpreted as false. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
    pub use_first_page_header_footer: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The style of column separators. This style can be set even when there is one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
pub enum SectionStyleColumnSeparatorStyleEnum {
    #[serde(rename = "COLUMN_SEPARATOR_STYLE_UNSPECIFIED")]
    #[doc = "An unspecified column separator style."]
    ColumnSeparatorStyleUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "No column separator lines between columns."]
    None,
    #[serde(rename = "BETWEEN_EACH_COLUMN")]
    #[doc = "Renders a column separator line between each column."]
    BetweenEachColumn,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
pub enum SectionStyleContentDirectionEnum {
    #[serde(rename = "CONTENT_DIRECTION_UNSPECIFIED")]
    #[doc = "The content direction is unspecified."]
    ContentDirectionUnspecified,
    #[serde(rename = "LEFT_TO_RIGHT")]
    #[doc = "The content goes from left to right."]
    LeftToRight,
    #[serde(rename = "RIGHT_TO_LEFT")]
    #[doc = "The content goes from right to left."]
    RightToLeft,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of section."]
pub enum SectionStyleSectionTypeEnum {
    #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
    #[doc = "The section type is unspecified."]
    SectionTypeUnspecified,
    #[serde(rename = "CONTINUOUS")]
    #[doc = "The section starts immediately after the last paragraph of the previous section."]
    Continuous,
    #[serde(rename = "NEXT_PAGE")]
    #[doc = "The section starts on the next page."]
    NextPage,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The shading of a paragraph."]
pub struct Shading {
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background color of this paragraph shading."]
    pub background_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base Shading have been changed in this suggested change. For any field set to true, there is a new suggested value."]
pub struct ShadingSuggestionState {
    #[serde(rename = "backgroundColorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to the Shading."]
    pub background_color_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a linked chart embedded from Google Sheets."]
pub struct SheetsChartReference {
    #[serde(rename = "chartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the specific chart in the Google Sheets spreadsheet that is embedded."]
    pub chart_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "spreadsheetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Google Sheets spreadsheet that contains the source chart."]
    pub spreadsheet_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base SheetsChartReference have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct SheetsChartReferenceSuggestionState {
    #[serde(rename = "chartIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to chart_id."]
    pub chart_id_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "spreadsheetIdSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to spreadsheet_id."]
    pub spreadsheet_id_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A width and height."]
pub struct Size {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the object."]
    pub height: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the object."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base Size have been changed in this suggestion. For any field set to true, the Size has a new suggested value."]
pub struct SizeSuggestionState {
    #[serde(rename = "heightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to height."]
    pub height_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "widthSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to width."]
    pub width_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A StructuralElement describes content that provides structure to the document."]
pub struct StructuralElement {
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based end index of this structural element, exclusive, in UTF-16 code units."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "paragraph")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A paragraph type of structural element."]
    pub paragraph: ::std::option::Option<::std::boxed::Box<Paragraph>>,
    #[serde(rename = "sectionBreak")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A section break type of structural element."]
    pub section_break: ::std::option::Option<::std::boxed::Box<SectionBreak>>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based start index of this structural element, in UTF-16 code units."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A table type of structural element."]
    pub table: ::std::option::Option<::std::boxed::Box<Table>>,
    #[serde(rename = "tableOfContents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A table of contents type of structural element."]
    pub table_of_contents: ::std::option::Option<::std::boxed::Box<TableOfContents>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A criteria that matches a specific string of text in the document."]
pub struct SubstringMatchCriteria {
    #[serde(rename = "matchCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive."]
    pub match_case: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to search for in the document."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to a Bullet."]
pub struct SuggestedBullet {
    #[serde(rename = "bullet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Bullet that only includes the changes made in this suggestion. This can be used along with the bullet_suggestion_state to see which fields have changed and their new values."]
    pub bullet: ::std::option::Option<::std::boxed::Box<Bullet>>,
    #[serde(rename = "bulletSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base Bullet have been changed in this suggestion."]
    pub bullet_suggestion_state: ::std::option::Option<::std::boxed::Box<BulletSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to the DocumentStyle."]
pub struct SuggestedDocumentStyle {
    #[serde(rename = "documentStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A DocumentStyle that only includes the changes made in this suggestion. This can be used along with the document_style_suggestion_state to see which fields have changed and their new values."]
    pub document_style: ::std::option::Option<::std::boxed::Box<DocumentStyle>>,
    #[serde(rename = "documentStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion."]
    pub document_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<DocumentStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to InlineObjectProperties."]
pub struct SuggestedInlineObjectProperties {
    #[serde(rename = "inlineObjectProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An InlineObjectProperties that only includes the changes made in this suggestion. This can be used along with the inline_object_properties_suggestion_state to see which fields have changed and their new values."]
    pub inline_object_properties: ::std::option::Option<::std::boxed::Box<InlineObjectProperties>>,
    #[serde(rename = "inlineObjectPropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base InlineObjectProperties have been changed in this suggestion."]
    pub inline_object_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<InlineObjectPropertiesSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to ListProperties."]
pub struct SuggestedListProperties {
    #[serde(rename = "listProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A ListProperties that only includes the changes made in this suggestion. This can be used along with the list_properties_suggestion_state to see which fields have changed and their new values."]
    pub list_properties: ::std::option::Option<::std::boxed::Box<ListProperties>>,
    #[serde(rename = "listPropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion."]
    pub list_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<ListPropertiesSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to the NamedStyles."]
pub struct SuggestedNamedStyles {
    #[serde(rename = "namedStyles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A NamedStyles that only includes the changes made in this suggestion. This can be used along with the named_styles_suggestion_state to see which fields have changed and their new values."]
    pub named_styles: ::std::option::Option<::std::boxed::Box<NamedStyles>>,
    #[serde(rename = "namedStylesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base NamedStyles have been changed in this suggestion."]
    pub named_styles_suggestion_state:
        ::std::option::Option<::std::boxed::Box<NamedStylesSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to a ParagraphStyle."]
pub struct SuggestedParagraphStyle {
    #[serde(rename = "paragraphStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A ParagraphStyle that only includes the changes made in this suggestion. This can be used along with the paragraph_suggestion_state to see which fields have changed and their new values."]
    pub paragraph_style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
    #[serde(rename = "paragraphStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion."]
    pub paragraph_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<ParagraphStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to PositionedObjectProperties."]
pub struct SuggestedPositionedObjectProperties {
    #[serde(rename = "positionedObjectProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A PositionedObjectProperties that only includes the changes made in this suggestion. This can be used along with the positioned_object_properties_suggestion_state to see which fields have changed and their new values."]
    pub positioned_object_properties:
        ::std::option::Option<::std::boxed::Box<PositionedObjectProperties>>,
    #[serde(rename = "positionedObjectPropertiesSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base PositionedObjectProperties have been changed in this suggestion."]
    pub positioned_object_properties_suggestion_state:
        ::std::option::Option<::std::boxed::Box<PositionedObjectPropertiesSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to a TableCellStyle."]
pub struct SuggestedTableCellStyle {
    #[serde(rename = "tableCellStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A TableCellStyle that only includes the changes made in this suggestion. This can be used along with the table_cell_style_suggestion_state to see which fields have changed and their new values."]
    pub table_cell_style: ::std::option::Option<::std::boxed::Box<TableCellStyle>>,
    #[serde(rename = "tableCellStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion."]
    pub table_cell_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TableCellStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to a TableRowStyle."]
pub struct SuggestedTableRowStyle {
    #[serde(rename = "tableRowStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A TableRowStyle that only includes the changes made in this suggestion. This can be used along with the table_row_style_suggestion_state to see which fields have changed and their new values."]
    pub table_row_style: ::std::option::Option<::std::boxed::Box<TableRowStyle>>,
    #[serde(rename = "tableRowStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion."]
    pub table_row_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TableRowStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A suggested change to a TextStyle."]
pub struct SuggestedTextStyle {
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A TextStyle that only includes the changes made in this suggestion. This can be used along with the text_style_suggestion_state to see which fields have changed and their new values."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
    #[serde(rename = "textStyleSuggestionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion."]
    pub text_style_suggestion_state:
        ::std::option::Option<::std::boxed::Box<TextStyleSuggestionState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tab stop within a paragraph."]
pub struct TabStop {
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alignment of this tab stop. If unset, the value defaults to START."]
    pub alignment: ::std::option::Option<TabStopAlignmentEnum>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset between this tab stop and the start margin."]
    pub offset: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alignment of this tab stop. If unset, the value defaults to START."]
pub enum TabStopAlignmentEnum {
    #[serde(rename = "TAB_STOP_ALIGNMENT_UNSPECIFIED")]
    #[doc = "The tab stop alignment is unspecified."]
    TabStopAlignmentUnspecified,
    #[serde(rename = "START")]
    #[doc = "The tab stop is aligned to the start of the line. This is the default."]
    Start,
    #[serde(rename = "CENTER")]
    #[doc = "The tab stop is aligned to the center of the line."]
    Center,
    #[serde(rename = "END")]
    #[doc = "The tab stop is aligned to the end of the line."]
    End,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A StructuralElement representing a table."]
pub struct Table {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of columns in the table. It is possible for a table to be non-rectangular, so some rows may have a different number of cells."]
    pub columns: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of rows in the table."]
    pub rows: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A Table may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tableRows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents and style of each row."]
    pub table_rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableRow>>>,
    #[serde(rename = "tableStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of the table."]
    pub table_style: ::std::option::Option<::std::boxed::Box<TableStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The contents and style of a cell in a Table."]
pub struct TableCell {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the cell."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based end index of this cell, exclusive, in UTF-16 code units."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based start index of this cell, in UTF-16 code units."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A TableCell may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTableCellStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested changes to the table cell style, keyed by suggestion ID."]
    pub suggested_table_cell_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTableCellStyle>>,
    >,
    #[serde(rename = "tableCellStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of the cell."]
    pub table_cell_style: ::std::option::Option<::std::boxed::Box<TableCellStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A border around a table cell. Table cell borders cannot be transparent. To hide a table cell border, make its width 0."]
pub struct TableCellBorder {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the border. This color cannot be transparent."]
    pub color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "dashStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dash style of the border."]
    pub dash_style: ::std::option::Option<TableCellBorderDashStyleEnum>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the border."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The dash style of the border."]
pub enum TableCellBorderDashStyleEnum {
    #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
    #[doc = "Unspecified dash style."]
    DashStyleUnspecified,
    #[serde(rename = "SOLID")]
    #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
    Solid,
    #[serde(rename = "DOT")]
    #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
    Dot,
    #[serde(rename = "DASH")]
    #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
    Dash,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of a single cell within a table."]
pub struct TableCellLocation {
    #[serde(rename = "columnIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based column index. For example, the second column in the table has a column index of 1."]
    pub column_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rowIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based row index. For example, the second row in the table has a row index of 1."]
    pub row_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "tableStartLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where the table starts in the document."]
    pub table_start_location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The style of a TableCell. Inherited table cell styles are represented as unset fields in this message. A table cell style can inherit from the table's style."]
pub struct TableCellStyle {
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background color of the cell."]
    pub background_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "borderBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bottom border of the cell."]
    pub border_bottom: ::std::option::Option<::std::boxed::Box<TableCellBorder>>,
    #[serde(rename = "borderLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The left border of the cell."]
    pub border_left: ::std::option::Option<::std::boxed::Box<TableCellBorder>>,
    #[serde(rename = "borderRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The right border of the cell."]
    pub border_right: ::std::option::Option<::std::boxed::Box<TableCellBorder>>,
    #[serde(rename = "borderTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top border of the cell."]
    pub border_top: ::std::option::Option<::std::boxed::Box<TableCellBorder>>,
    #[serde(rename = "columnSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The column span of the cell. This property is read-only."]
    pub column_span: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "contentAlignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor."]
    pub content_alignment: ::std::option::Option<TableCellStyleContentAlignmentEnum>,
    #[serde(rename = "paddingBottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bottom padding of the cell."]
    pub padding_bottom: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "paddingLeft")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The left padding of the cell."]
    pub padding_left: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "paddingRight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The right padding of the cell."]
    pub padding_right: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "paddingTop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top padding of the cell."]
    pub padding_top: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "rowSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The row span of the cell. This property is read-only."]
    pub row_span: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor."]
pub enum TableCellStyleContentAlignmentEnum {
    #[serde(rename = "CONTENT_ALIGNMENT_UNSPECIFIED")]
    #[doc = "An unspecified content alignment. The content alignment is inherited from the parent if one exists."]
    ContentAlignmentUnspecified,
    #[serde(rename = "CONTENT_ALIGNMENT_UNSUPPORTED")]
    #[doc = "An unsupported content alignment."]
    ContentAlignmentUnsupported,
    #[serde(rename = "TOP")]
    #[doc = "An alignment that aligns the content to the top of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 't'."]
    Top,
    #[serde(rename = "MIDDLE")]
    #[doc = "An alignment that aligns the content to the middle of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
    Middle,
    #[serde(rename = "BOTTOM")]
    #[doc = "An alignment that aligns the content to the bottom of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
    Bottom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct TableCellStyleSuggestionState {
    #[serde(rename = "backgroundColorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to background_color."]
    pub background_color_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_bottom."]
    pub border_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_left."]
    pub border_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_right."]
    pub border_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "borderTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to border_top."]
    pub border_top_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "columnSpanSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to column_span."]
    pub column_span_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "contentAlignmentSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to content_alignment."]
    pub content_alignment_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paddingBottomSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to padding_bottom."]
    pub padding_bottom_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paddingLeftSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to padding_left."]
    pub padding_left_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paddingRightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to padding_right."]
    pub padding_right_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paddingTopSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to padding_top."]
    pub padding_top_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rowSpanSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to row_span."]
    pub row_span_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of a column in a table."]
pub struct TableColumnProperties {
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the column. Set when the column's `width_type` is FIXED_WIDTH."]
    pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "widthType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width type of the column."]
    pub width_type: ::std::option::Option<TableColumnPropertiesWidthTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The width type of the column."]
pub enum TableColumnPropertiesWidthTypeEnum {
    #[serde(rename = "WIDTH_TYPE_UNSPECIFIED")]
    #[doc = "The column width type is unspecified."]
    WidthTypeUnspecified,
    #[serde(rename = "EVENLY_DISTRIBUTED")]
    #[doc = "The column width is evenly distributed among the other evenly distrubted columns. The width of the column is automatically determined and will have an equal portion of the width remaining for the table after accounting for all columns with specified widths."]
    EvenlyDistributed,
    #[serde(rename = "FIXED_WIDTH")]
    #[doc = "A fixed column width. The width property contains the column's width."]
    FixedWidth,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A StructuralElement representing a table of contents."]
pub struct TableOfContents {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the table of contents."]
    pub content: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuralElement>>>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A TableOfContents may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with table cell location = (table_start_location, row = 0, column = 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]"]
pub struct TableRange {
    #[serde(rename = "columnSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The column span of the table range."]
    pub column_span: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rowSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The row span of the table range."]
    pub row_span: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "tableCellLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cell location where the table range starts."]
    pub table_cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The contents and style of a row in a Table."]
pub struct TableRow {
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based end index of this row, exclusive, in UTF-16 code units."]
    pub end_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based start index of this row, in UTF-16 code units."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A TableRow may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTableRowStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested style changes to this row, keyed by suggestion ID."]
    pub suggested_table_row_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTableRowStyle>>,
    >,
    #[serde(rename = "tableCells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents and style of each cell in this row. It is possible for a table to be non-rectangular, so some rows may have a different number of cells than other rows in the same table."]
    pub table_cells: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableCell>>>,
    #[serde(rename = "tableRowStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of the table row."]
    pub table_row_style: ::std::option::Option<::std::boxed::Box<TableRowStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Styles that apply to a table row."]
pub struct TableRowStyle {
    #[serde(rename = "minRowHeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum height of the row. The row will be rendered in the Docs editor at a height equal to or greater than this value in order to show all the content in the row's cells."]
    pub min_row_height: ::std::option::Option<::std::boxed::Box<Dimension>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct TableRowStyleSuggestionState {
    #[serde(rename = "minRowHeightSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to min_row_height."]
    pub min_row_height_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Styles that apply to a table."]
pub struct TableStyle {
    #[serde(rename = "tableColumnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of each column. Note that in Docs, tables contain rows and rows contain cells, similar to HTML. So the properties for a row can be found on the row's table_row_style."]
    pub table_column_properties:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableColumnProperties>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ParagraphElement that represents a run of text that all has the same styling."]
pub struct TextRun {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text of this run. Any non-text elements in the run are replaced with the Unicode character U+E907."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suggestedDeletionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
    pub suggested_deletion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedInsertionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested insertion IDs. A TextRun may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
    pub suggested_insertion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "suggestedTextStyleChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested text style changes to this run, keyed by suggestion ID."]
    pub suggested_text_style_changes: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<SuggestedTextStyle>>,
    >,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text style of this run."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the styling that can be applied to text. Inherited text styles are represented as unset fields in this message. A text style's parent depends on where the text style is defined: * The TextStyle of text in a Paragraph inherits from the paragraph's corresponding named style type. * The TextStyle on a named style inherits from the normal text named style. * The TextStyle of the normal text named style inherits from the default text style in the Docs editor. * The TextStyle on a Paragraph element that is contained in a table may inherit its text style from the table style. If the text style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor."]
pub struct TextStyle {
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field."]
    pub background_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "baselineOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field."]
    pub baseline_offset: ::std::option::Option<TextStyleBaselineOffsetEnum>,
    #[serde(rename = "bold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the text is rendered as bold."]
    pub bold: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the text's font."]
    pub font_size: ::std::option::Option<::std::boxed::Box<Dimension>>,
    #[serde(rename = "foregroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The foreground color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field."]
    pub foreground_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
    #[serde(rename = "italic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the text is italicized."]
    pub italic: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hyperlink destination of the text. If unset, there is no link. Links are not inherited from parent text. Changing the link in an update request causes some other changes to the text style of the range: * When setting a link, the text foreground color will be updated to the default link color and the text will be underlined. If these fields are modified in the same request, those values will be used instead of the link defaults. * Setting a link on a text range that overlaps with an existing link will also update the existing link to point to the new URL. * Links are not settable on newline characters. As a result, setting a link on a text range that crosses a paragraph boundary, such as `\"ABC\\n123\"`, will separate the newline character(s) into their own text runs. The link will be applied separately to the runs before and after the newline. * Removing a link will update the text style of the range to match the style of the preceding text (or the default text styles if the preceding text is another link) unless different styles are being set in the same request."]
    pub link: ::std::option::Option<::std::boxed::Box<Link>>,
    #[serde(rename = "smallCaps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the text is in small capital letters."]
    pub small_caps: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "strikethrough")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the text is struck through."]
    pub strikethrough: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "underline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the text is underlined."]
    pub underline: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "weightedFontFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The font family and rendered weight of the text. If an update request specifies values for both `weighted_font_family` and `bold`, the `weighted_font_family` is applied first, then `bold`. If `weighted_font_family#weight` is not set, it defaults to `400`. If `weighted_font_family` is set, then `weighted_font_family#font_family` must also be set with a non-empty value. Otherwise, a 400 bad request error is returned."]
    pub weighted_font_family: ::std::option::Option<::std::boxed::Box<WeightedFontFamily>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field."]
pub enum TextStyleBaselineOffsetEnum {
    #[serde(rename = "BASELINE_OFFSET_UNSPECIFIED")]
    #[doc = "The text's baseline offset is inherited from the parent."]
    BaselineOffsetUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "The text is not vertically offset."]
    None,
    #[serde(rename = "SUPERSCRIPT")]
    #[doc = "The text is vertically offset upwards (superscript)."]
    Superscript,
    #[serde(rename = "SUBSCRIPT")]
    #[doc = "The text is vertically offset downwards (subscript)."]
    Subscript,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion. For any field set to true, there is a new suggested value."]
pub struct TextStyleSuggestionState {
    #[serde(rename = "backgroundColorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to background_color."]
    pub background_color_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "baselineOffsetSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to baseline_offset."]
    pub baseline_offset_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "boldSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to bold."]
    pub bold_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fontSizeSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to font_size."]
    pub font_size_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "foregroundColorSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to foreground_color."]
    pub foreground_color_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "italicSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to italic."]
    pub italic_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "linkSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to link."]
    pub link_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "smallCapsSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to small_caps."]
    pub small_caps_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "strikethroughSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to strikethrough."]
    pub strikethrough_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "underlineSuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to underline."]
    pub underline_suggested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "weightedFontFamilySuggested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if there was a suggested change to weighted_font_family."]
    pub weighted_font_family_suggested: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Unmerges cells in a Table."]
pub struct UnmergeTableCellsRequest {
    #[serde(rename = "tableRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The table range specifying which cells of the table to unmerge. All merged cells in this range will be unmerged, and cells that are already unmerged will not be affected. If the range has no merged cells, the request will do nothing. If there is text in any of the merged cells, the text will remain in the \"head\" cell of the resulting block of unmerged cells. The \"head\" cell is the upper-left cell when the content direction is from left to right, and the upper-right otherwise."]
    pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Updates the DocumentStyle."]
pub struct UpdateDocumentStyleRequest {
    #[serde(rename = "documentStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The styles to set on the document. Certain document style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of DocumentStyle for more information."]
    pub document_style: ::std::option::Option<::std::boxed::Box<DocumentStyle>>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `document_style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the background, set `fields` to `\"background\"`."]
    pub fields: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Update the styling of all paragraphs that overlap with the given range."]
pub struct UpdateParagraphStyleRequest {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `paragraph_style` is implied and should not be specified. For example, to update the paragraph style's alignment property, set `fields` to `\"alignment\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paragraphStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The styles to set on the paragraphs. Certain paragraph style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of ParagraphStyle for more information."]
    pub paragraph_style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range overlapping the paragraphs to style."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Updates the SectionStyle."]
pub struct UpdateSectionStyleRequest {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `section_style` is implied and must not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the left margin, set `fields` to `\"margin_left\"`."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range overlapping the sections to style. Because section breaks can only be inserted inside the body, the segment ID field must be empty."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
    #[serde(rename = "sectionStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The styles to be set on the section. Certain section style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of SectionStyle for more information."]
    pub section_style: ::std::option::Option<::std::boxed::Box<SectionStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Updates the style of a range of table cells."]
pub struct UpdateTableCellStyleRequest {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `tableCellStyle` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the table cell background color, set `fields` to `\"backgroundColor\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableCellStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style to set on the table cells. When updating borders, if a cell shares a border with an adjacent cell, the corresponding border property of the adjacent cell is updated as well. Borders that are merged and invisible are not updated. Since updating a border shared by adjacent cells in the same request can cause conflicting border updates, border updates are applied in the following order: - `border_right` - `border_left` - `border_bottom` - `border_top`"]
    pub table_cell_style: ::std::option::Option<::std::boxed::Box<TableCellStyle>>,
    #[serde(rename = "tableRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The table range representing the subset of the table to which the updates are applied."]
    pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
    #[serde(rename = "tableStartLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where the table starts in the document. When specified, the updates are applied to all the cells in the table."]
    pub table_start_location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Updates the TableColumnProperties of columns in a table."]
pub struct UpdateTableColumnPropertiesRequest {
    #[serde(rename = "columnIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of zero-based column indices whose property should be updated. If no indices are specified, all columns will be updated."]
    pub column_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `\"width\"`."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableColumnProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The table column properties to update. If the value of `table_column_properties#width` is less than 5 points (5/72 inch), a 400 bad request error is returned."]
    pub table_column_properties: ::std::option::Option<::std::boxed::Box<TableColumnProperties>>,
    #[serde(rename = "tableStartLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where the table starts in the document."]
    pub table_start_location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Updates the TableRowStyle of rows in a table."]
pub struct UpdateTableRowStyleRequest {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `tableRowStyle` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `\"min_row_height\"`."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rowIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of zero-based row indices whose style should be updated. If no indices are specified, all rows will be updated."]
    pub row_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "tableRowStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The styles to be set on the rows."]
    pub table_row_style: ::std::option::Option<::std::boxed::Box<TableRowStyle>>,
    #[serde(rename = "tableStartLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where the table starts in the document."]
    pub table_start_location: ::std::option::Option<::std::boxed::Box<Location>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Update the styling of text."]
pub struct UpdateTextStyleRequest {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields that should be updated. At least one field must be specified. The root `text_style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `\"bold\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range of text to style. The range may be extended to include adjacent newlines. If the range fully contains a paragraph belonging to a list, the paragraph's bullet is also updated with the matching text style. Ranges cannot be inserted inside a relative UpdateTextStyleRequest."]
    pub range: ::std::option::Option<::std::boxed::Box<Range>>,
    #[serde(rename = "textStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The styles to set on the text. If the value for a particular style matches that of the parent, that style will be set to inherit. Certain text style changes may cause other changes in order to to mirror the behavior of the Docs editor. See the documentation of TextStyle for more information."]
    pub text_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a font family and weight of text."]
pub struct WeightedFontFamily {
    #[serde(rename = "fontFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The font family of the text. The font family can be any font from the Font menu in Docs or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`."]
    pub font_family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The weight of the font. This field can have any value that is a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. The default value is `400` (\"normal\"). The font weight makes up just one component of the rendered font weight. The rendered weight is determined by a combination of the `weight` and the text style's resolved `bold` value, after accounting for inheritance: * If the text is bold and the weight is less than `400`, the rendered weight is 400. * If the text is bold and the weight is greater than or equal to `400` but is less than `700`, the rendered weight is `700`. * If the weight is greater than or equal to `700`, the rendered weight is equal to the weight. * If the text is not bold, the rendered weight is equal to the weight."]
    pub weight: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides control over how write requests are executed."]
pub struct WriteControl {
    #[serde(rename = "requiredRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision ID of the document that the write request will be applied to. If this is not the latest revision of the document, the request will not be processed and will return a 400 bad request error. When a required revision ID is returned in a response, it indicates the revision ID of the document after the request was applied."]
    pub required_revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target revision ID of the document that the write request will be applied to. If collaborator changes have occurred after the document was read using the API, the changes produced by this write request will be transformed against the collaborator changes. This results in a new revision of the document which incorporates both the changes in the request and the collaborator changes, and the Docs server will resolve conflicting changes. When using `target_revision_id`, the API client can be thought of as another collaborator of the document. The target revision ID may only be used to write to recent versions of a document. If the target revision is too far behind the latest revision, the request will not be processed and will return a 400 bad request error and the request should be retried after reading the latest version of the document. In most cases a `revision_id` will remain valid for use as a target revision for several minutes after it is read, but for frequently-edited documents this window may be shorter."]
    pub target_revision_id: ::std::option::Option<::std::string::String>,
}