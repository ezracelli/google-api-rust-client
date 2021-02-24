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
        serde_json::from_str(&"\"json\"").unwrap()
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
    pub mod voices {
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
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Recommended. [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. If specified, the ListVoices call will only return voices that can be used to synthesize this language_code. E.g. when specifying \"en-NZ\", you will get supported \"en-\\*\" voices; when specifying \"no\", you will get supported \"no-\\*\" (Norwegian) and \"nb-\\*\" (Norwegian Bokmal) voices; specifying \"zh\" will also get supported \"cmn-\\*\" voices; specifying \"zh-hk\" will also get supported \"yue-\\*\" voices."]
                    pub language_code: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of audio data to be synthesized."]
    pub struct AudioConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the audio byte stream."]
        pub audio_encoding: ::std::option::Option<AudioConfigAudioEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectsProfileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Input only. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given. See [audio profiles](https://cloud.google.com/text-to-speech/docs/audio-profiles) for current supported profile ids."]
        pub effects_profile_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pitch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Input only. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
        pub pitch: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The synthesis sample rate (in hertz) for this audio. When this is specified in SynthesizeSpeechRequest, if this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality), unless the specified sample rate is not supported for the encoding chosen, in which case it will fail the request and return google.rpc.Code.INVALID_ARGUMENT."]
        pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakingRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Input only. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error."]
        pub speaking_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeGainDb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Input only. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. Strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that."]
        pub volume_gain_db: ::std::option::Option<::std::primitive::f64>,
    }
    impl AudioConfig {
        pub fn builder() -> AudioConfigBuilder {
            AudioConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The format of the audio byte stream."]
    pub enum AudioConfigAudioEncodingEnum {
        #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
        #[doc = "Not specified. Will return result google.rpc.Code.INVALID_ARGUMENT."]
        AudioEncodingUnspecified,
        #[serde(rename = "LINEAR16")]
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header."]
        Linear16,
        #[serde(rename = "MP3")]
        #[doc = "MP3 audio at 32kbps."]
        Mp3,
        #[serde(rename = "MP3_64_KBPS")]
        #[doc = "MP3 at 64kbps."]
        Mp364Kbps,
        #[serde(rename = "OGG_OPUS")]
        #[doc = "Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate."]
        OggOpus,
        #[serde(rename = "MULAW")]
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law. Audio content returned as MULAW also contains a WAV header."]
        Mulaw,
    }
    impl ::std::default::Default for AudioConfigAudioEncodingEnum {
        fn default() -> Self {
            Self::AudioEncodingUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The message returned to the client by the `ListVoices` method."]
    pub struct ListVoicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of voices."]
        pub voices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Voice>>>,
    }
    impl ListVoicesResponse {
        pub fn builder() -> ListVoicesResponseBuilder {
            ListVoicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains text input to be synthesized. Either `text` or `ssml` must be supplied. Supplying both or neither returns google.rpc.Code.INVALID_ARGUMENT. The input size is limited to 5000 characters."]
    pub struct SynthesisInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSML document to be synthesized. The SSML document must be valid and well-formed. Otherwise the RPC will fail and return google.rpc.Code.INVALID_ARGUMENT. For more information, see [SSML](https://cloud.google.com/text-to-speech/docs/ssml)."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw text to be synthesized."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl SynthesisInput {
        pub fn builder() -> SynthesisInputBuilder {
            SynthesisInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The top-level message sent by the client for the `SynthesizeSpeech` method."]
    pub struct SynthesizeSpeechRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The configuration of the synthesized audio."]
        pub audio_config: ::std::option::Option<::std::boxed::Box<AudioConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableTimePointing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether and what timepoints are returned in the response."]
        pub enable_time_pointing:
            ::std::option::Option<::std::vec::Vec<SynthesizeSpeechRequestEnableTimePointingEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Synthesizer requires either plain text or SSML as input."]
        pub input: ::std::option::Option<::std::boxed::Box<SynthesisInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired voice of the synthesized audio."]
        pub voice: ::std::option::Option<::std::boxed::Box<VoiceSelectionParams>>,
    }
    impl SynthesizeSpeechRequest {
        pub fn builder() -> SynthesizeSpeechRequestBuilder {
            SynthesizeSpeechRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum SynthesizeSpeechRequestEnableTimePointingEnum {
        #[serde(rename = "TIMEPOINT_TYPE_UNSPECIFIED")]
        #[doc = "Not specified. No timepoint information will be returned."]
        TimepointTypeUnspecified,
        #[serde(rename = "SSML_MARK")]
        #[doc = "Timepoint information of tags in SSML input will be returned."]
        SsmlMark,
    }
    impl ::std::default::Default for SynthesizeSpeechRequestEnableTimePointingEnum {
        fn default() -> Self {
            Self::TimepointTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The message returned to the client by the `SynthesizeSpeech` method."]
    pub struct SynthesizeSpeechResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio metadata of `audio_content`."]
        pub audio_config: ::std::option::Option<::std::boxed::Box<AudioConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio data bytes encoded as specified in the request, including the header for encodings that are wrapped in containers (e.g. MP3, OGG_OPUS). For LINEAR16 audio, we include the WAV header. Note: as with all bytes fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        pub audio_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timepoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link between a position in the original request input and a corresponding time in the output audio. It's only supported via of SSML input."]
        pub timepoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Timepoint>>>,
    }
    impl SynthesizeSpeechResponse {
        pub fn builder() -> SynthesizeSpeechResponseBuilder {
            SynthesizeSpeechResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This contains a mapping between a certain point in the input text and a corresponding time in the output audio."]
    pub struct Timepoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "markName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timepoint name as received from the client within tag."]
        pub mark_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset in seconds from the start of the synthesized audio."]
        pub time_seconds: ::std::option::Option<::std::primitive::f64>,
    }
    impl Timepoint {
        pub fn builder() -> TimepointBuilder {
            TimepointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of a voice supported by the TTS service."]
    pub struct Voice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The languages that this voice supports, expressed as [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tags (e.g. \"en-US\", \"es-419\", \"cmn-tw\")."]
        pub language_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this voice. Each distinct voice has a unique name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "naturalSampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural sample rate (in hertz) for this voice."]
        pub natural_sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssmlGender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gender of this voice."]
        pub ssml_gender: ::std::option::Option<VoiceSsmlGenderEnum>,
    }
    impl Voice {
        pub fn builder() -> VoiceBuilder {
            VoiceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The gender of this voice."]
    pub enum VoiceSsmlGenderEnum {
        #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
        #[doc = "An unspecified gender. In VoiceSelectionParams, this means that the client doesn't care which gender the selected voice will have. In the Voice field of ListVoicesResponse, this may mean that the voice doesn't fit any of the other categories in this enum, or that the gender of the voice isn't known."]
        SsmlVoiceGenderUnspecified,
        #[serde(rename = "MALE")]
        #[doc = "A male voice."]
        Male,
        #[serde(rename = "FEMALE")]
        #[doc = "A female voice."]
        Female,
        #[serde(rename = "NEUTRAL")]
        #[doc = "A gender-neutral voice. This voice is not yet supported."]
        Neutral,
    }
    impl ::std::default::Default for VoiceSsmlGenderEnum {
        fn default() -> Self {
            Self::SsmlVoiceGenderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of which voice to use for a synthesis request."]
    pub struct VoiceSelectionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language (and potentially also the region) of the voice expressed as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag, e.g. \"en-US\". This should not include a script tag (e.g. use \"cmn-cn\" rather than \"cmn-Hant-cn\"), because the script will be inferred from the input provided in the SynthesisInput. The TTS service will use this parameter to help choose an appropriate voice. Note that the TTS service may choose a voice with a slightly different language code than the one selected; it may substitute a different region (e.g. using en-US rather than en-CA if there isn't a Canadian voice available), or even a different language, e.g. using \"nb\" (Norwegian Bokmal) instead of \"no\" (Norwegian)\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and gender."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssmlGender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement; if a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
        pub ssml_gender: ::std::option::Option<VoiceSelectionParamsSsmlGenderEnum>,
    }
    impl VoiceSelectionParams {
        pub fn builder() -> VoiceSelectionParamsBuilder {
            VoiceSelectionParamsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement; if a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
    pub enum VoiceSelectionParamsSsmlGenderEnum {
        #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
        #[doc = "An unspecified gender. In VoiceSelectionParams, this means that the client doesn't care which gender the selected voice will have. In the Voice field of ListVoicesResponse, this may mean that the voice doesn't fit any of the other categories in this enum, or that the gender of the voice isn't known."]
        SsmlVoiceGenderUnspecified,
        #[serde(rename = "MALE")]
        #[doc = "A male voice."]
        Male,
        #[serde(rename = "FEMALE")]
        #[doc = "A female voice."]
        Female,
        #[serde(rename = "NEUTRAL")]
        #[doc = "A gender-neutral voice. This voice is not yet supported."]
        Neutral,
    }
    impl ::std::default::Default for VoiceSelectionParamsSsmlGenderEnum {
        fn default() -> Self {
            Self::SsmlVoiceGenderUnspecified
        }
    }
}
