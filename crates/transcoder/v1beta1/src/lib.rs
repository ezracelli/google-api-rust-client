#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ad break."]
pub struct AdBreak {
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time in seconds for the ad break, relative to the output file timeline. The default is `0s`."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for AES-128 encryption."]
pub struct Aes128Encryption {
    #[serde(rename = "keyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI of the key delivery service. This URI is inserted into the M3U8 header."]
    pub key_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Animation types."]
pub struct Animation {
    #[serde(rename = "animationEnd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End previous animation."]
    pub animation_end: ::std::option::Option<::std::boxed::Box<AnimationEnd>>,
    #[serde(rename = "animationFade")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display overlay object with fade animation."]
    pub animation_fade: ::std::option::Option<::std::boxed::Box<AnimationFade>>,
    #[serde(rename = "animationStatic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display static overlay object."]
    pub animation_static: ::std::option::Option<::std::boxed::Box<AnimationStatic>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "End previous overlay animation from the video. Without AnimationEnd, the overlay object will keep the state of previous animation until the end of the video."]
pub struct AnimationEnd {
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time to end overlay object, in seconds. Default: 0"]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Display overlay object with fade animation."]
pub struct AnimationFade {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time to end the fade animation, in seconds. Default: `start_time_offset` + 1s"]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fadeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Type of fade animation: `FADE_IN` or `FADE_OUT`."]
    pub fade_type: ::std::option::Option<AnimationFadeFadeTypeEnum>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time to start the fade animation, in seconds. Default: 0"]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "xy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video."]
    pub xy: ::std::option::Option<::std::boxed::Box<NormalizedCoordinate>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Type of fade animation: `FADE_IN` or `FADE_OUT`."]
pub enum AnimationFadeFadeTypeEnum {
    #[serde(rename = "FADE_TYPE_UNSPECIFIED")]
    #[doc = "The fade type is not specified."]
    FadeTypeUnspecified,
    #[serde(rename = "FADE_IN")]
    #[doc = "Fade the overlay object into view."]
    FadeIn,
    #[serde(rename = "FADE_OUT")]
    #[doc = "Fade the overlay object out of view."]
    FadeOut,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Display static overlay object."]
pub struct AnimationStatic {
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time to start displaying the overlay object, in seconds. Default: 0"]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "xy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video."]
    pub xy: ::std::option::Option<::std::boxed::Box<NormalizedCoordinate>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Audio preprocessing configuration."]
pub struct Audio {
    #[serde(rename = "highBoost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable boosting high frequency components. The default is `false`."]
    pub high_boost: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lowBoost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable boosting low frequency components. The default is `false`."]
    pub low_boost: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lufs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify audio loudness normalization in loudness units relative to full scale (LUFS). Enter a value between -24 and 0 (the default), where: * -24 is the Advanced Television Systems Committee (ATSC A/85) standard * -23 is the EU R128 broadcast standard * -19 is the prior standard for online mono audio * -18 is the ReplayGain standard * -16 is the prior standard for stereo audio * -14 is the new online audio standard recommended by Spotify, as well as Amazon Echo * 0 disables normalization"]
    pub lufs: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`."]
pub struct AudioAtom {
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Channel`s for this audio stream. for in-depth explanation."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AudioChannel>>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The `EditAtom.key` that references the atom with audio inputs in the `Job.edit_list`."]
    pub key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The audio channel."]
pub struct AudioChannel {
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Job.inputs` for this audio channel."]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AudioChannelInput>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies which input file, track, and channel should be used."]
pub struct AudioChannelInput {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The zero-based index of the channel in the input file."]
    pub channel: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "gainDb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audio volume control in dB. Negative values decrease volume, positive values increase. The default is 0."]
    pub gain_db: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The `Input.key` that identifies the input file."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "track")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The zero-based index of the track in the input file."]
    pub track: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Audio stream resource."]
pub struct AudioStream {
    #[serde(rename = "bitrateBps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Audio bitrate in bits per second. Must be between 1 and 10,000,000."]
    pub bitrate_bps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "channelCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of audio channels. Must be between 1 and 6. The default is 2."]
    pub channel_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "channelLayout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of channel names specifying layout of the audio channels. This only affects the metadata embedded in the container headers, if supported by the specified format. The default is `[\"fl\", \"fr\"]`. Supported channel names: - 'fl' - Front left channel - 'fr' - Front right channel - 'sl' - Side left channel - 'sr' - Side right channel - 'fc' - Front center channel - 'lfe' - Low frequency"]
    pub channel_layout: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The codec for this audio stream. The default is `\"aac\"`. Supported audio codecs: - 'aac' - 'aac-he' - 'aac-he-v2' - 'mp3' - 'ac3' - 'eac3'"]
    pub codec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mapping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`."]
    pub mapping: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AudioAtom>>>,
    #[serde(rename = "sampleRateHertz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The audio sample rate in Hertz. The default is 48000 Hertz."]
    pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Color preprocessing configuration."]
pub struct Color {
    #[serde(rename = "brightness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Control brightness of the video. Enter a value between -1 and 1, where -1 is minimum brightness and 1 is maximum brightness. 0 is no change. The default is 0."]
    pub brightness: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "contrast")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Control black and white contrast of the video. Enter a value between -1 and 1, where -1 is minimum contrast and 1 is maximum contrast. 0 is no change. The default is 0."]
    pub contrast: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "saturation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Control color saturation of the video. Enter a value between -1 and 1, where -1 is fully desaturated and 1 is maximum saturation. 0 is no change. The default is 0."]
    pub saturation: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video cropping configuration for the input video. The cropped input video is scaled to match the output resolution."]
pub struct Crop {
    #[serde(rename = "bottomPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to crop from the bottom. The default is 0."]
    pub bottom_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "leftPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to crop from the left. The default is 0."]
    pub left_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rightPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to crop from the right. The default is 0."]
    pub right_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "topPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to crop from the top. The default is 0."]
    pub top_pixels: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deblock preprocessing configuration."]
pub struct Deblock {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable deblocker. The default is `false`."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set strength of the deblocker. Enter a value between 0 and 1. The higher the value, the stronger the block removal. 0 is no deblocking. The default is 0."]
    pub strength: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Denoise preprocessing configuration."]
pub struct Denoise {
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set strength of the denoise. Enter a value between 0 and 1. The higher the value, the smoother the image. 0 is no denoising. The default is 0."]
    pub strength: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "tune")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set the denoiser mode. The default is `\"standard\"`. Supported denoiser modes: - 'standard' - 'grain'"]
    pub tune: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Edit atom."]
pub struct EditAtom {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time in seconds for the atom, relative to the input file timeline. When `end_time_offset` is not specified, the `inputs` are used until the end of the atom."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Input.key`s identifying files that should be used in this atom. The listed `inputs` must have the same timeline."]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique key for this atom. Must be specified when using advanced mapping."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time in seconds for the atom, relative to the input file timeline. The default is `0s`."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encoding of an input file such as an audio, video, or text track. Elementary streams must be packaged before mapping and sharing between different output formats."]
pub struct ElementaryStream {
    #[serde(rename = "audioStream")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encoding of an audio stream."]
    pub audio_stream: ::std::option::Option<::std::boxed::Box<AudioStream>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique key for this elementary stream."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "textStream")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encoding of a text stream. For example, closed captions or subtitles."]
    pub text_stream: ::std::option::Option<::std::boxed::Box<TextStream>>,
    #[serde(rename = "videoStream")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encoding of a video stream."]
    pub video_stream: ::std::option::Option<::std::boxed::Box<VideoStream>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encryption settings."]
pub struct Encryption {
    #[serde(rename = "aes128")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for AES-128 encryption."]
    pub aes128: ::std::option::Option<::std::boxed::Box<Aes128Encryption>>,
    #[serde(rename = "iv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. 128 bit Initialization Vector (IV) represented as lowercase hexadecimal digits."]
    pub iv: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. 128 bit encryption key represented as lowercase hexadecimal digits."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpegCenc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for MPEG Common Encryption (MPEG-CENC)."]
    pub mpeg_cenc: ::std::option::Option<::std::boxed::Box<MpegCommonEncryption>>,
    #[serde(rename = "sampleAes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for SAMPLE-AES encryption."]
    pub sample_aes: ::std::option::Option<::std::boxed::Box<SampleAesEncryption>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional information about the reasons for the failure."]
pub struct FailureDetail {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the failure."]
    pub description: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Overlaid jpeg image."]
pub struct Image {
    #[serde(rename = "alpha")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target image opacity. Valid values: `1.0` (solid, default) to `0.0` (transparent)."]
    pub alpha: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized image resolution, based on output video resolution. Valid values: `0.0`–`1.0`. To respect the original image aspect ratio, set either `x` or `y` to `0.0`. To use the original image resolution, set both `x` and `y` to `0.0`."]
    pub resolution: ::std::option::Option<::std::boxed::Box<NormalizedCoordinate>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI of the image in Cloud Storage. For example, `gs://bucket/inputs/image.jpeg`."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Input asset."]
pub struct Input {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique key for this input. Must be specified when using advanced mapping and edit lists."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preprocessingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Preprocessing configurations."]
    pub preprocessing_config: ::std::option::Option<::std::boxed::Box<PreprocessingConfig>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). If empty, the value will be populated from `Job.input_uri`."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Transcoding job resource."]
pub struct Job {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for this job."]
    pub config: ::std::option::Option<::std::boxed::Box<JobConfig>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the job was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the transcoding finished."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. List of failure details. This property may contain additional information about the failure when `failure_reason` is present. *Note*: This feature is not yet available."]
    pub failure_details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FailureDetail>>>,
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A description of the reason for the failure. This property is always present when `state` is `FAILED`."]
    pub failure_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the job. Format: `projects/{project}/locations/{location}/jobs/{job}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The origin URI. *Note*: This feature is not yet available."]
    pub origin_uri: ::std::option::Option<::std::boxed::Box<OriginUri>>,
    #[serde(rename = "outputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`."]
    pub output_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the priority of the job. Enter a value between 0 and 100, where 0 is the lowest priority and 100 is the highest priority. The default is 0."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Estimated fractional progress, from `0` to `1` for each step. *Note*: This feature is not yet available."]
    pub progress: ::std::option::Option<::std::boxed::Box<Progress>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the transcoding started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the job."]
    pub state: ::std::option::Option<JobStateEnum>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`. Preset Transcoder templates: - `preset/{preset_id}` - User defined JobTemplate: `{job_template_id}`"]
    pub template_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ttlAfterCompletionDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30."]
    pub ttl_after_completion_days: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the job."]
pub enum JobStateEnum {
    #[serde(rename = "PROCESSING_STATE_UNSPECIFIED")]
    #[doc = "The processing state is not specified."]
    ProcessingStateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The job is enqueued and will be picked up for processing soon."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The job is being processed."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The job has been completed successfully."]
    Succeeded,
    #[serde(rename = "FAILED")]
    #[doc = "The job has failed. For additional information, see `failure_reason` and `failure_details`"]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Job configuration"]
pub struct JobConfig {
    #[serde(rename = "adBreaks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of ad breaks. Specifies where to insert ad break tags in the output manifests."]
    pub ad_breaks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdBreak>>>,
    #[serde(rename = "editList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Edit atom`s. Defines the ultimate timeline of the resulting file or manifest."]
    pub edit_list: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EditAtom>>>,
    #[serde(rename = "elementaryStreams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of elementary streams."]
    pub elementary_streams:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ElementaryStream>>>,
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of input assets stored in Cloud Storage."]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Input>>>,
    #[serde(rename = "manifests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of output manifests."]
    pub manifests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Manifest>>>,
    #[serde(rename = "muxStreams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of multiplexing settings for output streams."]
    pub mux_streams: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MuxStream>>>,
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output configuration."]
    pub output: ::std::option::Option<::std::boxed::Box<Output>>,
    #[serde(rename = "overlays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of overlays on the output video, in descending Z-order."]
    pub overlays: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Overlay>>>,
    #[serde(rename = "pubsubDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on Pub/Sub."]
    pub pubsub_destination: ::std::option::Option<::std::boxed::Box<PubsubDestination>>,
    #[serde(rename = "spriteSheets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of output sprite sheets."]
    pub sprite_sheets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpriteSheet>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Transcoding job template resource."]
pub struct JobTemplate {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for this template."]
    pub config: ::std::option::Option<::std::boxed::Box<JobConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the job template. Format: `projects/{project}/locations/{location}/jobTemplates/{job_template}`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TranscoderService.ListJobTemplates`."]
pub struct ListJobTemplatesResponse {
    #[serde(rename = "jobTemplates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of job templates in the specified region."]
    pub job_templates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<JobTemplate>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TranscoderService.ListJobs`."]
pub struct ListJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of jobs in the specified region."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Manifest configuration."]
pub struct Manifest {
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the generated file. The default is `\"manifest\"` with the extension suffix corresponding to the `Manifest.type`."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "muxStreams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of user given `MuxStream.key`s that should appear in this manifest. When `Manifest.type` is `HLS`, a media manifest with name `MuxStream.key` and `.m3u8` extension is generated for each element of the `Manifest.mux_streams`."]
    pub mux_streams: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Type of the manifest, can be \"HLS\" or \"DASH\"."]
    pub _type: ::std::option::Option<ManifestTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Type of the manifest, can be \"HLS\" or \"DASH\"."]
