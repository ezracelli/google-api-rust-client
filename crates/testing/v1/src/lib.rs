#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies an account and how to log into it."]
pub struct Account {
    #[serde(rename = "googleAuto")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An automatic google login account."]
    pub google_auto: ::std::option::Option<::std::boxed::Box<GoogleAuto>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single Android device."]
pub struct AndroidDevice {
    #[serde(rename = "androidModelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub android_model_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "androidVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub android_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub orientation: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The currently supported Android devices."]
pub struct AndroidDeviceCatalog {
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported Android device models."]
    pub models: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AndroidModel>>>,
    #[serde(rename = "runtimeConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported runtime configurations."]
    pub runtime_configuration:
        ::std::option::Option<::std::boxed::Box<AndroidRuntimeConfiguration>>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported Android OS versions."]
    pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AndroidVersion>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of Android device configurations in which the test is to be executed."]
pub struct AndroidDeviceList {
    #[serde(rename = "androidDevices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of Android devices."]
    pub android_devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AndroidDevice>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A test of an Android application that can control an Android component independently of its normal lifecycle. Android instrumentation tests run an application APK and test APK inside the same process on a virtual or physical AndroidDevice. They also specify a test runner class, such as com.google.GoogleTestRunner, which can vary on the specific instrumentation framework chosen. See for more information on types of Android tests."]
pub struct AndroidInstrumentationTest {
    #[serde(rename = "appApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The APK for the application under test."]
    pub app_apk: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "appBundle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A multi-apk app bundle for the application under test."]
    pub app_bundle: ::std::option::Option<::std::boxed::Box<AppBundle>>,
    #[serde(rename = "appPackageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The java package for the application under test. The default value is determined by examining the application's manifest."]
    pub app_package_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orchestratorOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator."]
    pub orchestrator_option:
        ::std::option::Option<AndroidInstrumentationTestOrchestratorOptionEnum>,
    #[serde(rename = "shardingOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The option to run tests in multiple shards in parallel."]
    pub sharding_option: ::std::option::Option<::std::boxed::Box<ShardingOption>>,
    #[serde(rename = "testApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The APK containing the test code to be executed."]
    pub test_apk: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "testPackageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The java package for the test to be executed. The default value is determined by examining the application's manifest."]
    pub test_package_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testRunnerClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The InstrumentationTestRunner class. The default value is determined by examining the application's manifest."]
    pub test_runner_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testTargets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each target must be fully qualified with the package name or class name, in one of these formats: - \"package package_name\" - \"class package_name.class_name\" - \"class package_name.class_name#method_name\" If empty, all targets in the module will be run."]
    pub test_targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator."]
pub enum AndroidInstrumentationTestOrchestratorOptionEnum {
    #[serde(rename = "ORCHESTRATOR_OPTION_UNSPECIFIED")]
    #[doc = "Default value: the server will choose the mode. Currently implies that the test will run without the orchestrator. In the future, all instrumentation tests will be run with the orchestrator. Using the orchestrator is highly encouraged because of all the benefits it offers."]
    OrchestratorOptionUnspecified,
    #[serde(rename = "USE_ORCHESTRATOR")]
    #[doc = "Run test using orchestrator. ** Only compatible with AndroidJUnitRunner version 1.0 or higher! ** Recommended."]
    UseOrchestrator,
    #[serde(rename = "DO_NOT_USE_ORCHESTRATOR")]
    #[doc = "Run test without using orchestrator."]
    DoNotUseOrchestrator,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of Android device configuration permutations is defined by the the cross-product of the given axes. Internally, the given AndroidMatrix will be expanded into a set of AndroidDevices. Only supported permutations will be instantiated. Invalid permutations (e.g., incompatible models/versions) are ignored."]
pub struct AndroidMatrix {
    #[serde(rename = "androidModelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ids of the set of Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub android_model_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "androidVersionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ids of the set of Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub android_version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "locales")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of locales the test device will enable for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub locales: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "orientations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of orientations to test with. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub orientations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of an Android device tests may be run on."]
pub struct AndroidModel {
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The company that this device is branded with. Example: \"Google\", \"Samsung\"."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "codename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the industrial design. This corresponds to android.os.Build.DEVICE."]
    pub codename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this device is virtual or physical."]
    pub form: ::std::option::Option<AndroidModelFormEnum>,
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this device is a phone, tablet, wearable, etc."]
    pub form_factor: ::std::option::Option<AndroidModelFormFactorEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique opaque id for this model. Use this for invoking the TestExecutionService."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lowFpsVideoRecording")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if and only if tests with this model are recorded by stitching together screenshots. See use_low_spec_video_recording in device config."]
    pub low_fps_video_recording: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The manufacturer of this device."]
    pub manufacturer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable marketing name for this device model. Examples: \"Nexus 5\", \"Galaxy S5\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "screenDensity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen density in DPI. This corresponds to ro.sf.lcd_density"]
    pub screen_density: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenX")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen size in the horizontal (X) dimension measured in pixels."]
    pub screen_x: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenY")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen size in the vertical (Y) dimension measured in pixels."]
    pub screen_y: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "supportedAbis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of supported ABIs for this device. This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2. The most preferred ABI is the first element in the list. Elements are optionally prefixed by \"version_id:\" (where version_id is the id of an AndroidVersion), denoting an ABI that is supported only on a particular version."]
    pub supported_abis: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "supportedVersionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of Android versions this device supports."]
    pub supported_version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a thumbnail image (photo) of the device. e.g. https://lh3.googleusercontent.com/90WcauuJiCYABEl8U0lcZeuS5STUbf2yW..."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether this device is virtual or physical."]
