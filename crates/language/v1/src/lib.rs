#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The entity analysis request message."]
pub struct AnalyzeEntitiesRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "encodingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding type used by the API to calculate offsets."]
    pub encoding_type: ::std::option::Option<AnalyzeEntitiesRequestEncodingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding type used by the API to calculate offsets."]
pub enum AnalyzeEntitiesRequestEncodingTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`."]
    None,
    #[serde(rename = "UTF8")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively."]
    Utf8,
    #[serde(rename = "UTF16")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and JavaScript are examples of languages that use this encoding natively."]
    Utf16,
    #[serde(rename = "UTF32")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively."]
    Utf32,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The entity analysis response message."]
pub struct AnalyzeEntitiesResponse {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recognized entities in the input document."]
    pub entities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entity>>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details."]
    pub language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The entity-level sentiment analysis request message."]
pub struct AnalyzeEntitySentimentRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "encodingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding type used by the API to calculate offsets."]
    pub encoding_type: ::std::option::Option<AnalyzeEntitySentimentRequestEncodingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding type used by the API to calculate offsets."]
pub enum AnalyzeEntitySentimentRequestEncodingTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`."]
    None,
    #[serde(rename = "UTF8")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively."]
    Utf8,
    #[serde(rename = "UTF16")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and JavaScript are examples of languages that use this encoding natively."]
    Utf16,
    #[serde(rename = "UTF32")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively."]
    Utf32,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The entity-level sentiment analysis response message."]
pub struct AnalyzeEntitySentimentResponse {
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recognized entities in the input document with associated sentiments."]
    pub entities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entity>>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details."]
    pub language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The sentiment analysis request message."]
pub struct AnalyzeSentimentRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "encodingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding type used by the API to calculate sentence offsets."]
    pub encoding_type: ::std::option::Option<AnalyzeSentimentRequestEncodingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding type used by the API to calculate sentence offsets."]
pub enum AnalyzeSentimentRequestEncodingTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`."]
    None,
    #[serde(rename = "UTF8")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively."]
    Utf8,
    #[serde(rename = "UTF16")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and JavaScript are examples of languages that use this encoding natively."]
    Utf16,
    #[serde(rename = "UTF32")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively."]
    Utf32,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The sentiment analysis response message."]
pub struct AnalyzeSentimentResponse {
    #[serde(rename = "documentSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overall sentiment of the input document."]
    pub document_sentiment: ::std::option::Option<::std::boxed::Box<Sentiment>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sentences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentiment for all the sentences in the document."]
    pub sentences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Sentence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The syntax analysis request message."]
pub struct AnalyzeSyntaxRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "encodingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding type used by the API to calculate offsets."]
    pub encoding_type: ::std::option::Option<AnalyzeSyntaxRequestEncodingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding type used by the API to calculate offsets."]
pub enum AnalyzeSyntaxRequestEncodingTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`."]
    None,
    #[serde(rename = "UTF8")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively."]
    Utf8,
    #[serde(rename = "UTF16")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and JavaScript are examples of languages that use this encoding natively."]
    Utf16,
    #[serde(rename = "UTF32")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively."]
    Utf32,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The syntax analysis response message."]
pub struct AnalyzeSyntaxResponse {
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sentences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentences in the input document."]
    pub sentences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Sentence>>>,
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tokens, along with their syntactic information, in the input document."]
    pub tokens: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Token>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for the text annotation API, which can perform multiple analysis types (sentiment, entities, and syntax) in one call."]
pub struct AnnotateTextRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "encodingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding type used by the API to calculate offsets."]
    pub encoding_type: ::std::option::Option<AnnotateTextRequestEncodingTypeEnum>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The enabled features."]
    pub features: ::std::option::Option<::std::boxed::Box<Features>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding type used by the API to calculate offsets."]
pub enum AnnotateTextRequestEncodingTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as `begin_offset`) will be set at `-1`."]
    None,
    #[serde(rename = "UTF8")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-8 encoding of the input. C++ and Go are examples of languages that use this encoding natively."]
    Utf8,
    #[serde(rename = "UTF16")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-16 encoding of the input. Java and JavaScript are examples of languages that use this encoding natively."]
    Utf16,
    #[serde(rename = "UTF32")]
    #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based on the UTF-32 encoding of the input. Python is an example of a language that uses this encoding natively."]
    Utf32,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The text annotations response message."]