pub enum ManifestTypeEnum {
    #[serde(rename = "MANIFEST_TYPE_UNSPECIFIED")]
    #[doc = "The manifest type is not specified."]
    ManifestTypeUnspecified,
    #[serde(rename = "HLS")]
    #[doc = "Create `\"HLS\"` manifest. The corresponding file extension is `\".m3u8\"`."]
    Hls,
    #[serde(rename = "DASH")]
    #[doc = "Create `\"DASH\"` manifest. The corresponding file extension is `\".mpd\"`."]
    Dash,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for MPEG Common Encryption (MPEG-CENC)."]
pub struct MpegCommonEncryption {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. 128 bit Key ID represented as lowercase hexadecimal digits for use with common encryption."]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheme")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specify the encryption scheme. Supported encryption schemes: - 'cenc' - 'cbcs'"]
    pub scheme: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Multiplexing settings for output stream."]
pub struct MuxStream {
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container format. The default is `\"mp4\"` Supported container formats: - 'ts' - 'fmp4'- the corresponding file extension is `\".m4s\"` - 'mp4' - 'vtt'"]
    pub container: ::std::option::Option<::std::string::String>,
    #[serde(rename = "elementaryStreams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `ElementaryStream.key`s multiplexed in this stream."]
    pub elementary_streams: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encryption settings."]
    pub encryption: ::std::option::Option<::std::boxed::Box<Encryption>>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the generated file. The default is `MuxStream.key` with the extension suffix corresponding to the `MuxStream.container`. Individual segments also have an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `\"mux_stream0000000123.ts\"`."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique key for this multiplexed stream. HLS media manifests will be named `MuxStream.key` with the `\".m3u8\"` extension suffix."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Segment settings for `\"ts\"`, `\"fmp4\"` and `\"vtt\"`."]
    pub segment_settings: ::std::option::Option<::std::boxed::Box<SegmentSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "2D normalized coordinates. Default: `{0.0, 0.0}`"]
pub struct NormalizedCoordinate {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized x coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the metadata of the long-running operation."]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Human-readable status of the operation, if any."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The origin URI."]
pub struct OriginUri {
    #[serde(rename = "dash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dash manifest URI. If multiple Dash manifests are created, only the first one is listed."]
    pub dash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HLS manifest URI per https://tools.ietf.org/html/rfc8216#section-4.3.4. If multiple HLS manifests are created, only the first one is listed."]
    pub hls: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of output file(s) in a Cloud Storage bucket."]
pub struct Output {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI for the output file(s). For example, `gs://my-bucket/outputs/`. If empty the value is populated from `Job.output_uri`."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Overlay configuration."]
pub struct Overlay {
    #[serde(rename = "animations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Animations. The list should be chronological, without any time overlap."]
    pub animations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Animation>>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Image overlay."]
    pub image: ::std::option::Option<::std::boxed::Box<Image>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pad filter configuration for the input video. The padded input video is scaled after padding with black to match the output resolution."]
pub struct Pad {
    #[serde(rename = "bottomPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to add to the bottom. The default is 0."]
    pub bottom_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "leftPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to add to the left. The default is 0."]
    pub left_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rightPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to add to the right. The default is 0."]
    pub right_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "topPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of pixels to add to the top. The default is 0."]
    pub top_pixels: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Preprocessing configurations."]
pub struct PreprocessingConfig {
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audio preprocessing configuration."]
    pub audio: ::std::option::Option<::std::boxed::Box<Audio>>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Color preprocessing configuration."]
    pub color: ::std::option::Option<::std::boxed::Box<Color>>,
    #[serde(rename = "crop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the video cropping configuration."]
    pub crop: ::std::option::Option<::std::boxed::Box<Crop>>,
    #[serde(rename = "deblock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deblock preprocessing configuration."]
    pub deblock: ::std::option::Option<::std::boxed::Box<Deblock>>,
    #[serde(rename = "denoise")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Denoise preprocessing configuration."]
    pub denoise: ::std::option::Option<::std::boxed::Box<Denoise>>,
    #[serde(rename = "pad")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the video pad filter configuration."]
    pub pad: ::std::option::Option<::std::boxed::Box<Pad>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Estimated fractional progress for each step, from `0` to `1`."]
pub struct Progress {
    #[serde(rename = "analyzed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated fractional progress for `analyzing` step."]
    pub analyzed: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "encoded")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated fractional progress for `encoding` step."]
    pub encoded: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "notified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated fractional progress for `notifying` step."]
    pub notified: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "uploaded")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated fractional progress for `uploading` step."]
    pub uploaded: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Pub/Sub destination."]
pub struct PubsubDestination {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Pub/Sub topic to publish job completion notification to. For example: `projects/{project}/topics/{topic}`."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for SAMPLE-AES encryption."]
pub struct SampleAesEncryption {
    #[serde(rename = "keyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI of the key delivery service. This URI is inserted into the M3U8 header."]
    pub key_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Segment settings for `\"ts\"`, `\"fmp4\"` and `\"vtt\"`."]
pub struct SegmentSettings {
    #[serde(rename = "individualSegments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Create an individual segment file. The default is `false`."]
    pub individual_segments: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "segmentDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the segments in seconds. The default is `\"6.0s\"`. Note that `segmentDuration` must be greater than or equal to [`gopDuration`](#videostream), and `segmentDuration` must be divisible by [`gopDuration`](#videostream)."]
    pub segment_duration: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Sprite sheet configuration."]
pub struct SpriteSheet {
    #[serde(rename = "columnCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of sprites per row in a sprite sheet. The default is 0, which indicates no maximum limit."]
    pub column_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time in seconds, relative to the output file timeline. When `end_time_offset` is not specified, the sprites are generated until the end of the output file."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filePrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. File name prefix for the generated sprite sheets. Each sprite sheet has an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `\"sprite_sheet0000000123.jpeg\"`."]
    pub file_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format type. The default is `\"jpeg\"`. Supported formats: - 'jpeg'"]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Starting from `0s`, create sprites at regular intervals. Specify the interval value in seconds."]
    pub interval: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quality of the generated sprite sheet. Enter a value between 1 and 100, where 1 is the lowest quality and 100 is the highest quality. The default is 100. A high quality value corresponds to a low image data compression ratio."]
    pub quality: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rowCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of rows per sprite sheet. When the sprite sheet is full, a new sprite sheet is created. The default is 0, which indicates no maximum limit."]
    pub row_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "spriteHeightPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The height of sprite in pixels. Must be an even integer."]
    pub sprite_height_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "spriteWidthPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The width of sprite in pixels. Must be an even integer."]
    pub sprite_width_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time in seconds, relative to the output file timeline. Determines the first sprite to pick. The default is `0s`."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of sprites. Create the specified number of sprites distributed evenly across the timeline of the output media. The default is 100."]
    pub total_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`."]
pub struct TextAtom {
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Job.inputs` that should be embedded in this atom. Only one input is supported."]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TextInput>>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The `EditAtom.key` that references atom with text inputs in the `Job.edit_list`."]
    pub key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies which input file and track should be used."]