pub enum AndroidModelFormEnum {
    #[serde(rename = "DEVICE_FORM_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    DeviceFormUnspecified,
    #[serde(rename = "VIRTUAL")]
    #[doc = "Android virtual device using Compute Engine native virtualization. Firebase Test Lab only."]
    Virtual,
    #[serde(rename = "PHYSICAL")]
    #[doc = "Actual hardware."]
    Physical,
    #[serde(rename = "EMULATOR")]
    #[doc = "Android virtual device using emulator in nested virtualization. Equivalent to Android Studio."]
    Emulator,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether this device is a phone, tablet, wearable, etc."]
pub enum AndroidModelFormFactorEnum {
    #[serde(rename = "DEVICE_FORM_FACTOR_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    DeviceFormFactorUnspecified,
    #[serde(rename = "PHONE")]
    #[doc = "This device has the shape of a phone."]
    Phone,
    #[serde(rename = "TABLET")]
    #[doc = "This device has the shape of a tablet."]
    Tablet,
    #[serde(rename = "WEARABLE")]
    #[doc = "This device has the shape of a watch or other wearable."]
    Wearable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A test of an android application that explores the application on a virtual or physical Android Device, finding culprits and crashes as it goes. Next tag: 30"]
pub struct AndroidRoboTest {
    #[serde(rename = "appApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The APK for the application under test."]
    pub app_apk: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "appBundle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A multi-apk app bundle for the application under test."]
    pub app_bundle: ::std::option::Option<::std::boxed::Box<AppBundle>>,
    #[serde(rename = "appInitialActivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial activity that should be used to start the app."]
    pub app_initial_activity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appPackageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The java package for the application under test. The default value is determined by examining the application's manifest."]
    pub app_package_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The max depth of the traversal stack Robo can explore. Needs to be at least 2 to make Robo explore the app beyond the first activity. Default is 50."]
    pub max_depth: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The max number of steps Robo can execute. Default is no limit."]
    pub max_steps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "roboDirectives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of directives Robo should apply during the crawl. This allows users to customize the crawl. For example, the username and password for a test account can be provided."]
    pub robo_directives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RoboDirective>>>,
    #[serde(rename = "roboScript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A JSON file with a sequence of actions Robo should perform as a prologue for the crawl."]
    pub robo_script: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "startingIntents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intents used to launch the app for the crawl. If none are provided, then the main launcher activity is launched. If some are provided, then only those provided are launched (the main launcher activity must be provided explicitly)."]
    pub starting_intents:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RoboStartingIntent>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Android configuration that can be selected at the time a test is run."]
pub struct AndroidRuntimeConfiguration {
    #[serde(rename = "locales")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of available locales."]
    pub locales: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Locale>>>,
    #[serde(rename = "orientations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of available orientations."]
    pub orientations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Orientation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A test of an Android Application with a Test Loop. The intent \\ will be implicitly added, since Games is the only user of this api, for the time being."]
pub struct AndroidTestLoop {
    #[serde(rename = "appApk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The APK for the application under test."]
    pub app_apk: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "appBundle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A multi-apk app bundle for the application under test."]
    pub app_bundle: ::std::option::Option<::std::boxed::Box<AppBundle>>,
    #[serde(rename = "appPackageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The java package for the application under test. The default is determined by examining the application's manifest."]
    pub app_package_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scenarioLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of scenario labels that should be run during the test. The scenario labels should map to labels defined in the application's manifest. For example, player_experience and com.google.test.loops.player_experience add all of the loops labeled in the manifest with the com.google.test.loops.player_experience name to the execution. Scenarios can also be specified in the scenarios field."]
    pub scenario_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "scenarios")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of scenarios that should be run during the test. The default is all test loops, derived from the application's manifest."]
    pub scenarios: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A version of the Android OS."]
pub struct AndroidVersion {
    #[serde(rename = "apiLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API level for this Android version. Examples: 18, 19."]
    pub api_level: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "codeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The code name for this Android version. Examples: \"JellyBean\", \"KitKat\"."]
    pub code_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Market share for this version."]
    pub distribution: ::std::option::Option<::std::boxed::Box<Distribution>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque id for this Android version. Use this id to invoke the TestExecutionService."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releaseDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date this Android version became available in the market."]
    pub release_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "versionString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string representing this version of the Android OS. Examples: \"4.3\", \"4.4\"."]
    pub version_string: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Android package file to install."]
