#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an APK. The resource for ApksService."]
pub struct Apk {
    #[serde(rename = "binary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the binary payload of this APK."]
    pub binary: ::std::option::Option<::std::boxed::Box<ApkBinary>>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version code of the APK, as specified in the manifest file."]
    pub version_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the binary payload of an APK."]
pub struct ApkBinary {
    #[serde(rename = "sha1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha1 hash of the APK payload, encoded as a hex string and matching the output of the sha1sum command."]
    pub sha1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha256 hash of the APK payload, encoded as a hex string and matching the output of the sha256sum command."]
    pub sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to create a new externally hosted APK."]
pub struct ApksAddExternallyHostedRequest {
    #[serde(rename = "externallyHostedApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The definition of the externally-hosted APK and where it is located."]
    pub externally_hosted_apk: ::std::option::Option<::std::boxed::Box<ExternallyHostedApk>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for creating a new externally hosted APK."]
pub struct ApksAddExternallyHostedResponse {
    #[serde(rename = "externallyHostedApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The definition of the externally-hosted APK and where it is located."]
    pub externally_hosted_apk: ::std::option::Option<::std::boxed::Box<ExternallyHostedApk>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all APKs."]
pub struct ApksListResponse {
    #[serde(rename = "apks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All APKs."]
    pub apks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Apk>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this response (\"androidpublisher#apksListResponse\")."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The app details. The resource for DetailsService."]
pub struct AppDetails {
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-visible support email for this app."]
    pub contact_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-visible support telephone number for this app."]
    pub contact_phone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactWebsite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-visible website for this app."]
    pub contact_website: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default language code, in BCP 47 format (eg \"en-US\")."]
    pub default_language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An app edit. The resource for EditsService."]
pub struct AppEdit {
    #[serde(rename = "expiryTimeSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time (as seconds since Epoch) at which the edit will expire and will be no longer valid for use."]
    pub expiry_time_seconds: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifier of the edit. Can be used in subsequent API calls."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a bundle. The resource for BundlesService."]
pub struct Bundle {
    #[serde(rename = "sha1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha1 hash of the upload payload, encoded as a hex string and matching the output of the sha1sum command."]
    pub sha1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha256 hash of the upload payload, encoded as a hex string and matching the output of the sha256sum command."]
    pub sha256: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version code of the Android App Bundle, as specified in the Android App Bundle's base module APK manifest file."]
    pub version_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all bundles."]
pub struct BundlesListResponse {
    #[serde(rename = "bundles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All bundles."]
    pub bundles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bundle>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this response (\"androidpublisher#bundlesListResponse\")."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An entry of conversation between user and developer."]
pub struct Comment {
    #[serde(rename = "developerComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A comment from a developer."]
    pub developer_comment: ::std::option::Option<::std::boxed::Box<DeveloperComment>>,
    #[serde(rename = "userComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A comment from a user."]
    pub user_comment: ::std::option::Option<::std::boxed::Box<UserComment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Country targeting specification."]
pub struct CountryTargeting {
    #[serde(rename = "countries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Countries to target, specified as two letter [CLDR codes](https://unicode.org/cldr/charts/latest/supplemental/territory_containment_un_m_49.html)."]
    pub countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeRestOfWorld")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include \"rest of world\" as well as explicitly targeted countries."]
    pub include_rest_of_world: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a deobfuscation file."]
pub struct DeobfuscationFile {
    #[serde(rename = "symbolType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the deobfuscation file."]
    pub symbol_type: ::std::option::Option<DeobfuscationFileSymbolTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the deobfuscation file."]
pub enum DeobfuscationFileSymbolTypeEnum {
    #[serde(rename = "deobfuscationFileTypeUnspecified")]
    #[doc = "Unspecified deobfuscation file type."]
    DeobfuscationFileTypeUnspecified,
    #[serde(rename = "proguard")]
    #[doc = "Proguard deobfuscation file type."]
    Proguard,
    #[serde(rename = "nativeCode")]
    #[doc = "Native debugging symbols file type."]
    NativeCode,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Responses for the upload."]
pub struct DeobfuscationFilesUploadResponse {
    #[serde(rename = "deobfuscationFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uploaded Deobfuscation File configuration."]
    pub deobfuscation_file: ::std::option::Option<::std::boxed::Box<DeobfuscationFile>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Developer entry from conversation between user and developer."]
pub struct DeveloperComment {
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time at which this comment was updated."]
    pub last_modified: ::std::option::Option<::std::boxed::Box<Timestamp>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the comment, i.e. reply body."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Characteristics of the user's device."]
pub struct DeviceMetadata {
    #[serde(rename = "cpuMake")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device CPU make, e.g. \"Qualcomm\""]
    pub cpu_make: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cpuModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device CPU model, e.g. \"MSM8974\""]
    pub cpu_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device class (e.g. tablet)"]
    pub device_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "glEsVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OpenGL version"]
    pub gl_es_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device manufacturer (e.g. Motorola)"]
    pub manufacturer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nativePlatform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comma separated list of native platforms (e.g. \"arm\", \"arm7\")"]
    pub native_platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device model name (e.g. Droid)"]
    pub product_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ramMb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device RAM in Megabytes, e.g. \"2048\""]
    pub ram_mb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenDensityDpi")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen density in DPI"]
    pub screen_density_dpi: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenHeightPx")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen height in pixels"]
    pub screen_height_px: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenWidthPx")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen width in pixels"]
    pub screen_width_px: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The device spec used to generate a system APK."]
pub struct DeviceSpec {
    #[serde(rename = "screenDensity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen dpi."]
    pub screen_density: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "supportedAbis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported ABI architectures in the order of preference. The values should be the string as reported by the platform, e.g. \"armeabi-v7a\", \"x86_64\"."]
    pub supported_abis: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "supportedLocales")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All installed locales represented as BCP-47 strings, e.g. \"en-US\"."]
    pub supported_locales: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An expansion file. The resource for ExpansionFilesService."]
pub struct ExpansionFile {
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, this field indicates that this APK has an expansion file uploaded to it: this APK does not reference another APK's expansion file. The field's value is the size of the uploaded expansion file in bytes."]
    pub file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referencesVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, this APK's expansion file references another APK's expansion file. The file_size field will not be set."]
    pub references_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for uploading an expansion file."]
pub struct ExpansionFilesUploadResponse {
    #[serde(rename = "expansionFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uploaded expansion file configuration."]
    pub expansion_file: ::std::option::Option<::std::boxed::Box<ExpansionFile>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines an APK available for this application that is hosted externally and not uploaded to Google Play. This function is only available to organizations using Managed Play whose application is configured to restrict distribution to the organizations."]
pub struct ExternallyHostedApk {
    #[serde(rename = "applicationLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The application label."]
    pub application_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "certificateBase64s")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A certificate (or array of certificates if a certificate-chain is used) used to sign this APK, represented as a base64 encoded byte array."]
    pub certificate_base64s: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "externallyHostedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL at which the APK is hosted. This must be an https URL."]
    pub externally_hosted_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileSha1Base64")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sha1 checksum of this APK, represented as a base64 encoded byte array."]
    pub file_sha1_base64: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileSha256Base64")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sha256 checksum of this APK, represented as a base64 encoded byte array."]
    pub file_sha256_base64: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file size in bytes of this APK."]
    pub file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconBase64")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The icon image from the APK, as a base64 encoded byte array."]
    pub icon_base64: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maximumSdk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum SDK supported by this APK (optional)."]
    pub maximum_sdk: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minimumSdk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum SDK targeted by this APK."]
    pub minimum_sdk: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nativeCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The native code environments supported by this APK (optional)."]
    pub native_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The package name."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usesFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The features required by this APK (optional)."]
    pub uses_features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "usesPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permissions requested by this APK."]
    pub uses_permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsesPermission>>>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version code of this APK."]
    pub version_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "versionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version name of this APK."]
    pub version_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An uploaded image. The resource for ImagesService."]
pub struct Image {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique id representing this image."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha1 hash of the image."]
    pub sha1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A sha256 hash of the image."]
    pub sha256: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL that will serve a preview of the image."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for deleting all images."]
pub struct ImagesDeleteAllResponse {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deleted images."]
    pub deleted: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Image>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all images."]
pub struct ImagesListResponse {
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All listed Images."]
    pub images: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Image>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for uploading an image."]
pub struct ImagesUploadResponse {
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uploaded image."]
    pub image: ::std::option::Option<::std::boxed::Box<Image>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An in-app product. The resource for InappproductsService."]
pub struct InAppProduct {
    #[serde(rename = "defaultLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default language of the localized data, as defined by BCP-47. e.g. \"en-US\"."]
    pub default_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default price. Cannot be zero, as in-app products are never free. Always in the developer's Checkout merchant currency."]
    pub default_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "gracePeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Grace period of the subscription, specified in ISO 8601 format. Allows developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values are P0D (zero days), P3D (three days), P7D (seven days), P14D (14 days), and P30D (30 days)."]
    pub grace_period: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of localized title and description data. Map key is the language of the localized data, as defined by BCP-47, e.g. \"en-US\"."]
    pub listings: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<InAppProductListing>>,
    >,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Package name of the parent app."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Prices per buyer region. None of these can be zero, as in-app products are never free. Map key is region code, as defined by ISO 3166-2."]
    pub prices:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Price>>>,
    #[serde(rename = "purchaseType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the product, e.g. a recurring subscription."]
    pub purchase_type: ::std::option::Option<InAppProductPurchaseTypeEnum>,
    #[serde(rename = "sku")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stock-keeping-unit (SKU) of the product, unique within an app."]
    pub sku: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the product, e.g. whether it's active."]
    pub status: ::std::option::Option<InAppProductStatusEnum>,
    #[serde(rename = "subscriptionPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subscription period, specified in ISO 8601 format. Acceptable values are P1W (one week), P1M (one month), P3M (three months), P6M (six months), and P1Y (one year)."]
    pub subscription_period: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trialPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trial period, specified in ISO 8601 format. Acceptable values are anything between P7D (seven days) and P999D (999 days)."]
    pub trial_period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the product, e.g. a recurring subscription."]
pub enum InAppProductPurchaseTypeEnum {
    #[serde(rename = "purchaseTypeUnspecified")]
    #[doc = "Unspecified purchase type."]
    PurchaseTypeUnspecified,
    #[serde(rename = "managedUser")]
    #[doc = "The default product type - one time purchase."]
    ManagedUser,
    #[serde(rename = "subscription")]
    #[doc = "In-app product with a recurring period."]
    Subscription,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the product, e.g. whether it's active."]
pub enum InAppProductStatusEnum {
    #[serde(rename = "statusUnspecified")]
    #[doc = "Unspecified status."]
    StatusUnspecified,
    #[serde(rename = "active")]
    #[doc = "The product is published and active in the store."]
    Active,
    #[serde(rename = "inactive")]
    #[doc = "The product is not published and therefore inactive in the store."]
    Inactive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Store listing of a single in-app product."]
pub struct InAppProductListing {
    #[serde(rename = "benefits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized entitlement benefits for a subscription."]
    pub benefits: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description for the store listing."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title for the store listing."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all in-app products."]
pub struct InappproductsListResponse {
    #[serde(rename = "inappproduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All in-app products."]
    pub inappproduct: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InAppProduct>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this response (\"androidpublisher#inappproductsListResponse\")."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the current page."]
    pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
    #[serde(rename = "tokenPagination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token, to handle a number of products that is over one page."]
    pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An artifact resource which gets created when uploading an APK or Android App Bundle through internal app sharing."]
pub struct InternalAppSharingArtifact {
    #[serde(rename = "certificateFingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sha256 fingerprint of the certificate used to sign the generated artifact."]
    pub certificate_fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The download URL generated for the uploaded artifact. Users that are authorized to download can follow the link to the Play Store app to install it."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sha256 hash of the artifact represented as a lowercase hexadecimal number, matching the output of the sha256sum command."]
    pub sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains the introductory price information for a subscription."]
pub struct IntroductoryPriceInfo {
    #[serde(rename = "introductoryPriceAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Introductory price of the subscription, not including tax. The currency is the same as price_currency_code. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000."]
    pub introductory_price_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "introductoryPriceCurrencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 4217 currency code for the introductory subscription price. For example, if the price is specified in British pounds sterling, price_currency_code is \"GBP\"."]
    pub introductory_price_currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "introductoryPriceCycles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of billing period to offer introductory pricing."]
    pub introductory_price_cycles: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "introductoryPricePeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Introductory price period, specified in ISO 8601 format. Common values are (but not limited to) \"P1W\" (one week), \"P1M\" (one month), \"P3M\" (three months), \"P6M\" (six months), and \"P1Y\" (one year)."]
    pub introductory_price_period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A localized store listing. The resource for ListingsService."]
pub struct Listing {
    #[serde(rename = "fullDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full description of the app."]
    pub full_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language localization code (a BCP-47 language tag; for example, \"de-AT\" for Austrian German)."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description of the app."]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Localized title of the app."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "video")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a promotional YouTube video for the app."]
    pub video: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all localized listings."]
pub struct ListingsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this response (\"androidpublisher#listingsListResponse\")."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All localized listings."]
    pub listings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Listing>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Release notes specification, i.e. language and text."]
pub struct LocalizedText {
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language localization code (a BCP-47 language tag; for example, \"de-AT\" for Austrian German)."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text in the given language."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the current page. List operations that supports paging return only one \"page\" of results. This protocol buffer message describes the page that has been returned."]
pub struct PageInfo {
    #[serde(rename = "resultPerPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of results returned in one page. ! The number of results included in the API response."]
    pub result_per_page: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Index of the first result returned in the current page."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of results available on the backend ! The total number of results in the result set."]
    pub total_results: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a price, i.e. currency and units."]
pub struct Price {
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "3 letter Currency code, as defined by ISO 4217. See java/com/google/common/money/CurrencyCode.java"]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priceMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price in 1/million of the currency base unit, represented as a string."]
    pub price_micros: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ProductPurchase resource indicates the status of a user's inapp product purchase."]
pub struct ProductPurchase {
    #[serde(rename = "acknowledgementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The acknowledgement state of the inapp product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged"]
    pub acknowledgement_state: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "consumptionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The consumption state of the inapp product. Possible values are: 0. Yet to be consumed 1. Consumed"]
    pub consumption_state: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "developerPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-specified string that contains supplemental information about an order."]
    pub developer_payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This kind represents an inappPurchase object in the androidpublisher service."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obfuscatedExternalAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An obfuscated version of the id that is uniquely associated with the user's account in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made."]
    pub obfuscated_external_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obfuscatedExternalProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made."]
    pub obfuscated_external_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order id associated with the purchase of the inapp product."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The inapp product SKU."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The purchase state of the order. Possible values are: 0. Purchased 1. Canceled 2. Pending"]
    pub purchase_state: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "purchaseTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970)."]
    pub purchase_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The purchase token generated to identify this purchase."]
    pub purchase_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code) 2. Rewarded (i.e. from watching a video ad instead of paying)"]
    pub purchase_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity associated with the purchase of the inapp product."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 3166-1 alpha-2 billing region code of the user at the time the product was granted."]
    pub region_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the product.purchases.acknowledge API."]
pub struct ProductPurchasesAcknowledgeRequest {
    #[serde(rename = "developerPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payload to attach to the purchase."]
    pub developer_payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Android app review."]
pub struct Review {
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the user who wrote the review."]
    pub author_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A repeated field containing comments for the review."]
    pub comments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    #[serde(rename = "reviewId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for this review."]
    pub review_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of replying/updating a reply to review."]
pub struct ReviewReplyResult {
    #[serde(rename = "lastEdited")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the reply took effect."]
    pub last_edited: ::std::option::Option<::std::boxed::Box<Timestamp>>,
    #[serde(rename = "replyText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reply text that was applied."]
    pub reply_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing reviews."]
pub struct ReviewsListResponse {
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the current page."]
    pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
    #[serde(rename = "reviews")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of reviews."]
    pub reviews: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Review>>>,
    #[serde(rename = "tokenPagination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token, to handle a number of products that is over one page."]
    pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to reply to review or update existing reply."]
pub struct ReviewsReplyRequest {
    #[serde(rename = "replyText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text to set as the reply. Replies of more than approximately 350 characters will be rejected. HTML tags will be stripped."]
    pub reply_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response on status of replying to a review."]
pub struct ReviewsReplyResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of replying/updating a reply to review."]
    pub result: ::std::option::Option<::std::boxed::Box<ReviewReplyResult>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey)."]
pub struct SubscriptionCancelSurveyResult {
    #[serde(rename = "cancelSurveyReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cancellation reason the user chose in the survey. Possible values are: 0. Other 1. I don't use this service enough 2. Technical issues 3. Cost-related reasons 4. I found a better app"]
    pub cancel_survey_reason: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "userInputCancelReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The customized input cancel reason from the user. Only present when cancelReason is 0."]
    pub user_input_cancel_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SubscriptionDeferralInfo contains the data needed to defer a subscription purchase to a future expiry time."]
pub struct SubscriptionDeferralInfo {
    #[serde(rename = "desiredExpiryTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired next expiry time to assign to the subscription, in milliseconds since the Epoch. The given time must be later/greater than the current expiry time for the subscription."]
    pub desired_expiry_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expectedExpiryTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expected expiry time for the subscription. If the current expiry time for the subscription is not the value specified here, the deferral will not occur."]
    pub expected_expiry_time_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains the price change information for a subscription that can be used to control the user journey for the price change in the app. This can be in the form of seeking confirmation from the user or tailoring the experience for a successful conversion."]
pub struct SubscriptionPriceChange {
    #[serde(rename = "newPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new price the subscription will renew with if the price change is accepted by the user."]
    pub new_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the price change. Possible values are: 0. Outstanding: State for a pending price change waiting for the user to agree. In this state, you can optionally seek confirmation from the user using the In-App API. 1. Accepted: State for an accepted price change that the subscription will renew with unless it's canceled. The price change takes effect on a future date when the subscription renews. Note that the change might not occur when the subscription is renewed next."]
    pub state: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SubscriptionPurchase resource indicates the status of a user's subscription purchase."]
pub struct SubscriptionPurchase {
    #[serde(rename = "acknowledgementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The acknowledgement state of the subscription product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged"]
    pub acknowledgement_state: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "autoRenewing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the subscription will automatically be renewed when it reaches its current expiry time."]
    pub auto_renewing: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "autoResumeTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the subscription will be automatically resumed, in milliseconds since the Epoch. Only present if the user has requested to pause the subscription."]
    pub auto_resume_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason why a subscription was canceled or is not auto-renewing. Possible values are: 0. User canceled the subscription 1. Subscription was canceled by the system, for example because of a billing problem 2. Subscription was replaced with a new subscription 3. Subscription was canceled by the developer"]
    pub cancel_reason: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "cancelSurveyResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey)."]
    pub cancel_survey_result:
        ::std::option::Option<::std::boxed::Box<SubscriptionCancelSurveyResult>>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 3166-1 alpha-2 billing country/region code of the user at the time the subscription was granted."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "developerPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-specified string that contains supplemental information about an order."]
    pub developer_payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiryTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the subscription will expire, in milliseconds since the Epoch."]
    pub expiry_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User account identifier in the third-party service. Only present if account linking happened as part of the subscription purchase flow."]
    pub external_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The family name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'."]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The given name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'."]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "introductoryPriceInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Introductory price information of the subscription. This is only present when the subscription was purchased with an introductory price. This field does not indicate the subscription is currently in introductory price period."]
    pub introductory_price_info: ::std::option::Option<::std::boxed::Box<IntroductoryPriceInfo>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This kind represents a subscriptionPurchase object in the androidpublisher service."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkedPurchaseToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The purchase token of the originating purchase if this subscription is one of the following: 0. Re-signup of a canceled but non-lapsed subscription 1. Upgrade/downgrade from a previous subscription For example, suppose a user originally signs up and you receive purchase token X, then the user cancels and goes through the resignup flow (before their subscription lapses) and you receive purchase token Y, and finally the user upgrades their subscription and you receive purchase token Z. If you call this API with purchase token Z, this field will be set to Y. If you call this API with purchase token Y, this field will be set to X. If you call this API with purchase token X, this field will not be set."]
    pub linked_purchase_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obfuscatedExternalAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An obfuscated version of the id that is uniquely associated with the user's account in your app. Present for the following purchases: * If account linking happened as part of the subscription purchase flow. * It was specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made."]
    pub obfuscated_external_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obfuscatedExternalProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made."]
    pub obfuscated_external_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order id of the latest recurring order associated with the purchase of the subscription."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paymentState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The payment state of the subscription. Possible values are: 0. Payment pending 1. Payment received 2. Free trial 3. Pending deferred upgrade/downgrade Not present for canceled, expired subscriptions."]
    pub payment_state: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "priceAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the subscription, not including tax. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000."]
    pub price_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priceChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest price change information available. This is present only when there is an upcoming price change for the subscription yet to be applied. Once the subscription renews with the new price or the subscription is canceled, no price change information will be returned."]
    pub price_change: ::std::option::Option<::std::boxed::Box<SubscriptionPriceChange>>,
    #[serde(rename = "priceCurrencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 4217 currency code for the subscription price. For example, if the price is specified in British pounds sterling, price_currency_code is \"GBP\"."]
    pub price_currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google profile id of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'."]
    pub profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The profile name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'."]
    pub profile_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The promotion code applied on this purchase. This field is only set if a vanity code promotion is applied when the subscription was purchased."]
    pub promotion_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of promotion applied on this purchase. This field is only set if a promotion is applied when the subscription was purchased. Possible values are: 0. One time code 1. Vanity code"]
    pub promotion_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "purchaseType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of purchase of the subscription. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code)"]
    pub purchase_type: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the subscription was granted, in milliseconds since the Epoch."]
    pub start_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userCancellationTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the subscription was canceled by the user, in milliseconds since the epoch. Only present if cancelReason is 0."]
    pub user_cancellation_time_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the purchases.subscriptions.acknowledge API."]
pub struct SubscriptionPurchasesAcknowledgeRequest {
    #[serde(rename = "developerPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payload to attach to the purchase."]
    pub developer_payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the purchases.subscriptions.defer API."]
pub struct SubscriptionPurchasesDeferRequest {
    #[serde(rename = "deferralInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The information about the new desired expiry time for the subscription."]
    pub deferral_info: ::std::option::Option<::std::boxed::Box<SubscriptionDeferralInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the purchases.subscriptions.defer API."]
pub struct SubscriptionPurchasesDeferResponse {
    #[serde(rename = "newExpiryTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new expiry time for the subscription in milliseconds since the Epoch."]
    pub new_expiry_time_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to list previously created system APK variants."]
pub struct SystemApksListResponse {
    #[serde(rename = "variants")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All system APK variants created."]
    pub variants: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variant>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The testers of an app. The resource for TestersService."]
pub struct Testers {
    #[serde(rename = "googleGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All testing Google Groups, as email addresses."]
    pub google_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Timestamp represents a point in time independent of any time zone or local calendar, encoded as a count of seconds and fractions of seconds at nanosecond resolution. The count is relative to an epoch at UTC midnight on January 1, 1970."]
pub struct Timestamp {
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-negative fractions of a second at nanosecond resolution. Must be from 0 to 999,999,999 inclusive."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents seconds of UTC time since Unix epoch."]
    pub seconds: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pagination information returned by a List operation when token pagination is enabled. List operations that supports paging return only one \"page\" of results. This protocol buffer message describes the page that has been returned. When using token pagination, clients should use the next/previous token to get another page of the result. The presence or absence of next/previous token indicates whether a next/previous page is available and provides a mean of accessing this page. ListRequest.page_token should be set to either next_page_token or previous_page_token to access another page."]
pub struct TokenPagination {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tokens to pass to the standard list field 'page_token'. Whenever available, tokens are preferred over manipulating start_index."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track configuration. The resource for TracksService."]
pub struct Track {
    #[serde(rename = "releases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In a read request, represents all active releases in the track. In an update request, represents desired changes."]
    pub releases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrackRelease>>>,
    #[serde(rename = "track")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the track."]
    pub track: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A release within a track."]
pub struct TrackRelease {
    #[serde(rename = "countryTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts a release to a specific set of countries."]
    pub country_targeting: ::std::option::Option<::std::boxed::Box<CountryTargeting>>,
    #[serde(rename = "inAppUpdatePriority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In-app update priority of the release. All newly added APKs in the release will be considered at this priority. Can take values in the range [0, 5], with 5 the highest priority. Defaults to 0. in_app_update_priority can not be updated once the release is rolled out. See https://developer.android.com/guide/playcore/in-app-updates."]
    pub in_app_update_priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The release name. Not required to be unique. If not set, the name is generated from the APK's version_name. If the release contains multiple APKs, the name is generated from the date."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseNotes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of what is new in this release."]
    pub release_notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedText>>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the release."]
    pub status: ::std::option::Option<TrackReleaseStatusEnum>,
    #[serde(rename = "userFraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fraction of users who are eligible for a staged release. 0 < fraction < 1. Can only be set when status is \"inProgress\" or \"halted\"."]
    pub user_fraction: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "versionCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version codes of all APKs in the release. Must include version codes to retain from previous releases."]
    pub version_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the release."]
pub enum TrackReleaseStatusEnum {
    #[serde(rename = "statusUnspecified")]
    #[doc = "Unspecified status."]
    StatusUnspecified,
    #[serde(rename = "draft")]
    #[doc = "The release's APKs are not being served to users."]
    Draft,
    #[serde(rename = "inProgress")]
    #[doc = "The release's APKs are being served to a fraction of users, determined by 'user_fraction'."]
    InProgress,
    #[serde(rename = "halted")]
    #[doc = "The release's APKs will no longer be served to users. Users who already have these APKs are unaffected."]
    Halted,
    #[serde(rename = "completed")]
    #[doc = "The release will have no further changes. Its APKs are being served to all users, unless they are eligible to APKs of a more recent release."]
    Completed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response listing all tracks."]
pub struct TracksListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this response (\"androidpublisher#tracksListResponse\")."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All tracks."]
    pub tracks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Track>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User entry from conversation between user and developer."]
pub struct UserComment {
    #[serde(rename = "androidOsVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integer Android SDK version of the user's device at the time the review was written, e.g. 23 is Marshmallow. May be absent."]
    pub android_os_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "appVersionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integer version code of the app as installed at the time the review was written. May be absent."]
    pub app_version_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "appVersionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String version name of the app as installed at the time the review was written. May be absent."]
    pub app_version_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Codename for the reviewer's device, e.g. klte, flounder. May be absent."]
    pub device: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the characteristics of the user's device."]
    pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time at which this comment was updated."]
    pub last_modified: ::std::option::Option<::std::boxed::Box<Timestamp>>,
    #[serde(rename = "originalText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Untranslated text of the review, where the review was translated. If the review was not translated this is left blank."]
    pub original_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reviewerLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for the reviewer. This is taken from the device settings so is not guaranteed to match the language the review is written in. May be absent."]
    pub reviewer_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "starRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The star rating associated with the review, from 1 to 5."]
    pub star_rating: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the comment, i.e. review body. In some cases users have been able to write a review with separate title and body; in those cases the title and body are concatenated and separated by a tab character."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbsDownCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of users who have given this review a thumbs down."]
    pub thumbs_down_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "thumbsUpCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of users who have given this review a thumbs up."]
    pub thumbs_up_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A permission used by this APK."]
pub struct UsesPermission {
    #[serde(rename = "maxSdkVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optionally, the maximum SDK version for which the permission is required."]
    pub max_sdk_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the permission requested."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "APK that is suitable for inclusion in a system image. The resource of SystemApksService."]
pub struct Variant {
    #[serde(rename = "deviceSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device spec used to generate the APK."]
    pub device_spec: ::std::option::Option<::std::boxed::Box<DeviceSpec>>,
    #[serde(rename = "variantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of a previously created system APK variant."]
    pub variant_id: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A VoidedPurchase resource indicates a purchase that was either canceled/refunded/charged-back."]
pub struct VoidedPurchase {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This kind represents a voided purchase object in the androidpublisher service."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order id which uniquely identifies a one-time purchase, subscription purchase, or subscription renewal."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the purchase was made, in milliseconds since the epoch (Jan 1, 1970)."]
    pub purchase_time_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token which uniquely identifies a one-time purchase or subscription. To uniquely identify subscription renewals use order_id (available starting from version 3 of the API)."]
    pub purchase_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "voidedReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason why the purchase was voided, possible values are: 0. Other 1. Remorse 2. Not_received 3. Defective 4. Accidental_purchase 5. Fraud 6. Friendly_fraud 7. Chargeback"]
    pub voided_reason: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "voidedSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initiator of voided purchase, possible values are: 0. User 1. Developer 2. Google"]
    pub voided_source: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "voidedTimeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the purchase was canceled/refunded/charged-back, in milliseconds since the epoch (Jan 1, 1970)."]
    pub voided_time_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the voidedpurchases.list API."]
pub struct VoidedPurchasesListResponse {
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General pagination information."]
    pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
    #[serde(rename = "tokenPagination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination information for token pagination."]
    pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
    #[serde(rename = "voidedPurchases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voided_purchases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VoidedPurchase>>>,
}