pub struct AnnotateTextResponse {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categories identified in the input document."]
    pub categories:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClassificationCategory>>>,
    #[serde(rename = "documentSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overall sentiment for the document. Populated if the user enables AnnotateTextRequest.Features.extract_document_sentiment."]
    pub document_sentiment: ::std::option::Option<::std::boxed::Box<Sentiment>>,
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities, along with their semantic information, in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_entities."]
    pub entities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entity>>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sentences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentences in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_syntax."]
    pub sentences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Sentence>>>,
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tokens, along with their syntactic information, in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_syntax."]
    pub tokens: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Token>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a category returned from the text classifier."]
pub struct ClassificationCategory {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The classifier's confidence of the category. Number represents how certain the classifier is that this category represents the given text."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the category representing the document, from the [predefined taxonomy](https://cloud.google.com/natural-language/docs/categories)."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The document classification request message."]
pub struct ClassifyTextRequest {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input document."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The document classification response message."]
pub struct ClassifyTextResponse {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categories representing the input document."]
    pub categories:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClassificationCategory>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents dependency parse tree information for a token. (For more information on dependency labels, see http://www.aclweb.org/anthology/P13-2017"]
pub struct DependencyEdge {
    #[serde(rename = "headTokenIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents the head of this token in the dependency tree. This is the index of the token which has an arc going to this token. The index is the position of the token in the array of tokens returned by the API method. If this token is a root token, then the `head_token_index` is its own index."]
    pub head_token_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parse label for the token."]
    pub label: ::std::option::Option<DependencyEdgeLabelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The parse label for the token."]
pub enum DependencyEdgeLabelEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown"]
    Unknown,
    #[serde(rename = "ABBREV")]
    #[doc = "Abbreviation modifier"]
    Abbrev,
    #[serde(rename = "ACOMP")]
    #[doc = "Adjectival complement"]
    Acomp,
    #[serde(rename = "ADVCL")]
    #[doc = "Adverbial clause modifier"]
    Advcl,
    #[serde(rename = "ADVMOD")]
    #[doc = "Adverbial modifier"]
    Advmod,
    #[serde(rename = "AMOD")]
    #[doc = "Adjectival modifier of an NP"]
    Amod,
    #[serde(rename = "APPOS")]
    #[doc = "Appositional modifier of an NP"]
    Appos,
    #[serde(rename = "ATTR")]
    #[doc = "Attribute dependent of a copular verb"]
    Attr,
    #[serde(rename = "AUX")]
    #[doc = "Auxiliary (non-main) verb"]
    Aux,
    #[serde(rename = "AUXPASS")]
    #[doc = "Passive auxiliary"]
    Auxpass,
    #[serde(rename = "CC")]
    #[doc = "Coordinating conjunction"]
    Cc,
    #[serde(rename = "CCOMP")]
    #[doc = "Clausal complement of a verb or adjective"]
    Ccomp,
    #[serde(rename = "CONJ")]
    #[doc = "Conjunct"]
    Conj,
    #[serde(rename = "CSUBJ")]
    #[doc = "Clausal subject"]
    Csubj,
    #[serde(rename = "CSUBJPASS")]
    #[doc = "Clausal passive subject"]
    Csubjpass,
    #[serde(rename = "DEP")]
    #[doc = "Dependency (unable to determine)"]
    Dep,
    #[serde(rename = "DET")]
    #[doc = "Determiner"]
    Det,
    #[serde(rename = "DISCOURSE")]
    #[doc = "Discourse"]
    Discourse,
    #[serde(rename = "DOBJ")]
    #[doc = "Direct object"]
    Dobj,
    #[serde(rename = "EXPL")]
    #[doc = "Expletive"]
    Expl,
    #[serde(rename = "GOESWITH")]
    #[doc = "Goes with (part of a word in a text not well edited)"]
    Goeswith,
    #[serde(rename = "IOBJ")]
    #[doc = "Indirect object"]
    Iobj,
    #[serde(rename = "MARK")]
    #[doc = "Marker (word introducing a subordinate clause)"]
    Mark,
    #[serde(rename = "MWE")]
    #[doc = "Multi-word expression"]
    Mwe,
    #[serde(rename = "MWV")]
    #[doc = "Multi-word verbal expression"]
    Mwv,
    #[serde(rename = "NEG")]
    #[doc = "Negation modifier"]
    Neg,
    #[serde(rename = "NN")]
    #[doc = "Noun compound modifier"]
    Nn,
    #[serde(rename = "NPADVMOD")]
    #[doc = "Noun phrase used as an adverbial modifier"]
    Npadvmod,
    #[serde(rename = "NSUBJ")]
    #[doc = "Nominal subject"]
    Nsubj,
    #[serde(rename = "NSUBJPASS")]
    #[doc = "Passive nominal subject"]
    Nsubjpass,
    #[serde(rename = "NUM")]
    #[doc = "Numeric modifier of a noun"]
    Num,
    #[serde(rename = "NUMBER")]
    #[doc = "Element of compound number"]
    Number,
    #[serde(rename = "P")]
    #[doc = "Punctuation mark"]
    P,
    #[serde(rename = "PARATAXIS")]
    #[doc = "Parataxis relation"]
    Parataxis,
    #[serde(rename = "PARTMOD")]
    #[doc = "Participial modifier"]
    Partmod,
    #[serde(rename = "PCOMP")]
    #[doc = "The complement of a preposition is a clause"]
    Pcomp,
    #[serde(rename = "POBJ")]
    #[doc = "Object of a preposition"]
    Pobj,
    #[serde(rename = "POSS")]
    #[doc = "Possession modifier"]
    Poss,
    #[serde(rename = "POSTNEG")]
    #[doc = "Postverbal negative particle"]
    Postneg,
    #[serde(rename = "PRECOMP")]
    #[doc = "Predicate complement"]
    Precomp,
    #[serde(rename = "PRECONJ")]
    #[doc = "Preconjunt"]
    Preconj,
    #[serde(rename = "PREDET")]
    #[doc = "Predeterminer"]
    Predet,
    #[serde(rename = "PREF")]
    #[doc = "Prefix"]
    Pref,
    #[serde(rename = "PREP")]
    #[doc = "Prepositional modifier"]
    Prep,
    #[serde(rename = "PRONL")]
    #[doc = "The relationship between a verb and verbal morpheme"]
    Pronl,
    #[serde(rename = "PRT")]
    #[doc = "Particle"]
    Prt,
    #[serde(rename = "PS")]
    #[doc = "Associative or possessive marker"]
    Ps,
    #[serde(rename = "QUANTMOD")]
    #[doc = "Quantifier phrase modifier"]
    Quantmod,
    #[serde(rename = "RCMOD")]
    #[doc = "Relative clause modifier"]
    Rcmod,
    #[serde(rename = "RCMODREL")]
    #[doc = "Complementizer in relative clause"]
    Rcmodrel,
    #[serde(rename = "RDROP")]
    #[doc = "Ellipsis without a preceding predicate"]
    Rdrop,
    #[serde(rename = "REF")]
    #[doc = "Referent"]
    Ref,
    #[serde(rename = "REMNANT")]
    #[doc = "Remnant"]
    Remnant,
    #[serde(rename = "REPARANDUM")]
    #[doc = "Reparandum"]
    Reparandum,
    #[serde(rename = "ROOT")]
    #[doc = "Root"]
    Root,
    #[serde(rename = "SNUM")]
    #[doc = "Suffix specifying a unit of number"]
    Snum,
    #[serde(rename = "SUFF")]
    #[doc = "Suffix"]
    Suff,
    #[serde(rename = "TMOD")]
    #[doc = "Temporal modifier"]
    Tmod,
    #[serde(rename = "TOPIC")]
    #[doc = "Topic marker"]
    Topic,
    #[serde(rename = "VMOD")]
    #[doc = "Clause headed by an infinite form of the verb that modifies a noun"]
    Vmod,
    #[serde(rename = "VOCATIVE")]
    #[doc = "Vocative"]
    Vocative,
    #[serde(rename = "XCOMP")]
    #[doc = "Open clausal complement"]
    Xcomp,
    #[serde(rename = "SUFFIX")]
    #[doc = "Name suffix"]
    Suffix,
    #[serde(rename = "TITLE")]
    #[doc = "Name title"]
    Title,
    #[serde(rename = "ADVPHMOD")]
    #[doc = "Adverbial phrase modifier"]
    Advphmod,
    #[serde(rename = "AUXCAUS")]
    #[doc = "Causative auxiliary"]
    Auxcaus,
    #[serde(rename = "AUXVV")]
    #[doc = "Helper auxiliary"]
    Auxvv,
    #[serde(rename = "DTMOD")]
    #[doc = "Rentaishi (Prenominal modifier)"]
    Dtmod,
    #[serde(rename = "FOREIGN")]
    #[doc = "Foreign words"]
    Foreign,
    #[serde(rename = "KW")]
    #[doc = "Keyword"]
    Kw,
    #[serde(rename = "LIST")]
    #[doc = "List for chains of comparable items"]
    List,
    #[serde(rename = "NOMC")]
    #[doc = "Nominalized clause"]
    Nomc,
    #[serde(rename = "NOMCSUBJ")]
    #[doc = "Nominalized clausal subject"]
    Nomcsubj,
    #[serde(rename = "NOMCSUBJPASS")]
    #[doc = "Nominalized clausal passive"]
    Nomcsubjpass,
    #[serde(rename = "NUMC")]
    #[doc = "Compound of numeric modifier"]
    Numc,
    #[serde(rename = "COP")]
    #[doc = "Copula"]
    Cop,
    #[serde(rename = "DISLOCATED")]
    #[doc = "Dislocated relation (for fronted/topicalized elements)"]
    Dislocated,
    #[serde(rename = "ASP")]
    #[doc = "Aspect marker"]
    Asp,
    #[serde(rename = "GMOD")]
    #[doc = "Genitive modifier"]
    Gmod,
    #[serde(rename = "GOBJ")]
    #[doc = "Genitive object"]
    Gobj,
    #[serde(rename = "INFMOD")]
    #[doc = "Infinitival modifier"]
    Infmod,
    #[serde(rename = "MES")]
    #[doc = "Measure"]
    Mes,
    #[serde(rename = "NCOMP")]
    #[doc = "Nominal complement of a noun"]
    Ncomp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "################################################################ # Represents the input to API methods."]
pub struct Document {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the input in string format. Cloud audit logging exempt since it is based on user data."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsContentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Storage URI where the file content is located. This URI must be of the form: gs://bucket_name/object_name. For more details, see https://cloud.google.com/storage/docs/reference-uris. NOTE: Cloud Storage object versioning is not supported."]
    pub gcs_content_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the document (if not specified, the language is automatically detected). Both ISO and BCP-47 language codes are accepted. [Language Support](https://cloud.google.com/natural-language/docs/languages) lists currently supported languages for each API method. If the language (either specified by the caller or automatically detected) is not supported by the called API method, an `INVALID_ARGUMENT` error is returned."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. If the type is not set or is `TYPE_UNSPECIFIED`, returns an `INVALID_ARGUMENT` error."]
    pub _type: ::std::option::Option<DocumentTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. If the type is not set or is `TYPE_UNSPECIFIED`, returns an `INVALID_ARGUMENT` error."]
pub enum DocumentTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The content type is not specified."]
    TypeUnspecified,
    #[serde(rename = "PLAIN_TEXT")]
    #[doc = "Plain text"]
    PlainText,
    #[serde(rename = "HTML")]
    #[doc = "HTML"]
    Html,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a phrase in the text that is a known entity, such as a person, an organization, or location. The API associates information, such as salience and mentions, with entities."]
pub struct Entity {
    #[serde(rename = "mentions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mentions of this entity in the input document. The API currently supports proper noun mentions."]
    pub mentions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityMention>>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with the entity. For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`), if they are available. For the metadata associated with other entity types, see the Type table below."]
    pub metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The representative name for the entity."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "salience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The salience score associated with the entity in the [0, 1.0] range. The salience score for an entity provides information about the importance or centrality of that entity to the entire document text. Scores closer to 0 are less salient, while scores closer to 1.0 are highly salient."]
    pub salience: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "sentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For calls to AnalyzeEntitySentiment or if AnnotateTextRequest.Features.extract_entity_sentiment is set to true, this field will contain the aggregate sentiment expressed for this entity in the provided document."]
    pub sentiment: ::std::option::Option<::std::boxed::Box<Sentiment>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity type."]
    pub _type: ::std::option::Option<EntityTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The entity type."]
pub enum EntityTypeEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown"]
    Unknown,
    #[serde(rename = "PERSON")]
    #[doc = "Person"]
    Person,
    #[serde(rename = "LOCATION")]
    #[doc = "Location"]
    Location,
    #[serde(rename = "ORGANIZATION")]
    #[doc = "Organization"]
    Organization,
    #[serde(rename = "EVENT")]
    #[doc = "Event"]
    Event,
    #[serde(rename = "WORK_OF_ART")]
    #[doc = "Artwork"]
    WorkOfArt,
    #[serde(rename = "CONSUMER_GOOD")]
    #[doc = "Consumer product"]
    ConsumerGood,
    #[serde(rename = "OTHER")]
    #[doc = "Other types of entities"]
    Other,
    #[serde(rename = "PHONE_NUMBER")]
    #[doc = "Phone number The metadata lists the phone number, formatted according to local convention, plus whichever additional elements appear in the text: * `number` - the actual number, broken down into sections as per local convention * `national_prefix` - country code, if detected * `area_code` - region or area code, if detected * `extension` - phone extension (to be dialed after connection), if detected"]
    PhoneNumber,
    #[serde(rename = "ADDRESS")]
    #[doc = "Address The metadata identifies the street number and locality plus whichever additional elements appear in the text: * `street_number` - street number * `locality` - city or town * `street_name` - street/route name, if detected * `postal_code` - postal code, if detected * `country` - country, if detected< * `broad_region` - administrative area, such as the state, if detected * `narrow_region` - smaller administrative area, such as county, if detected * `sublocality` - used in Asian addresses to demark a district within a city, if detected"]
    Address,
    #[serde(rename = "DATE")]
    #[doc = "Date The metadata identifies the components of the date: * `year` - four digit year, if detected * `month` - two digit month number, if detected * `day` - two digit day number, if detected"]
    Date,
    #[serde(rename = "NUMBER")]
    #[doc = "Number The metadata is the number itself."]
    Number,
    #[serde(rename = "PRICE")]
    #[doc = "Price The metadata identifies the `value` and `currency`."]
    Price,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a mention for an entity in the text. Currently, proper noun mentions are supported."]
pub struct EntityMention {
    #[serde(rename = "sentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For calls to AnalyzeEntitySentiment or if AnnotateTextRequest.Features.extract_entity_sentiment is set to true, this field will contain the sentiment expressed for this mention of the entity in the provided document."]
    pub sentiment: ::std::option::Option<::std::boxed::Box<Sentiment>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mention text."]
    pub text: ::std::option::Option<::std::boxed::Box<TextSpan>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the entity mention."]
    pub _type: ::std::option::Option<EntityMentionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the entity mention."]
pub enum EntityMentionTypeEnum {
    #[serde(rename = "TYPE_UNKNOWN")]
    #[doc = "Unknown"]
    TypeUnknown,
    #[serde(rename = "PROPER")]
    #[doc = "Proper name"]
    Proper,
    #[serde(rename = "COMMON")]
    #[doc = "Common noun (or noun compound)"]
    Common,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All available features for sentiment, syntax, and semantic analysis. Setting each one to true will enable that specific analysis for the input."]
pub struct Features {
    #[serde(rename = "classifyText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Classify the full document into categories."]
    pub classify_text: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "extractDocumentSentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extract document-level sentiment."]
    pub extract_document_sentiment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "extractEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extract entities."]
    pub extract_entities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "extractEntitySentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extract entities and their associated sentiment."]
    pub extract_entity_sentiment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "extractSyntax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extract syntax information."]
    pub extract_syntax: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents part of speech information for a token. Parts of speech are as defined in http://www.lrec-conf.org/proceedings/lrec2012/pdf/274_Paper.pdf"]
pub struct PartOfSpeech {
    #[serde(rename = "aspect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical aspect."]
    pub aspect: ::std::option::Option<PartOfSpeechAspectEnum>,
    #[serde(rename = "case")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical case."]
    pub case: ::std::option::Option<PartOfSpeechCaseEnum>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical form."]
    pub form: ::std::option::Option<PartOfSpeechFormEnum>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical gender."]
    pub gender: ::std::option::Option<PartOfSpeechGenderEnum>,
    #[serde(rename = "mood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical mood."]
    pub mood: ::std::option::Option<PartOfSpeechMoodEnum>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical number."]
    pub number: ::std::option::Option<PartOfSpeechNumberEnum>,
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical person."]
    pub person: ::std::option::Option<PartOfSpeechPersonEnum>,
    #[serde(rename = "proper")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical properness."]
    pub proper: ::std::option::Option<PartOfSpeechProperEnum>,
    #[serde(rename = "reciprocity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical reciprocity."]
    pub reciprocity: ::std::option::Option<PartOfSpeechReciprocityEnum>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The part of speech tag."]
    pub tag: ::std::option::Option<PartOfSpeechTagEnum>,
    #[serde(rename = "tense")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical tense."]
    pub tense: ::std::option::Option<PartOfSpeechTenseEnum>,
    #[serde(rename = "voice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grammatical voice."]
    pub voice: ::std::option::Option<PartOfSpeechVoiceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical aspect."]
pub enum PartOfSpeechAspectEnum {
    #[serde(rename = "ASPECT_UNKNOWN")]
    #[doc = "Aspect is not applicable in the analyzed language or is not predicted."]
    AspectUnknown,
    #[serde(rename = "PERFECTIVE")]
    #[doc = "Perfective"]
    Perfective,
    #[serde(rename = "IMPERFECTIVE")]
    #[doc = "Imperfective"]
    Imperfective,
    #[serde(rename = "PROGRESSIVE")]
    #[doc = "Progressive"]
    Progressive,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical case."]
pub enum PartOfSpeechCaseEnum {
    #[serde(rename = "CASE_UNKNOWN")]
    #[doc = "Case is not applicable in the analyzed language or is not predicted."]
    CaseUnknown,
    #[serde(rename = "ACCUSATIVE")]
    #[doc = "Accusative"]
    Accusative,
    #[serde(rename = "ADVERBIAL")]
    #[doc = "Adverbial"]
    Adverbial,
    #[serde(rename = "COMPLEMENTIVE")]
    #[doc = "Complementive"]
    Complementive,
    #[serde(rename = "DATIVE")]
    #[doc = "Dative"]
    Dative,
    #[serde(rename = "GENITIVE")]
    #[doc = "Genitive"]
    Genitive,
    #[serde(rename = "INSTRUMENTAL")]
    #[doc = "Instrumental"]
    Instrumental,
    #[serde(rename = "LOCATIVE")]
    #[doc = "Locative"]
    Locative,
    #[serde(rename = "NOMINATIVE")]
    #[doc = "Nominative"]
    Nominative,
    #[serde(rename = "OBLIQUE")]
    #[doc = "Oblique"]
    Oblique,
    #[serde(rename = "PARTITIVE")]
    #[doc = "Partitive"]
    Partitive,
    #[serde(rename = "PREPOSITIONAL")]
    #[doc = "Prepositional"]
    Prepositional,
    #[serde(rename = "REFLEXIVE_CASE")]
    #[doc = "Reflexive"]
    ReflexiveCase,
    #[serde(rename = "RELATIVE_CASE")]
    #[doc = "Relative"]
    RelativeCase,
    #[serde(rename = "VOCATIVE")]
    #[doc = "Vocative"]
    Vocative,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical form."]
pub enum PartOfSpeechFormEnum {
    #[serde(rename = "FORM_UNKNOWN")]
    #[doc = "Form is not applicable in the analyzed language or is not predicted."]
    FormUnknown,
    #[serde(rename = "ADNOMIAL")]
    #[doc = "Adnomial"]
    Adnomial,
    #[serde(rename = "AUXILIARY")]
    #[doc = "Auxiliary"]
    Auxiliary,
    #[serde(rename = "COMPLEMENTIZER")]
    #[doc = "Complementizer"]
    Complementizer,
    #[serde(rename = "FINAL_ENDING")]
    #[doc = "Final ending"]
    FinalEnding,
    #[serde(rename = "GERUND")]
    #[doc = "Gerund"]
    Gerund,
    #[serde(rename = "REALIS")]
    #[doc = "Realis"]
    Realis,
    #[serde(rename = "IRREALIS")]
    #[doc = "Irrealis"]
    Irrealis,
    #[serde(rename = "SHORT")]
    #[doc = "Short form"]
    Short,
    #[serde(rename = "LONG")]
    #[doc = "Long form"]
    Long,
    #[serde(rename = "ORDER")]
    #[doc = "Order form"]
    Order,
    #[serde(rename = "SPECIFIC")]
    #[doc = "Specific form"]
    Specific,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical gender."]
pub enum PartOfSpeechGenderEnum {
    #[serde(rename = "GENDER_UNKNOWN")]
    #[doc = "Gender is not applicable in the analyzed language or is not predicted."]
    GenderUnknown,
    #[serde(rename = "FEMININE")]
    #[doc = "Feminine"]
    Feminine,
    #[serde(rename = "MASCULINE")]
    #[doc = "Masculine"]
    Masculine,
    #[serde(rename = "NEUTER")]
    #[doc = "Neuter"]
    Neuter,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical mood."]
pub enum PartOfSpeechMoodEnum {
    #[serde(rename = "MOOD_UNKNOWN")]
    #[doc = "Mood is not applicable in the analyzed language or is not predicted."]
    MoodUnknown,
    #[serde(rename = "CONDITIONAL_MOOD")]
    #[doc = "Conditional"]
    ConditionalMood,
    #[serde(rename = "IMPERATIVE")]
    #[doc = "Imperative"]
    Imperative,
    #[serde(rename = "INDICATIVE")]
    #[doc = "Indicative"]
    Indicative,
    #[serde(rename = "INTERROGATIVE")]
    #[doc = "Interrogative"]
    Interrogative,
    #[serde(rename = "JUSSIVE")]
    #[doc = "Jussive"]
    Jussive,
    #[serde(rename = "SUBJUNCTIVE")]
    #[doc = "Subjunctive"]
    Subjunctive,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical number."]
pub enum PartOfSpeechNumberEnum {
    #[serde(rename = "NUMBER_UNKNOWN")]
    #[doc = "Number is not applicable in the analyzed language or is not predicted."]
    NumberUnknown,
    #[serde(rename = "SINGULAR")]
    #[doc = "Singular"]
    Singular,
    #[serde(rename = "PLURAL")]
    #[doc = "Plural"]
    Plural,
    #[serde(rename = "DUAL")]
    #[doc = "Dual"]
    Dual,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical person."]
pub enum PartOfSpeechPersonEnum {
    #[serde(rename = "PERSON_UNKNOWN")]
    #[doc = "Person is not applicable in the analyzed language or is not predicted."]
    PersonUnknown,
    #[serde(rename = "FIRST")]
    #[doc = "First"]
    First,
    #[serde(rename = "SECOND")]
    #[doc = "Second"]
    Second,
    #[serde(rename = "THIRD")]
    #[doc = "Third"]
    Third,
    #[serde(rename = "REFLEXIVE_PERSON")]
    #[doc = "Reflexive"]
    ReflexivePerson,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical properness."]
pub enum PartOfSpeechProperEnum {
    #[serde(rename = "PROPER_UNKNOWN")]
    #[doc = "Proper is not applicable in the analyzed language or is not predicted."]
    ProperUnknown,
    #[serde(rename = "PROPER")]
    #[doc = "Proper"]
    Proper,
    #[serde(rename = "NOT_PROPER")]
    #[doc = "Not proper"]
    NotProper,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical reciprocity."]
pub enum PartOfSpeechReciprocityEnum {
    #[serde(rename = "RECIPROCITY_UNKNOWN")]
    #[doc = "Reciprocity is not applicable in the analyzed language or is not predicted."]
    ReciprocityUnknown,
    #[serde(rename = "RECIPROCAL")]
    #[doc = "Reciprocal"]
    Reciprocal,
    #[serde(rename = "NON_RECIPROCAL")]
    #[doc = "Non-reciprocal"]
    NonReciprocal,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The part of speech tag."]
pub enum PartOfSpeechTagEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown"]
    Unknown,
    #[serde(rename = "ADJ")]
    #[doc = "Adjective"]
    Adj,
    #[serde(rename = "ADP")]
    #[doc = "Adposition (preposition and postposition)"]
    Adp,
    #[serde(rename = "ADV")]
    #[doc = "Adverb"]
    Adv,
    #[serde(rename = "CONJ")]
    #[doc = "Conjunction"]
    Conj,
    #[serde(rename = "DET")]
    #[doc = "Determiner"]
    Det,
    #[serde(rename = "NOUN")]
    #[doc = "Noun (common and proper)"]
    Noun,
    #[serde(rename = "NUM")]
    #[doc = "Cardinal number"]
    Num,
    #[serde(rename = "PRON")]
    #[doc = "Pronoun"]
    Pron,
    #[serde(rename = "PRT")]
    #[doc = "Particle or other function word"]
    Prt,
    #[serde(rename = "PUNCT")]
    #[doc = "Punctuation"]
    Punct,
    #[serde(rename = "VERB")]
    #[doc = "Verb (all tenses and modes)"]
    Verb,
    #[serde(rename = "X")]
    #[doc = "Other: foreign words, typos, abbreviations"]
    X,
    #[serde(rename = "AFFIX")]
    #[doc = "Affix"]
    Affix,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical tense."]
pub enum PartOfSpeechTenseEnum {
    #[serde(rename = "TENSE_UNKNOWN")]
    #[doc = "Tense is not applicable in the analyzed language or is not predicted."]
    TenseUnknown,
    #[serde(rename = "CONDITIONAL_TENSE")]
    #[doc = "Conditional"]
    ConditionalTense,
    #[serde(rename = "FUTURE")]
    #[doc = "Future"]
    Future,
    #[serde(rename = "PAST")]
    #[doc = "Past"]
    Past,
    #[serde(rename = "PRESENT")]
    #[doc = "Present"]
    Present,
    #[serde(rename = "IMPERFECT")]
    #[doc = "Imperfect"]
    Imperfect,
    #[serde(rename = "PLUPERFECT")]
    #[doc = "Pluperfect"]
    Pluperfect,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The grammatical voice."]
pub enum PartOfSpeechVoiceEnum {
    #[serde(rename = "VOICE_UNKNOWN")]
    #[doc = "Voice is not applicable in the analyzed language or is not predicted."]
    VoiceUnknown,
    #[serde(rename = "ACTIVE")]
    #[doc = "Active"]
    Active,
    #[serde(rename = "CAUSATIVE")]
    #[doc = "Causative"]
    Causative,
    #[serde(rename = "PASSIVE")]
    #[doc = "Passive"]
    Passive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a sentence in the input document."]
pub struct Sentence {
    #[serde(rename = "sentiment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For calls to AnalyzeSentiment or if AnnotateTextRequest.Features.extract_document_sentiment is set to true, this field will contain the sentiment for the sentence."]
    pub sentiment: ::std::option::Option<::std::boxed::Box<Sentiment>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sentence text."]
    pub text: ::std::option::Option<::std::boxed::Box<TextSpan>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the feeling associated with the entire text or entities in the text."]
pub struct Sentiment {
    #[serde(rename = "magnitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment regardless of score (positive or negative)."]
    pub magnitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
    pub score: ::std::option::Option<::std::primitive::f64>,
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
#[doc = "Represents an output piece of text."]
pub struct TextSpan {
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API calculates the beginning offset of the content in the original document according to the EncodingType specified in the API request."]
    pub begin_offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the output text."]
    pub content: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the smallest syntactic building block of the text."]
pub struct Token {
    #[serde(rename = "dependencyEdge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dependency tree parse for this token."]
    pub dependency_edge: ::std::option::Option<::std::boxed::Box<DependencyEdge>>,
    #[serde(rename = "lemma")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Lemma](https://en.wikipedia.org/wiki/Lemma_%28morphology%29) of the token."]
    pub lemma: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partOfSpeech")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parts of speech tag for this token."]
    pub part_of_speech: ::std::option::Option<::std::boxed::Box<PartOfSpeech>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token text."]
    pub text: ::std::option::Option<::std::boxed::Box<TextSpan>>,
}