pub struct Apk {
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to an APK to be installed on the device before the test begins."]
    pub location: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The java package for the APK to be installed. Value is determined by examining the application's manifest."]
    pub package_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Android application details based on application manifest and apk archive contents."]
pub struct ApkDetail {
    #[serde(rename = "apkManifest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub apk_manifest: ::std::option::Option<::std::boxed::Box<ApkManifest>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Android app manifest. See http://developer.android.com/guide/topics/manifest/manifest-intro.html"]
pub struct ApkManifest {
    #[serde(rename = "applicationLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-readable name for the application."]
    pub application_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intentFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub intent_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IntentFilter>>>,
    #[serde(rename = "maxSdkVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum API level on which the application is designed to run."]
    pub max_sdk_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minSdkVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum API level required for the application to run."]
    pub min_sdk_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full Java-style package name for this application, e.g. \"com.example.foo\"."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetSdkVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the API Level on which the application is designed to run."]
    pub target_sdk_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "usesPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Permissions declared to be used by the application"]
    pub uses_permission: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Android App Bundle file format, containing a BundleConfig.pb file, a base module directory, zero or more dynamic feature module directories. See https://developer.android.com/guide/app-bundle/build for guidance on building App Bundles."]
pub struct AppBundle {
    #[serde(rename = "bundleLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = ".aab file representing the app bundle under test."]
    pub bundle_location: ::std::option::Option<::std::boxed::Box<FileReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing the current state of the specified test matrix."]
pub struct CancelTestMatrixResponse {
    #[serde(rename = "testState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect."]
    pub test_state: ::std::option::Option<CancelTestMatrixResponseTestStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect."]
pub enum CancelTestMatrixResponseTestStateEnum {
    #[serde(rename = "TEST_STATE_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    TestStateUnspecified,
    #[serde(rename = "VALIDATING")]
    #[doc = "The execution or matrix is being validated."]
    Validating,
    #[serde(rename = "PENDING")]
    #[doc = "The execution or matrix is waiting for resources to become available."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The execution is currently being processed. Can only be set on an execution."]
    Running,
    #[serde(rename = "FINISHED")]
    #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
    Finished,
    #[serde(rename = "ERROR")]
    #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
    Error,
    #[serde(rename = "UNSUPPORTED_ENVIRONMENT")]
    #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
    UnsupportedEnvironment,
    #[serde(rename = "INCOMPATIBLE_ENVIRONMENT")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
    IncompatibleEnvironment,
    #[serde(rename = "INCOMPATIBLE_ARCHITECTURE")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
    IncompatibleArchitecture,
    #[serde(rename = "CANCELLED")]
    #[doc = "The user cancelled the execution. Can only be set on an execution."]
    Cancelled,
    #[serde(rename = "INVALID")]
    #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
    Invalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the client which invoked the test."]
pub struct ClientInfo {
    #[serde(rename = "clientInfoDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of detailed information about client."]
    pub client_info_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientInfoDetail>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Client name, such as gcloud."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Key-value pair of detailed information about the client which invoked the test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}."]
pub struct ClientInfoDetail {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The key of detailed client information."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The value of detailed client information."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single device file description."]
pub struct DeviceFile {
    #[serde(rename = "obbFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to an opaque binary blob file."]
    pub obb_file: ::std::option::Option<::std::boxed::Box<ObbFile>>,
    #[serde(rename = "regularFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to a regular file."]
    pub regular_file: ::std::option::Option<::std::boxed::Box<RegularFile>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single device IP block"]
pub struct DeviceIpBlock {
    #[serde(rename = "addedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date this block was added to Firebase Test Lab"]
    pub added_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "block")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An IP address block in CIDR notation eg: 34.68.194.64/29"]
    pub block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this block is used by physical or virtual devices"]
    pub form: ::std::option::Option<DeviceIpBlockFormEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether this block is used by physical or virtual devices"]
pub enum DeviceIpBlockFormEnum {
    #[serde(rename = "DEVICE_FORM_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    DeviceFormUnspecified,
    #[serde(rename = "VIRTUAL")]
    #[doc = "Android virtual device using Compute Engine native virtualization. Firebase Test Lab only."]
    Virtual,
    #[serde(rename = "PHYSICAL")]
    #[doc = "Actual hardware."]
    Physical,
    #[serde(rename = "EMULATOR")]
    #[doc = "Android virtual device using emulator in nested virtualization. Equivalent to Android Studio."]
    Emulator,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of IP blocks used by the Firebase Test Lab"]
pub struct DeviceIpBlockCatalog {
    #[serde(rename = "ipBlocks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device IP blocks used by Firebase Test Lab"]
    pub ip_blocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceIpBlock>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data about the relative number of devices running a given configuration of the Android platform."]
pub struct Distribution {
    #[serde(rename = "marketShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated fraction (0-1) of the total market with this configuration."]
    pub market_share: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "measurementTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this distribution was measured."]
    pub measurement_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The environment in which the test is run."]
pub struct Environment {
    #[serde(rename = "androidDevice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Android device which must be used with an Android test."]
    pub android_device: ::std::option::Option<::std::boxed::Box<AndroidDevice>>,
    #[serde(rename = "iosDevice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An iOS device which must be used with an iOS test."]
    pub ios_device: ::std::option::Option<::std::boxed::Box<IosDevice>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The matrix of environments in which the test is to be executed."]
pub struct EnvironmentMatrix {
    #[serde(rename = "androidDeviceList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Android devices; the test will be run only on the specified devices."]
    pub android_device_list: ::std::option::Option<::std::boxed::Box<AndroidDeviceList>>,
    #[serde(rename = "androidMatrix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A matrix of Android devices."]
    pub android_matrix: ::std::option::Option<::std::boxed::Box<AndroidMatrix>>,
    #[serde(rename = "iosDeviceList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of iOS devices."]
    pub ios_device_list: ::std::option::Option<::std::boxed::Box<IosDeviceList>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A key-value pair passed as an environment variable to the test."]
pub struct EnvironmentVariable {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key for the environment variable."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for the environment variable."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a file, used for user inputs."]
pub struct FileReference {
    #[serde(rename = "gcsPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding)"]
    pub gcs_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing the details of the specified Android application APK."]
pub struct GetApkDetailsResponse {
    #[serde(rename = "apkDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the Android APK."]
    pub apk_detail: ::std::option::Option<::std::boxed::Box<ApkDetail>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enables automatic Google account login. If set, the service automatically generates a Google test account and adds it to the device, before executing the test. Note that test accounts might be reused. Many applications show their full set of functionalities when an account is present on the device. Logging into the device with these generated accounts allows testing more functionalities."]
pub struct GoogleAuto {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A storage location within Google cloud storage (GCS)."]
pub struct GoogleCloudStorage {
    #[serde(rename = "gcsPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The path to a directory in GCS that will eventually contain the results for this test. The requesting user must have write access on the bucket in the supplied path."]
    pub gcs_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The section of an tag. https://developer.android.com/guide/topics/manifest/intent-filter-element.html"]
pub struct IntentFilter {
    #[serde(rename = "actionNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The android:name value of the tag."]
    pub action_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "categoryNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The android:name value of the tag."]
    pub category_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The android:mimeType value of the tag."]
    pub mime_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single iOS device."]
pub struct IosDevice {
    #[serde(rename = "iosModelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the iOS device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub ios_model_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iosVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the iOS major software version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub ios_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options."]
    pub orientation: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The currently supported iOS devices."]
pub struct IosDeviceCatalog {
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported iOS device models."]
    pub models: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosModel>>>,
    #[serde(rename = "runtimeConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported runtime configurations."]
    pub runtime_configuration: ::std::option::Option<::std::boxed::Box<IosRuntimeConfiguration>>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported iOS software versions."]
    pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosVersion>>>,
    #[serde(rename = "xcodeVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of supported Xcode versions."]
    pub xcode_versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<XcodeVersion>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A file or directory to install on the device before the test starts."]
pub struct IosDeviceFile {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bundle id of the app where this file lives. iOS apps sandbox their own filesystem, so app files must specify which app installed on the device."]
    pub bundle_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source file"]
    pub content: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "devicePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the file on the device, inside the app's sandboxed filesystem"]
    pub device_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of iOS device configurations in which the test is to be executed."]
pub struct IosDeviceList {
    #[serde(rename = "iosDevices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of iOS devices."]
    pub ios_devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosDevice>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of an iOS device tests may be run on."]
pub struct IosModel {
    #[serde(rename = "deviceCapabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device capabilities. Copied from https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html"]
    pub device_capabilities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this device is a phone, tablet, wearable, etc."]
    pub form_factor: ::std::option::Option<IosModelFormFactorEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique opaque id for this model. Use this for invoking the TestExecutionService."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable name for this device model. Examples: \"iPhone 4s\", \"iPad Mini 2\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "screenDensity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen density in DPI."]
    pub screen_density: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenX")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen size in the horizontal (X) dimension measured in pixels."]
    pub screen_x: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "screenY")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Screen size in the vertical (Y) dimension measured in pixels."]
    pub screen_y: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "supportedVersionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of iOS major software versions this device supports."]
    pub supported_version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether this device is a phone, tablet, wearable, etc."]
pub enum IosModelFormFactorEnum {
    #[serde(rename = "DEVICE_FORM_FACTOR_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    DeviceFormFactorUnspecified,
    #[serde(rename = "PHONE")]
    #[doc = "This device has the shape of a phone."]
    Phone,
    #[serde(rename = "TABLET")]
    #[doc = "This device has the shape of a tablet."]
    Tablet,
    #[serde(rename = "WEARABLE")]
    #[doc = "This device has the shape of a watch or other wearable."]
    Wearable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "iOS configuration that can be selected at the time a test is run."]
pub struct IosRuntimeConfiguration {
    #[serde(rename = "locales")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of available locales."]
    pub locales: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Locale>>>,
    #[serde(rename = "orientations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of available orientations."]
    pub orientations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Orientation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A test of an iOS application that implements one or more game loop scenarios. This test type accepts an archived application (.ipa file) and a list of integer scenarios that will be executed on the app sequentially."]
pub struct IosTestLoop {
    #[serde(rename = "appBundleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The bundle id for the application under test."]
    pub app_bundle_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appIpa")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The .ipa of the application to test."]
    pub app_ipa: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "scenarios")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of scenarios that should be run during the test. Defaults to the single scenario 0 if unspecified."]
    pub scenarios: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of how to set up an iOS device prior to running the test."]
pub struct IosTestSetup {
    #[serde(rename = "additionalIpas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "iOS apps to install in addition to those being directly tested."]
    pub additional_ipas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileReference>>>,
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
    pub network_profile: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pullDirectories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of directories on the device to upload to Cloud Storage at the end of the test. Directories should either be in a shared directory (e.g. /private/var/mobile/Media) or within an accessible directory inside the app's filesystem (e.g. /Documents) by specifying the bundle id."]
    pub pull_directories: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosDeviceFile>>>,
    #[serde(rename = "pushFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of files to push to the device before starting the test."]
    pub push_files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IosDeviceFile>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An iOS version."]
pub struct IosVersion {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque id for this iOS version. Use this id to invoke the TestExecutionService."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "majorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer representing the major iOS version. Examples: \"8\", \"9\"."]
    pub major_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer representing the minor iOS version. Examples: \"1\", \"2\"."]
    pub minor_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "supportedXcodeVersionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The available Xcode versions for this version."]
    pub supported_xcode_version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A test of an iOS application that uses the XCTest framework. Xcode supports the option to \"build for testing\", which generates an .xctestrun file that contains a test specification (arguments, test methods, etc). This test type accepts a zip file containing the .xctestrun file and the corresponding contents of the Build/Products directory that contains all the binaries needed to run the tests."]
pub struct IosXcTest {
    #[serde(rename = "appBundleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The bundle id for the application under test."]
    pub app_bundle_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testSpecialEntitlements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The option to test special app entitlements. Setting this would re-sign the app having special entitlements with an explicit application-identifier. Currently supports testing aps-environment entitlement."]
    pub test_special_entitlements: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "testsZip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The .zip containing the .xctestrun file and the contents of the DerivedData/Build/Products directory. The .xctestrun file in this zip is ignored if the xctestrun field is specified."]
    pub tests_zip: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "xcodeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Xcode version that should be used for the test. Use the TestEnvironmentDiscoveryService to get supported options. Defaults to the latest Xcode version Firebase Test Lab supports."]
    pub xcode_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "xctestrun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An .xctestrun file that will override the .xctestrun file in the tests zip. Because the .xctestrun file contains environment variables along with test methods to run and/or ignore, this can be useful for sharding tests. Default is taken from the tests zip."]
    pub xctestrun: ::std::option::Option<::std::boxed::Box<FileReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an intent that starts the main launcher activity."]
pub struct LauncherActivityIntent {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A location/region designation for language."]
pub struct Locale {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id for this locale. Example: \"en_US\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-friendly name for this language/locale. Example: \"English\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-friendly string representing the region for this locale. Example: \"United States\". Not present for every locale."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Example: \"default\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shards test cases into the specified groups of packages, classes, and/or methods. With manual sharding enabled, specifying test targets via environment_variables or in InstrumentationTest is invalid."]
pub struct ManualSharding {
    #[serde(rename = "testTargetsForShard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Group of packages, classes, and/or test methods to be run for each shard. When any physical devices are selected, the number of test_targets_for_shard must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500."]
    pub test_targets_for_shard:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestTargetsForShard>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkConfiguration {
    #[serde(rename = "downRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The emulation rule applying to the download traffic."]
    pub down_rule: ::std::option::Option<::std::boxed::Box<TrafficRule>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique opaque id for this network traffic configuration."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "upRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The emulation rule applying to the upload traffic."]
    pub up_rule: ::std::option::Option<::std::boxed::Box<TrafficRule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkConfigurationCatalog {
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub configurations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NetworkConfiguration>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An opaque binary blob file to install on the device before the test starts."]
pub struct ObbFile {
    #[serde(rename = "obb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Opaque Binary Blob (OBB) file(s) to install on the device."]
    pub obb: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "obbFileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. OBB file name which must conform to the format as specified by Android e.g. [main|patch].0300110.com.example.android.obb which will be installed into \\/Android/obb/\\/ on the device."]
    pub obb_file_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Screen orientation of the device."]
pub struct Orientation {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id for this orientation. Example: \"portrait\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-friendly name for this orientation. Example: \"portrait\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this dimension. Example: \"default\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The currently provided software environment on the devices under test."]
pub struct ProvidedSoftwareCatalog {
    #[serde(rename = "androidxOrchestratorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string representing the current version of AndroidX Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#androidx.test:orchestrator."]
    pub androidx_orchestrator_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orchestratorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string representing the current version of Android Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#com.android.support.test:orchestrator."]
    pub orchestrator_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A file or directory to install on the device before the test starts."]
pub struct RegularFile {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The source file."]
    pub content: ::std::option::Option<::std::boxed::Box<FileReference>>,
    #[serde(rename = "devicePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Where to put the content on the device. Must be an absolute, allowlisted path. If the file exists, it will be replaced. The following device-side directories and any of their subdirectories are allowlisted: ${EXTERNAL_STORAGE}, /sdcard, or /storage ${ANDROID_DATA}/local/tmp, or /data/local/tmp Specifying a path outside of these directory trees is invalid. The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device and copy the file there. It is strongly advised to use the Environment API in app and test code to access files on the device in a portable way."]
    pub device_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Locations where the results of running the test are stored."]
pub struct ResultStorage {
    #[serde(rename = "googleCloudStorage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required."]
    pub google_cloud_storage: ::std::option::Option<::std::boxed::Box<GoogleCloudStorage>>,
    #[serde(rename = "resultsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL to the results in the Firebase Web Console."]
    pub results_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toolResultsExecution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The tool results execution that results are written to."]
    pub tool_results_execution: ::std::option::Option<::std::boxed::Box<ToolResultsExecution>>,
    #[serde(rename = "toolResultsHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tool results history that contains the tool results execution that results are written to. If not provided, the service will choose an appropriate value."]
    pub tool_results_history: ::std::option::Option<::std::boxed::Box<ToolResultsHistory>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Directs Robo to interact with a specific UI element if it is encountered during the crawl. Currently, Robo can perform text entry or element click."]
pub struct RoboDirective {
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of action that Robo should perform on the specified element."]
    pub action_type: ::std::option::Option<RoboDirectiveActionTypeEnum>,
    #[serde(rename = "inputText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text that Robo is directed to set. If left empty, the directive will be treated as a CLICK on the element matching the resource_name."]
    pub input_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The android resource name of the target UI element. For example, in Java: R.string.foo in xml: @string/foo Only the \"foo\" part is needed. Reference doc: https://developer.android.com/guide/topics/resources/accessing-resources.html"]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of action that Robo should perform on the specified element."]
pub enum RoboDirectiveActionTypeEnum {
    #[serde(rename = "ACTION_TYPE_UNSPECIFIED")]
    #[doc = "DO NOT USE. For proto versioning only."]
    ActionTypeUnspecified,
    #[serde(rename = "SINGLE_CLICK")]
    #[doc = "Direct Robo to click on the specified element. No-op if specified element is not clickable."]
    SingleClick,
    #[serde(rename = "ENTER_TEXT")]
    #[doc = "Direct Robo to enter text on the specified element. No-op if specified element is not enabled or does not allow text entry."]
    EnterText,
    #[serde(rename = "IGNORE")]
    #[doc = "Direct Robo to ignore interactions with a specific element."]
    Ignore,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for specifying the start activities to crawl."]
pub struct RoboStartingIntent {
    #[serde(rename = "launcherActivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An intent that starts the main launcher activity."]
    pub launcher_activity: ::std::option::Option<::std::boxed::Box<LauncherActivityIntent>>,
    #[serde(rename = "startActivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An intent that starts an activity with specific details."]
    pub start_activity: ::std::option::Option<::std::boxed::Box<StartActivityIntent>>,
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timeout in seconds for each intent."]
    pub timeout: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output only. Details about the shard."]
pub struct Shard {
    #[serde(rename = "numShards")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The total number of shards."]
    pub num_shards: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "shardIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The index of the shard among all the shards."]
    pub shard_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "testTargetsForShard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Test targets for each shard."]
    pub test_targets_for_shard: ::std::option::Option<::std::boxed::Box<TestTargetsForShard>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for enabling sharding."]
pub struct ShardingOption {
    #[serde(rename = "manualSharding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shards test cases into the specified groups of packages, classes, and/or methods."]
    pub manual_sharding: ::std::option::Option<::std::boxed::Box<ManualSharding>>,
    #[serde(rename = "uniformSharding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniformly shards test cases given a total number of shards."]
    pub uniform_sharding: ::std::option::Option<::std::boxed::Box<UniformSharding>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A starting intent specified by an action, uri, and categories."]
pub struct StartActivityIntent {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Action name. Required for START_ACTIVITY."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Intent categories to set on the intent."]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI for the action."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystraceSetup {
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Systrace duration in seconds. Should be between 1 and 30 seconds. 0 disables systrace."]
    pub duration_seconds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional details about the progress of the running test."]
pub struct TestDetails {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the TestState is ERROR, then this string will contain human-readable details about the error."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Human-readable, detailed descriptions of the test's progress. For example: \"Provisioning a device\", \"Starting Test\". During the course of execution new data may be appended to the end of progress_messages."]
    pub progress_messages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of a test environment."]
pub struct TestEnvironmentCatalog {
    #[serde(rename = "androidDeviceCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported Android devices."]
    pub android_device_catalog: ::std::option::Option<::std::boxed::Box<AndroidDeviceCatalog>>,
    #[serde(rename = "deviceIpBlockCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP blocks used by devices in the test environment."]
    pub device_ip_block_catalog: ::std::option::Option<::std::boxed::Box<DeviceIpBlockCatalog>>,
    #[serde(rename = "iosDeviceCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported iOS devices."]
    pub ios_device_catalog: ::std::option::Option<::std::boxed::Box<IosDeviceCatalog>>,
    #[serde(rename = "networkConfigurationCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported network configurations."]
    pub network_configuration_catalog:
        ::std::option::Option<::std::boxed::Box<NetworkConfigurationCatalog>>,
    #[serde(rename = "softwareCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The software test environment provided by TestExecutionService."]
    pub software_catalog: ::std::option::Option<::std::boxed::Box<ProvidedSoftwareCatalog>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single test executed in a single environment."]
pub struct TestExecution {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. How the host machine(s) are configured."]
    pub environment: ::std::option::Option<::std::boxed::Box<Environment>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Unique id set by the service."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matrixId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Id of the containing TestMatrix."]
    pub matrix_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The cloud project that owns the test execution."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Details about the shard."]
    pub shard: ::std::option::Option<::std::boxed::Box<Shard>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the current progress of the test execution (e.g., FINISHED)."]
    pub state: ::std::option::Option<TestExecutionStateEnum>,
    #[serde(rename = "testDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional details about the running test."]
    pub test_details: ::std::option::Option<::std::boxed::Box<TestDetails>>,
    #[serde(rename = "testSpecification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. How to run the test."]
    pub test_specification: ::std::option::Option<::std::boxed::Box<TestSpecification>>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this test execution was initially created."]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toolResultsStep")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Where the results for this execution are written."]
    pub tool_results_step: ::std::option::Option<::std::boxed::Box<ToolResultsStep>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Indicates the current progress of the test execution (e.g., FINISHED)."]
pub enum TestExecutionStateEnum {
    #[serde(rename = "TEST_STATE_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    TestStateUnspecified,
    #[serde(rename = "VALIDATING")]
    #[doc = "The execution or matrix is being validated."]
    Validating,
    #[serde(rename = "PENDING")]
    #[doc = "The execution or matrix is waiting for resources to become available."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The execution is currently being processed. Can only be set on an execution."]
    Running,
    #[serde(rename = "FINISHED")]
    #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
    Finished,
    #[serde(rename = "ERROR")]
    #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
    Error,
    #[serde(rename = "UNSUPPORTED_ENVIRONMENT")]
    #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
    UnsupportedEnvironment,
    #[serde(rename = "INCOMPATIBLE_ENVIRONMENT")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
    IncompatibleEnvironment,
    #[serde(rename = "INCOMPATIBLE_ARCHITECTURE")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
    IncompatibleArchitecture,
    #[serde(rename = "CANCELLED")]
    #[doc = "The user cancelled the execution. Can only be set on an execution."]
    Cancelled,
    #[serde(rename = "INVALID")]
    #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
    Invalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "TestMatrix captures all details about a test. It contains the environment configuration, test specification, test executions and overall state and outcome."]
pub struct TestMatrix {
    #[serde(rename = "clientInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the client which invoked the test."]
    pub client_info: ::std::option::Option<::std::boxed::Box<ClientInfo>>,
    #[serde(rename = "environmentMatrix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The devices the tests are being executed on."]
    pub environment_matrix: ::std::option::Option<::std::boxed::Box<EnvironmentMatrix>>,
    #[serde(rename = "failFast")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation."]
    pub fail_fast: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "flakyTestAttempts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns."]
    pub flaky_test_attempts: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "invalidMatrixDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state."]
    pub invalid_matrix_details: ::std::option::Option<TestMatrixInvalidMatrixDetailsEnum>,
    #[serde(rename = "outcomeSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED."]
    pub outcome_summary: ::std::option::Option<TestMatrixOutcomeSummaryEnum>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cloud project that owns the test matrix."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resultStorage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Where the results for the matrix are written."]
    pub result_storage: ::std::option::Option<::std::boxed::Box<ResultStorage>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the current progress of the test matrix."]
    pub state: ::std::option::Option<TestMatrixStateEnum>,
    #[serde(rename = "testExecutions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The list of test executions that the service creates for this matrix."]
    pub test_executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestExecution>>>,
    #[serde(rename = "testMatrixId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Unique id set by the service."]
    pub test_matrix_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testSpecification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. How to run the test."]
    pub test_specification: ::std::option::Option<::std::boxed::Box<TestSpecification>>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this test matrix was initially created."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state."]
pub enum TestMatrixInvalidMatrixDetailsEnum {
    #[serde(rename = "INVALID_MATRIX_DETAILS_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    InvalidMatrixDetailsUnspecified,
    #[serde(rename = "DETAILS_UNAVAILABLE")]
    #[doc = "The matrix is INVALID, but there are no further details available."]
    DetailsUnavailable,
    #[serde(rename = "MALFORMED_APK")]
    #[doc = "The input app APK could not be parsed."]
    MalformedApk,
    #[serde(rename = "MALFORMED_TEST_APK")]
    #[doc = "The input test APK could not be parsed."]
    MalformedTestApk,
    #[serde(rename = "NO_MANIFEST")]
    #[doc = "The AndroidManifest.xml could not be found."]
    NoManifest,
    #[serde(rename = "NO_PACKAGE_NAME")]
    #[doc = "The APK manifest does not declare a package name."]
    NoPackageName,
    #[serde(rename = "INVALID_PACKAGE_NAME")]
    #[doc = "The APK application ID (aka package name) is invalid. See also https://developer.android.com/studio/build/application-id"]
    InvalidPackageName,
    #[serde(rename = "TEST_SAME_AS_APP")]
    #[doc = "The test package and app package are the same."]
    TestSameAsApp,
    #[serde(rename = "NO_INSTRUMENTATION")]
    #[doc = "The test apk does not declare an instrumentation."]
    NoInstrumentation,
    #[serde(rename = "NO_SIGNATURE")]
    #[doc = "The input app apk does not have a signature."]
    NoSignature,
    #[serde(rename = "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE")]
    #[doc = "The test runner class specified by user or in the test APK's manifest file is not compatible with Android Test Orchestrator. Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or higher. Orchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR OrchestratorOption."]
    InstrumentationOrchestratorIncompatible,
    #[serde(rename = "NO_TEST_RUNNER_CLASS")]
    #[doc = "The test APK does not contain the test runner class specified by user or in the manifest file. This can be caused by either of the following reasons: - the user provided a runner class name that's incorrect, or - the test runner isn't built into the test APK (might be in the app APK instead)."]
    NoTestRunnerClass,
    #[serde(rename = "NO_LAUNCHER_ACTIVITY")]
    #[doc = "A main launcher activity could not be found."]
    NoLauncherActivity,
    #[serde(rename = "FORBIDDEN_PERMISSIONS")]
    #[doc = "The app declares one or more permissions that are not allowed."]
    ForbiddenPermissions,
    #[serde(rename = "INVALID_ROBO_DIRECTIVES")]
    #[doc = "There is a conflict in the provided robo_directives."]
    InvalidRoboDirectives,
    #[serde(rename = "INVALID_RESOURCE_NAME")]
    #[doc = "There is at least one invalid resource name in the provided robo directives"]
    InvalidResourceName,
    #[serde(rename = "INVALID_DIRECTIVE_ACTION")]
    #[doc = "Invalid definition of action in the robo directives (e.g. a click or ignore action includes an input text field)"]
    InvalidDirectiveAction,
    #[serde(rename = "TEST_LOOP_INTENT_FILTER_NOT_FOUND")]
    #[doc = "There is no test loop intent filter, or the one that is given is not formatted correctly."]
    TestLoopIntentFilterNotFound,
    #[serde(rename = "SCENARIO_LABEL_NOT_DECLARED")]
    #[doc = "The request contains a scenario label that was not declared in the manifest."]
    ScenarioLabelNotDeclared,
    #[serde(rename = "SCENARIO_LABEL_MALFORMED")]
    #[doc = "There was an error when parsing a label's value."]
    ScenarioLabelMalformed,
    #[serde(rename = "SCENARIO_NOT_DECLARED")]
    #[doc = "The request contains a scenario number that was not declared in the manifest."]
    ScenarioNotDeclared,
    #[serde(rename = "DEVICE_ADMIN_RECEIVER")]
    #[doc = "Device administrator applications are not allowed."]
    DeviceAdminReceiver,
    #[serde(rename = "MALFORMED_XC_TEST_ZIP")]
    #[doc = "The zipped XCTest was malformed. The zip did not contain a single .xctestrun file and the contents of the DerivedData/Build/Products directory."]
    MalformedXcTestZip,
    #[serde(rename = "BUILT_FOR_IOS_SIMULATOR")]
    #[doc = "The zipped XCTest was built for the iOS simulator rather than for a physical device."]
    BuiltForIosSimulator,
    #[serde(rename = "NO_TESTS_IN_XC_TEST_ZIP")]
    #[doc = "The .xctestrun file did not specify any test targets."]
    NoTestsInXcTestZip,
    #[serde(rename = "USE_DESTINATION_ARTIFACTS")]
    #[doc = "One or more of the test targets defined in the .xctestrun file specifies \"UseDestinationArtifacts\", which is disallowed."]
    UseDestinationArtifacts,
    #[serde(rename = "TEST_NOT_APP_HOSTED")]
    #[doc = "XC tests which run on physical devices must have \"IsAppHostedTestBundle\" == \"true\" in the xctestrun file."]
    TestNotAppHosted,
    #[serde(rename = "PLIST_CANNOT_BE_PARSED")]
    #[doc = "An Info.plist file in the XCTest zip could not be parsed."]
    PlistCannotBeParsed,
    #[serde(rename = "TEST_ONLY_APK")]
    #[doc = "The APK is marked as \"testOnly\". Deprecated and not currently used."]
    TestOnlyApk,
    #[serde(rename = "MALFORMED_IPA")]
    #[doc = "The input IPA could not be parsed."]
    MalformedIpa,
    #[serde(rename = "MISSING_URL_SCHEME")]
    #[doc = "The application doesn't register the game loop URL scheme."]
    MissingUrlScheme,
    #[serde(rename = "MALFORMED_APP_BUNDLE")]
    #[doc = "The iOS application bundle (.app) couldn't be processed."]
    MalformedAppBundle,
    #[serde(rename = "NO_CODE_APK")]
    #[doc = "APK contains no code. See also https://developer.android.com/guide/topics/manifest/application-element.html#code"]
    NoCodeApk,
    #[serde(rename = "INVALID_INPUT_APK")]
    #[doc = "Either the provided input APK path was malformed, the APK file does not exist, or the user does not have permission to access the APK file."]
    InvalidInputApk,
    #[serde(rename = "INVALID_APK_PREVIEW_SDK")]
    #[doc = "APK is built for a preview SDK which is unsupported"]
    InvalidApkPreviewSdk,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED."]
pub enum TestMatrixOutcomeSummaryEnum {
    #[serde(rename = "OUTCOME_SUMMARY_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    OutcomeSummaryUnspecified,
    #[serde(rename = "SUCCESS")]
    #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
    Success,
    #[serde(rename = "FAILURE")]
    #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
    Failure,
    #[serde(rename = "INCONCLUSIVE")]
    #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
    Inconclusive,
    #[serde(rename = "SKIPPED")]
    #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
    Skipped,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Indicates the current progress of the test matrix."]
pub enum TestMatrixStateEnum {
    #[serde(rename = "TEST_STATE_UNSPECIFIED")]
    #[doc = "Do not use. For proto versioning only."]
    TestStateUnspecified,
    #[serde(rename = "VALIDATING")]
    #[doc = "The execution or matrix is being validated."]
    Validating,
    #[serde(rename = "PENDING")]
    #[doc = "The execution or matrix is waiting for resources to become available."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The execution is currently being processed. Can only be set on an execution."]
    Running,
    #[serde(rename = "FINISHED")]
    #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
    Finished,
    #[serde(rename = "ERROR")]
    #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
    Error,
    #[serde(rename = "UNSUPPORTED_ENVIRONMENT")]
    #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
    UnsupportedEnvironment,
    #[serde(rename = "INCOMPATIBLE_ENVIRONMENT")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
    IncompatibleEnvironment,
    #[serde(rename = "INCOMPATIBLE_ARCHITECTURE")]
    #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
    IncompatibleArchitecture,
    #[serde(rename = "CANCELLED")]
    #[doc = "The user cancelled the execution. Can only be set on an execution."]
    Cancelled,
    #[serde(rename = "INVALID")]
    #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
    Invalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of how to set up the Android device prior to running the test."]
pub struct TestSetup {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device will be logged in on this account for the duration of the test."]
    pub account: ::std::option::Option<::std::boxed::Box<Account>>,
    #[serde(rename = "additionalApks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "APKs to install in addition to those being directly tested. Currently capped at 100."]
    pub additional_apks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Apk>>>,
    #[serde(rename = "directoriesToPull")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of directories on the device to upload to GCS at the end of the test; they must be absolute paths under /sdcard, /storage or /data/local/tmp. Path names are restricted to characters a-z A-Z 0-9 _ - . + and / Note: The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device."]
    pub directories_to_pull: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dontAutograntPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to prevent all runtime permissions to be granted at app install"]
    pub dont_autogrant_permissions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment variables to set for the test (only applicable for instrumentation tests)."]
    pub environment_variables:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnvironmentVariable>>>,
    #[serde(rename = "filesToPush")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of files to push to the device before starting the test."]
    pub files_to_push: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceFile>>>,
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
    pub network_profile: ::std::option::Option<::std::string::String>,
    #[serde(rename = "systrace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Systrace configuration for the run. If set a systrace will be taken, starting on test start and lasting for the configured duration. The systrace file thus obtained is put in the results bucket together with the other artifacts from the run."]
    pub systrace: ::std::option::Option<::std::boxed::Box<SystraceSetup>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of how to run the test."]
pub struct TestSpecification {
    #[serde(rename = "androidInstrumentationTest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Android instrumentation test."]
    pub android_instrumentation_test:
        ::std::option::Option<::std::boxed::Box<AndroidInstrumentationTest>>,
    #[serde(rename = "androidRoboTest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Android robo test."]
    pub android_robo_test: ::std::option::Option<::std::boxed::Box<AndroidRoboTest>>,
    #[serde(rename = "androidTestLoop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Android Application with a Test Loop."]
    pub android_test_loop: ::std::option::Option<::std::boxed::Box<AndroidTestLoop>>,
    #[serde(rename = "disablePerformanceMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Disables performance metrics recording. May reduce test latency."]
    pub disable_performance_metrics: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "disableVideoRecording")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Disables video recording. May reduce test latency."]
    pub disable_video_recording: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "iosTestLoop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An iOS application with a test loop."]
    pub ios_test_loop: ::std::option::Option<::std::boxed::Box<IosTestLoop>>,
    #[serde(rename = "iosTestSetup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Test setup requirements for iOS."]
    pub ios_test_setup: ::std::option::Option<::std::boxed::Box<IosTestSetup>>,
    #[serde(rename = "iosXcTest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An iOS XCTest, via an .xctestrun file."]
    pub ios_xc_test: ::std::option::Option<::std::boxed::Box<IosXcTest>>,
    #[serde(rename = "testSetup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Test setup requirements for Android e.g. files to install, bootstrap scripts."]
    pub test_setup: ::std::option::Option<::std::boxed::Box<TestSetup>>,
    #[serde(rename = "testTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max time a test execution is allowed to run before it is automatically cancelled. The default value is 5 min."]
    pub test_timeout: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Test targets for a shard."]
pub struct TestTargetsForShard {
    #[serde(rename = "testTargets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Group of packages, classes, and/or test methods to be run for each shard. The targets need to be specified in AndroidJUnitRunner argument format. For example, \"package com.my.packages\" \"class com.my.package.MyClass\". The number of shard_test_targets must be greater than 0."]
    pub test_targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a tool results execution resource. This has the results of a TestMatrix."]
pub struct ToolResultsExecution {
    #[serde(rename = "executionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A tool results execution ID."]
    pub execution_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A tool results history ID."]
    pub history_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The cloud project that owns the tool results execution."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a tool results history resource."]
pub struct ToolResultsHistory {
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A tool results history ID."]
    pub history_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cloud project that owns the tool results history."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a tool results step resource. This has the results of a TestExecution."]
pub struct ToolResultsStep {
    #[serde(rename = "executionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A tool results execution ID."]
    pub execution_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A tool results history ID."]
    pub history_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The cloud project that owns the tool results step."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stepId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A tool results step ID."]
    pub step_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Network emulation parameters."]
pub struct TrafficRule {
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bandwidth in kbits/second."]
    pub bandwidth: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "burst")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Burst size in kbits."]
    pub burst: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "delay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Packet delay, must be >= 0."]
    pub delay: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packetDuplicationRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Packet duplication ratio (0.0 - 1.0)."]
    pub packet_duplication_ratio: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "packetLossRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Packet loss ratio (0.0 - 1.0)."]
    pub packet_loss_ratio: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Uniformly shards test cases given a total number of shards. For Instrumentation test, it will be translated to \"-e numShard\" \"-e shardIndex\" AndroidJUnitRunner arguments. With uniform sharding enabled, specifying these sharding arguments via environment_variables is invalid."]
pub struct UniformSharding {
    #[serde(rename = "numShards")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Total number of shards. When any physical devices are selected, the number must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500."]
    pub num_shards: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Xcode version that an iOS version is compatible with."]
pub struct XcodeVersion {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags for this Xcode version. Example: \"default\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id for this version. Example: \"9.2\"."]
    pub version: ::std::option::Option<::std::string::String>,
}