pub struct TextInput {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The `Input.key` that identifies the input file."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "track")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The zero-based index of the track in the input file."]
    pub track: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encoding of a text stream. For example, closed captions or subtitles."]
pub struct TextStream {
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The codec for this text stream. The default is `\"webvtt\"`. Supported text codecs: - 'srt' - 'ttml' - 'cea608' - 'cea708' - 'webvtt'"]
    pub codec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BCP-47 language code, such as `\"en-US\"` or `\"sr-Latn\"`. For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mapping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`."]
    pub mapping: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TextAtom>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video stream resource."]
pub struct VideoStream {
    #[serde(rename = "allowOpenGop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether an open Group of Pictures (GOP) structure should be allowed or not. The default is `false`."]
    pub allow_open_gop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "aqStrength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A higher value equals a lower bitrate but smoother image. The default is 0."]
    pub aq_strength: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "bFrameCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of consecutive B-frames. Must be greater than or equal to zero. Must be less than `VideoStream.gop_frame_count` if set. The default is 0."]
    pub b_frame_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "bPyramid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allow B-pyramid for reference frame selection. This may not be supported on all decoders. The default is `false`."]
    pub b_pyramid: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "bitrateBps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The video bitrate in bits per second. Must be between 1 and 1,000,000,000."]
    pub bitrate_bps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Codec type. The following codecs are supported: * `h264` (default) * `h265` * `vp9`"]
    pub codec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crfLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21."]
    pub crf_level: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "enableTwoPass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use two-pass encoding strategy to achieve better video quality. `VideoStream.rate_control_mode` must be `\"vbr\"`. The default is `false`."]
    pub enable_two_pass: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "entropyCoder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entropy coder to use. The default is `\"cabac\"`. Supported entropy coders: - 'cavlc' - 'cabac'"]
    pub entropy_coder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frameRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. The following table shows the computed video FPS given the target FPS (in parenthesis) and input FPS (in the first column): ``` | | (30) | (60) | (25) | (50) | |--------|--------|--------|------|------| | 240 | Fail | Fail | Fail | Fail | | 120 | 30 | 60 | 20 | 30 | | 100 | 25 | 50 | 20 | 30 | | 50 | 25 | 50 | 20 | 30 | | 60 | 30 | 60 | 20 | 30 | | 59.94 | 29.97 | 59.94 | 20 | 30 | | 48 | 24 | 48 | 20 | 30 | | 30 | 30 | 30 | 20 | 30 | | 25 | 25 | 25 | 20 | 30 | | 24 | 24 | 24 | 20 | 30 | | 23.976 | 23.976 | 23.976 | 20 | 30 | | 15 | 15 | 15 | 20 | 30 | | 12 | 12 | 12 | 20 | 30 | | 10 | 10 | 10 | 20 | 30 | ```"]
    pub frame_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "gopDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Select the GOP size based on the specified duration. The default is `\"3s\"`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`."]
    pub gop_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gopFrameCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Select the GOP size based on the specified frame count. Must be greater than zero."]
    pub gop_frame_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "heightPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used."]
    pub height_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pixelFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pixel format to use. The default is `\"yuv420p\"`. Supported pixel formats: - 'yuv420p' pixel format. - 'yuv422p' pixel format. - 'yuv444p' pixel format. - 'yuv420p10' 10-bit HDR pixel format. - 'yuv422p10' 10-bit HDR pixel format. - 'yuv444p10' 10-bit HDR pixel format. - 'yuv420p12' 12-bit HDR pixel format. - 'yuv422p12' 12-bit HDR pixel format. - 'yuv444p12' 12-bit HDR pixel format."]
    pub pixel_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enforces the specified codec preset. The default is `veryfast`. The available options are FFmpeg-compatible. Note that certain values for this field may cause the transcoder to override other fields you set in the `VideoStream` message."]
    pub preset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enforces the specified codec profile. The following profiles are supported: * `baseline` * `main` * `high` (default) The available options are FFmpeg-compatible. Note that certain values for this field may cause the transcoder to override other fields you set in the `VideoStream` message."]
    pub profile: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the `rate_control_mode`. The default is `\"vbr\"`. Supported rate control modes: - 'vbr' - variable bitrate - 'crf' - constant rate factor"]
    pub rate_control_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tune")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enforces the specified codec tune. The available options are FFmpeg-compatible. Note that certain values for this field may cause the transcoder to override other fields you set in the `VideoStream` message."]
    pub tune: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vbvFullnessBits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to 90% of `VideoStream.vbv_size_bits`."]
    pub vbv_fullness_bits: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "vbvSizeBits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to `VideoStream.bitrate_bps`."]
    pub vbv_size_bits: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "widthPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used."]
    pub width_pixels: ::std::option::Option<::std::primitive::i64>,
}
