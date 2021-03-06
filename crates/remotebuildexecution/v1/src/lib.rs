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
    pub mod operations {
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
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
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
    #[doc = "An `Action` captures all the information about an execution which is required to reproduce it. `Action`s are the core component of the [Execution] service. A single `Action` represents a repeatable action that can be performed by the execution service. `Action`s can be succinctly identified by the digest of their wire format encoding and, once an `Action` has been executed, will be cached in the action cache. Future requests can then use the cached result rather than needing to run afresh. When a server completes execution of an Action, it MAY choose to cache the result in the ActionCache unless `do_not_cache` is `true`. Clients SHOULD expect the server to do so. By default, future calls to Execute the same `Action` will also serve their results from the cache. Clients must take care to understand the caching behaviour. Ideally, all `Action`s will be reproducible so that serving a result from cache is always desirable and correct."]
    pub struct BuildBazelRemoteExecutionV2Action {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the Command to run, which MUST be present in the ContentAddressableStorage."]
        pub command_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doNotCache")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, then the `Action`'s result cannot be cached, and in-flight requests for the same `Action` may not be merged."]
        pub do_not_cache: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputRootDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the root Directory for the input files. The files in the directory tree are available in the correct location on the build machine before the command is executed. The root directory, as well as every subdirectory and content blob referred to, MUST be in the ContentAddressableStorage."]
        pub input_root_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputNodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of required supported NodeProperty keys. In order to ensure that equivalent `Action`s always hash to the same value, the supported node properties MUST be lexicographically sorted by name. Sorting of strings is done by code point, equivalently, by the UTF-8 bytes. The interpretation of these properties is server-dependent. If a property is not recognized by the server, the server will return an `INVALID_ARGUMENT` error."]
        pub output_node_properties: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A timeout after which the execution should be killed. If the timeout is absent, then the client is specifying that the execution should continue as long as the server will let it. The server SHOULD impose a timeout if the client does not specify one, however, if the client does specify a timeout that is longer than the server's maximum timeout, the server MUST reject the request. The timeout is a part of the Action message, and therefore two `Actions` with different timeouts are different, even if they are otherwise identical. This is because, if they were not, running an `Action` with a lower timeout than is required might result in a cache hit from an execution run with a longer timeout, hiding the fact that the timeout is too short. By encoding it directly in the `Action`, a lower timeout will result in a cache miss and the execution timeout will fail immediately, rather than whenever the cache entry gets evicted."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2Action {
        pub fn builder() -> BuildBazelRemoteExecutionV2ActionBuilder {
            BuildBazelRemoteExecutionV2ActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An ActionResult represents the result of an Action being run."]
    pub struct BuildBazelRemoteExecutionV2ActionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the execution that originally produced this result."]
        pub execution_metadata: ::std::option::Option<
            ::std::boxed::Box<BuildBazelRemoteExecutionV2ExecutedActionMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exit code of the command."]
        pub exit_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputDirectories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output directories of the action. For each output directory requested in the `output_directories` or `output_paths` field of the Action, if the corresponding directory existed after the action completed, a single entry will be present in the output list, which will contain the digest of a Tree message containing the directory tree, and the path equal exactly to the corresponding Action output_directories member. As an example, suppose the Action had an output directory `a/b/dir` and the execution produced the following contents in `a/b/dir`: a file named `bar` and a directory named `foo` with an executable file named `baz`. Then, output_directory will contain (hashes shortened for readability): ```json // OutputDirectory proto: { path: \"a/b/dir\" tree_digest: { hash: \"4a73bc9d03...\", size: 55 } } // Tree proto with hash \"4a73bc9d03...\" and size 55: { root: { files: [ { name: \"bar\", digest: { hash: \"4a73bc9d03...\", size: 65534 } } ], directories: [ { name: \"foo\", digest: { hash: \"4cf2eda940...\", size: 43 } } ] } children : { // (Directory proto with hash \"4cf2eda940...\" and size 43) files: [ { name: \"baz\", digest: { hash: \"b2c941073e...\", size: 1294, }, is_executable: true } ] } } ``` If an output of the same name as listed in `output_files` of the Command was found in `output_directories`, but was not a directory, the server will return a FAILED_PRECONDITION."]
        pub output_directories: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2OutputDirectory>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputDirectorySymlinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output directories of the action that are symbolic links to other directories. Those may be links to other output directories, or input directories, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output directory requested in the `output_directories` field of the Action, if the directory existed after the action completed, a single entry will be present either in this field, or in the `output_directories` field, if the directory was not a symbolic link. If an output of the same name was found, but was a symbolic link to a file instead of a directory, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`."]
        pub output_directory_symlinks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2OutputSymlink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFileSymlinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output files of the action that are symbolic links to other files. Those may be links to other output files, or input files, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or in the `output_files` field, if the file was not a symbolic link. If an output symbolic link of the same name as listed in `output_files` of the Command was found, but its target type was not a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`."]
        pub output_file_symlinks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2OutputSymlink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output files of the action. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or the `output_file_symlinks` field if the file was a symbolic link to another file (`output_symlinks` field after v2.1). If an output listed in `output_files` was found, but was a directory rather than a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted."]
        pub output_files: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2OutputFile>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputSymlinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New in v2.1: this field will only be populated if the command `output_paths` field was used, and not the pre v2.1 `output_files` or `output_directories` fields. The output paths of the action that are symbolic links to other paths. Those may be links to other outputs, or inputs, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. A single entry for each output requested in `output_paths` field of the Action, if the corresponding path existed after the action completed and was a symbolic link. If the action does not produce a requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted."]
        pub output_symlinks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2OutputSymlink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stderrDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest for a blob containing the standard error of the action, which can be retrieved from the ContentAddressableStorage."]
        pub stderr_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stderrRaw")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard error buffer of the action. The server SHOULD NOT inline stderr unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits."]
        pub stderr_raw: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stdoutDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest for a blob containing the standard output of the action, which can be retrieved from the ContentAddressableStorage."]
        pub stdout_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stdoutRaw")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard output buffer of the action. The server SHOULD NOT inline stdout unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits."]
        pub stdout_raw: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2ActionResult {
        pub fn builder() -> BuildBazelRemoteExecutionV2ActionResultBuilder {
            BuildBazelRemoteExecutionV2ActionResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Command` is the actual command executed by a worker running an Action and specifications of its environment. Except as otherwise required, the environment (such as which system libraries or binaries are available, and what filesystems are mounted where) is defined by and specific to the implementation of the remote execution API."]
    pub struct BuildBazelRemoteExecutionV2Command {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The arguments to the command. The first argument must be the path to the executable, which must be either a relative path, in which case it is evaluated with respect to the input root, or an absolute path."]
        pub arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment variables to set when running the program. The worker may provide its own default environment variables; these defaults can be overridden using this field. Additional variables can also be specified. In order to ensure that equivalent Commands always hash to the same value, the environment variables MUST be lexicographically sorted by name. Sorting of strings is done by code point, equivalently, by the UTF-8 bytes."]
        pub environment_variables: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<BuildBazelRemoteExecutionV2CommandEnvironmentVariable>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputDirectories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the output directories that the client expects to retrieve from the action. Only the listed directories will be returned (an entire directory structure will be returned as a Tree message digest, see OutputDirectory), as well as files listed in `output_files`. Other files or directories that may be created during command execution are discarded. The paths are relative to the working directory of the action execution. The paths are specified using a single forward slash (`/`) as a path separator, even if the execution platform natively uses a different separator. The path MUST NOT include a trailing slash, nor a leading slash, being a relative path. The special value of empty string is allowed, although not recommended, and can be used to capture the entire working directory tree, including inputs. In order to ensure consistent hashing of the same Action, the output paths MUST be sorted lexicographically by code point (or, equivalently, by UTF-8 bytes). An output directory cannot be duplicated or have the same path as any of the listed output files. An output directory is allowed to be a parent of another output directory. Directories leading up to the output directories (but not the output directories themselves) are created by the worker prior to execution, even if they are not explicitly part of the input root. DEPRECATED since 2.1: Use `output_paths` instead."]
        pub output_directories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the output files that the client expects to retrieve from the action. Only the listed files, as well as directories listed in `output_directories`, will be returned to the client as output. Other files or directories that may be created during command execution are discarded. The paths are relative to the working directory of the action execution. The paths are specified using a single forward slash (`/`) as a path separator, even if the execution platform natively uses a different separator. The path MUST NOT include a trailing slash, nor a leading slash, being a relative path. In order to ensure consistent hashing of the same Action, the output paths MUST be sorted lexicographically by code point (or, equivalently, by UTF-8 bytes). An output file cannot be duplicated, be a parent of another output file, or have the same path as any of the listed output directories. Directories leading up to the output files are created by the worker prior to execution, even if they are not explicitly part of the input root. DEPRECATED since v2.1: Use `output_paths` instead."]
        pub output_files: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputPaths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the output paths that the client expects to retrieve from the action. Only the listed paths will be returned to the client as output. The type of the output (file or directory) is not specified, and will be determined by the server after action execution. If the resulting path is a file, it will be returned in an OutputFile) typed field. If the path is a directory, the entire directory structure will be returned as a Tree message digest, see OutputDirectory) Other files or directories that may be created during command execution are discarded. The paths are relative to the working directory of the action execution. The paths are specified using a single forward slash (`/`) as a path separator, even if the execution platform natively uses a different separator. The path MUST NOT include a trailing slash, nor a leading slash, being a relative path. In order to ensure consistent hashing of the same Action, the output paths MUST be deduplicated and sorted lexicographically by code point (or, equivalently, by UTF-8 bytes). Directories leading up to the output paths are created by the worker prior to execution, even if they are not explicitly part of the input root. New in v2.1: this field supersedes the DEPRECATED `output_files` and `output_directories` fields. If `output_paths` is used, `output_files` and `output_directories` will be ignored!"]
        pub output_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform requirements for the execution environment. The server MAY choose to execute the action on any worker satisfying the requirements, so the client SHOULD ensure that running the action on any such worker will have the same result. A detailed lexicon for this can be found in the accompanying platform.md."]
        pub platform: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Platform>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workingDirectory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The working directory, relative to the input root, for the command to run in. It must be a directory which exists in the input tree. If it is left empty, then the action is run in the input root."]
        pub working_directory: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2Command {
        pub fn builder() -> BuildBazelRemoteExecutionV2CommandBuilder {
            BuildBazelRemoteExecutionV2CommandBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An `EnvironmentVariable` is one variable to set in the running program's environment."]
    pub struct BuildBazelRemoteExecutionV2CommandEnvironmentVariable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variable name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variable value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2CommandEnvironmentVariable {
        pub fn builder() -> BuildBazelRemoteExecutionV2CommandEnvironmentVariableBuilder {
            BuildBazelRemoteExecutionV2CommandEnvironmentVariableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A content digest. A digest for a given blob consists of the size of the blob and its hash. The hash algorithm to use is defined by the server. The size is considered to be an integral part of the digest and cannot be separated. That is, even if the `hash` field is correctly specified but `size_bytes` is not, the server MUST reject the request. The reason for including the size in the digest is as follows: in a great many cases, the server needs to know the size of the blob it is about to work with prior to starting an operation with it, such as flattening Merkle tree structures or streaming it to a worker. Technically, the server could implement a separate metadata store, but this results in a significantly more complicated implementation as opposed to having the client specify the size up-front (or storing the size along with the digest in every message where digests are embedded). This does mean that the API leaks some implementation details of (what we consider to be) a reasonable server implementation, but we consider this to be a worthwhile tradeoff. When a `Digest` is used to refer to a proto message, it always refers to the message in binary encoded form. To ensure consistent hashing, clients and servers MUST ensure that they serialize messages according to the following rules, even if there are alternate valid encodings for the same message: * Fields are serialized in tag order. * There are no unknown fields. * There are no duplicate fields. * Fields are serialized according to the default semantics for their type. Most protocol buffer implementations will always follow these rules when serializing, but care should be taken to avoid shortcuts. For instance, concatenating two messages to merge them may produce duplicate fields."]
    pub struct BuildBazelRemoteExecutionV2Digest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long."]
        pub hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the blob, in bytes."]
        pub size_bytes: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2Digest {
        pub fn builder() -> BuildBazelRemoteExecutionV2DigestBuilder {
            BuildBazelRemoteExecutionV2DigestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Directory` represents a directory node in a file tree, containing zero or more children FileNodes, DirectoryNodes and SymlinkNodes. Each `Node` contains its name in the directory, either the digest of its content (either a file blob or a `Directory` proto) or a symlink target, as well as possibly some metadata about the file or directory. In order to ensure that two equivalent directory trees hash to the same value, the following restrictions MUST be obeyed when constructing a a `Directory`: * Every child in the directory must have a path of exactly one segment. Multiple levels of directory hierarchy may not be collapsed. * Each child in the directory must have a unique path segment (file name). Note that while the API itself is case-sensitive, the environment where the Action is executed may or may not be case-sensitive. That is, it is legal to call the API with a Directory that has both \"Foo\" and \"foo\" as children, but the Action may be rejected by the remote system upon execution. * The files, directories and symlinks in the directory must each be sorted in lexicographical order by path. The path strings must be sorted by code point, equivalently, by UTF-8 bytes. * The NodeProperties of files, directories, and symlinks must be sorted in lexicographical order by property name. A `Directory` that obeys the restrictions is said to be in canonical form. As an example, the following could be used for a file named `bar` and a directory named `foo` with an executable file named `baz` (hashes shortened for readability): ```json // (Directory proto) { files: [ { name: \"bar\", digest: { hash: \"4a73bc9d03...\", size: 65534 }, node_properties: [ { \"name\": \"MTime\", \"value\": \"2017-01-15T01:30:15.01Z\" } ] } ], directories: [ { name: \"foo\", digest: { hash: \"4cf2eda940...\", size: 43 } } ] } // (Directory proto with hash \"4cf2eda940...\" and size 43) { files: [ { name: \"baz\", digest: { hash: \"b2c941073e...\", size: 1294, }, is_executable: true } ] } ```"]
    pub struct BuildBazelRemoteExecutionV2Directory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subdirectories in the directory."]
        pub directories: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2DirectoryNode>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The files in the directory."]
        pub files: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2FileNode>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node properties of the Directory."]
        pub node_properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2NodeProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symlinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The symlinks in the directory."]
        pub symlinks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2SymlinkNode>>,
        >,
    }
    impl BuildBazelRemoteExecutionV2Directory {
        pub fn builder() -> BuildBazelRemoteExecutionV2DirectoryBuilder {
            BuildBazelRemoteExecutionV2DirectoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `DirectoryNode` represents a child of a Directory which is itself a `Directory` and its associated metadata."]
    pub struct BuildBazelRemoteExecutionV2DirectoryNode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the Directory object represented. See Digest for information about how to take the digest of a proto message."]
        pub digest: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the directory."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2DirectoryNode {
        pub fn builder() -> BuildBazelRemoteExecutionV2DirectoryNodeBuilder {
            BuildBazelRemoteExecutionV2DirectoryNodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about an ongoing execution, which will be contained in the metadata field of the Operation."]
    pub struct BuildBazelRemoteExecutionV2ExecuteOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the Action being executed."]
        pub action_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current stage of execution."]
        pub stage:
            ::std::option::Option<BuildBazelRemoteExecutionV2ExecuteOperationMetadataStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stderrStreamName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the client can use this name with ByteStream.Read to stream the standard error."]
        pub stderr_stream_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stdoutStreamName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the client can use this name with ByteStream.Read to stream the standard output."]
        pub stdout_stream_name: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2ExecuteOperationMetadata {
        pub fn builder() -> BuildBazelRemoteExecutionV2ExecuteOperationMetadataBuilder {
            BuildBazelRemoteExecutionV2ExecuteOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current stage of execution."]
    pub enum BuildBazelRemoteExecutionV2ExecuteOperationMetadataStageEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Invalid value."]
        Unknown,
        #[serde(rename = "CACHE_CHECK")]
        #[doc = "Checking the result against the cache."]
        CacheCheck,
        #[serde(rename = "QUEUED")]
        #[doc = "Currently idle, awaiting a free machine to execute."]
        Queued,
        #[serde(rename = "EXECUTING")]
        #[doc = "Currently being executed by a worker."]
        Executing,
        #[serde(rename = "COMPLETED")]
        #[doc = "Finished execution."]
        Completed,
    }
    impl ::std::default::Default for BuildBazelRemoteExecutionV2ExecuteOperationMetadataStageEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Execution.Execute, which will be contained in the response field of the Operation."]
    pub struct BuildBazelRemoteExecutionV2ExecuteResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cachedResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the result was served from cache, false if it was executed."]
        pub cached_result: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Freeform informational message with details on the execution of the action that may be displayed to the user upon failure or when requested explicitly."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the action."]
        pub result:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2ActionResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverLogs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional list of additional log outputs the server wishes to provide. A server can use this to return execution-specific logs however it wishes. This is intended primarily to make it easier for users to debug issues that may be outside of the actual job execution, such as by identifying the worker executing the action or by providing logs from the worker's setup phase. The keys SHOULD be human readable so that a client can display them to a user."]
        pub server_logs: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<BuildBazelRemoteExecutionV2LogFile>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the status has a code other than `OK`, it indicates that the action did not finish execution. For example, if the operation times out during execution, the status will have a `DEADLINE_EXCEEDED` code. Servers MUST use this field for errors in execution, rather than the error field on the `Operation` object. If the status code is other than `OK`, then the result MUST NOT be cached. For an error status, the `result` field is optional; the server may populate the output-, stdout-, and stderr-related fields if it has any information available, such as the stdout and stderr of a timed-out action."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl BuildBazelRemoteExecutionV2ExecuteResponse {
        pub fn builder() -> BuildBazelRemoteExecutionV2ExecuteResponseBuilder {
            BuildBazelRemoteExecutionV2ExecuteResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ExecutedActionMetadata contains details about a completed execution."]
    pub struct BuildBazelRemoteExecutionV2ExecutedActionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionCompletedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker completed executing the action command."]
        pub execution_completed_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStartTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker started executing the action command."]
        pub execution_start_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputFetchCompletedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker finished fetching action inputs."]
        pub input_fetch_completed_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputFetchStartTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker started fetching action inputs."]
        pub input_fetch_start_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUploadCompletedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker finished uploading action outputs."]
        pub output_upload_completed_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUploadStartTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker started uploading action outputs."]
        pub output_upload_start_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queuedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When was the action added to the queue."]
        pub queued_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "worker")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the worker which ran the execution."]
        pub worker: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerCompletedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker completed the action, including all stages."]
        pub worker_completed_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerStartTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the worker received the action."]
        pub worker_start_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2ExecutedActionMetadata {
        pub fn builder() -> BuildBazelRemoteExecutionV2ExecutedActionMetadataBuilder {
            BuildBazelRemoteExecutionV2ExecutedActionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `FileNode` represents a single file and associated metadata."]
    pub struct BuildBazelRemoteExecutionV2FileNode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the file's content."]
        pub digest: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isExecutable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if file is executable, false otherwise."]
        pub is_executable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the file."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node properties of the FileNode."]
        pub node_properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2NodeProperty>>,
        >,
    }
    impl BuildBazelRemoteExecutionV2FileNode {
        pub fn builder() -> BuildBazelRemoteExecutionV2FileNodeBuilder {
            BuildBazelRemoteExecutionV2FileNodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `LogFile` is a log stored in the CAS."]
    pub struct BuildBazelRemoteExecutionV2LogFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the log contents."]
        pub digest: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReadable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a hint as to the purpose of the log, and is set to true if the log is human-readable text that can be usefully displayed to a user, and false otherwise. For instance, if a command-line client wishes to print the server logs to the terminal for a failed action, this allows it to avoid displaying a binary file."]
        pub human_readable: ::std::option::Option<::std::primitive::bool>,
    }
    impl BuildBazelRemoteExecutionV2LogFile {
        pub fn builder() -> BuildBazelRemoteExecutionV2LogFileBuilder {
            BuildBazelRemoteExecutionV2LogFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single property for FileNodes, DirectoryNodes, and SymlinkNodes. The server is responsible for specifying the property `name`s that it accepts. If permitted by the server, the same `name` may occur multiple times."]
    pub struct BuildBazelRemoteExecutionV2NodeProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2NodeProperty {
        pub fn builder() -> BuildBazelRemoteExecutionV2NodePropertyBuilder {
            BuildBazelRemoteExecutionV2NodePropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An `OutputDirectory` is the output in an `ActionResult` corresponding to a directory's full contents rather than a single file."]
    pub struct BuildBazelRemoteExecutionV2OutputDirectory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full path of the directory relative to the working directory. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash. The empty string value is allowed, and it denotes the entire working directory."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "treeDigest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the encoded Tree proto containing the directory's contents."]
        pub tree_digest:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
    }
    impl BuildBazelRemoteExecutionV2OutputDirectory {
        pub fn builder() -> BuildBazelRemoteExecutionV2OutputDirectoryBuilder {
            BuildBazelRemoteExecutionV2OutputDirectoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An `OutputFile` is similar to a FileNode, but it is used as an output in an `ActionResult`. It allows a full file path rather than only a name."]
    pub struct BuildBazelRemoteExecutionV2OutputFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the file if inlining was requested. The server SHOULD NOT inline file contents unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits."]
        pub contents: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the file's content."]
        pub digest: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isExecutable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if file is executable, false otherwise."]
        pub is_executable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The supported node properties of the OutputFile, if requested by the Action."]
        pub node_properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2NodeProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full path of the file relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2OutputFile {
        pub fn builder() -> BuildBazelRemoteExecutionV2OutputFileBuilder {
            BuildBazelRemoteExecutionV2OutputFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An `OutputSymlink` is similar to a Symlink, but it is used as an output in an `ActionResult`. `OutputSymlink` is binary-compatible with `SymlinkNode`."]
    pub struct BuildBazelRemoteExecutionV2OutputSymlink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The supported node properties of the OutputSymlink, if requested by the Action."]
        pub node_properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2NodeProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full path of the symlink relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. The canonical form forbids the substrings `/./` and `//` in the target path. `..` components are allowed anywhere in the target path."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2OutputSymlink {
        pub fn builder() -> BuildBazelRemoteExecutionV2OutputSymlinkBuilder {
            BuildBazelRemoteExecutionV2OutputSymlinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Platform` is a set of requirements, such as hardware, operating system, or compiler toolchain, for an Action's execution environment. A `Platform` is represented as a series of key-value pairs representing the properties that are required of the platform."]
    pub struct BuildBazelRemoteExecutionV2Platform {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties that make up this platform. In order to ensure that equivalent `Platform`s always hash to the same value, the properties MUST be lexicographically sorted by name, and then by value. Sorting of strings is done by code point, equivalently, by the UTF-8 bytes."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2PlatformProperty>>,
        >,
    }
    impl BuildBazelRemoteExecutionV2Platform {
        pub fn builder() -> BuildBazelRemoteExecutionV2PlatformBuilder {
            BuildBazelRemoteExecutionV2PlatformBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single property for the environment. The server is responsible for specifying the property `name`s that it accepts. If an unknown `name` is provided in the requirements for an Action, the server SHOULD reject the execution request. If permitted by the server, the same `name` may occur multiple times. The server is also responsible for specifying the interpretation of property `value`s. For instance, a property describing how much RAM must be available may be interpreted as allowing a worker with 16GB to fulfill a request for 8GB, while a property describing the OS environment on which the action must be performed may require an exact match with the worker's OS. The server MAY use the `value` of one or more properties to determine how it sets up the execution environment, such as by making specific system files available to the worker."]
    pub struct BuildBazelRemoteExecutionV2PlatformProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2PlatformProperty {
        pub fn builder() -> BuildBazelRemoteExecutionV2PlatformPropertyBuilder {
            BuildBazelRemoteExecutionV2PlatformPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An optional Metadata to attach to any RPC request to tell the server about an external context of the request. The server may use this for logging or other purposes. To use it, the client attaches the header to the call using the canonical proto serialization: * name: `build.bazel.remote.execution.v2.requestmetadata-bin` * contents: the base64 encoded binary `RequestMetadata` message. Note: the gRPC library serializes binary headers encoded in base 64 by default (https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md#requests). Therefore, if the gRPC library is used to pass/retrieve this metadata, the user may ignore the base64 encoding and assume it is simply serialized as a binary message."]
    pub struct BuildBazelRemoteExecutionV2RequestMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier that ties multiple requests to the same action. For example, multiple requests to the CAS, Action Cache, and Execution API are used in order to compile foo.cc."]
        pub action_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "correlatedInvocationsId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier to tie multiple tool invocations together. For example, runs of foo_test, bar_test and baz_test on a post-submit of a given patch."]
        pub correlated_invocations_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details for the tool invoking the requests."]
        pub tool_details:
            ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2ToolDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolInvocationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier that ties multiple actions together to a final result. For example, multiple actions are required to build and run foo_test."]
        pub tool_invocation_id: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2RequestMetadata {
        pub fn builder() -> BuildBazelRemoteExecutionV2RequestMetadataBuilder {
            BuildBazelRemoteExecutionV2RequestMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `SymlinkNode` represents a symbolic link."]
    pub struct BuildBazelRemoteExecutionV2SymlinkNode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the symlink."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node properties of the SymlinkNode."]
        pub node_properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2NodeProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. The canonical form forbids the substrings `/./` and `//` in the target path. `..` components are allowed anywhere in the target path."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2SymlinkNode {
        pub fn builder() -> BuildBazelRemoteExecutionV2SymlinkNodeBuilder {
            BuildBazelRemoteExecutionV2SymlinkNodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the tool used to call the API."]
    pub struct BuildBazelRemoteExecutionV2ToolDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the tool, e.g. bazel."]
        pub tool_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the tool used for the request, e.g. 5.0.3."]
        pub tool_version: ::std::option::Option<::std::string::String>,
    }
    impl BuildBazelRemoteExecutionV2ToolDetails {
        pub fn builder() -> BuildBazelRemoteExecutionV2ToolDetailsBuilder {
            BuildBazelRemoteExecutionV2ToolDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Tree` contains all the Directory protos in a single directory Merkle tree, compressed into one message."]
    pub struct BuildBazelRemoteExecutionV2Tree {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "children")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the child directories: the directories referred to by the root and, recursively, all its children. In order to reconstruct the directory tree, the client must take the digests of each of the child directories and then build up a tree starting from the `root`."]
        pub children: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BuildBazelRemoteExecutionV2Directory>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "root")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The root directory in the tree."]
        pub root: ::std::option::Option<::std::boxed::Box<BuildBazelRemoteExecutionV2Directory>>,
    }
    impl BuildBazelRemoteExecutionV2Tree {
        pub fn builder() -> BuildBazelRemoteExecutionV2TreeBuilder {
            BuildBazelRemoteExecutionV2TreeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Media resource."]
    pub struct GoogleBytestreamMedia {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the media resource."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleBytestreamMedia {
        pub fn builder() -> GoogleBytestreamMediaBuilder {
            GoogleBytestreamMediaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CommandDuration contains the various duration metrics tracked when a bot performs a command."]
    pub struct GoogleDevtoolsRemotebuildbotCommandDurations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cmWaitForAssignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent waiting for Container Manager to assign an asynchronous container for execution."]
        pub cm_wait_for_assignment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerPrep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent preparing the command to be run in a Docker container (includes pulling the Docker image, if necessary)."]
        pub docker_prep: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerPrepStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when docker preparation begins."]
        pub docker_prep_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "download")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent downloading the input files and constructing the working directory."]
        pub download: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when downloading the input files begins."]
        pub download_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "execStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when execution begins."]
        pub exec_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "execution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent executing the command (i.e., doing useful work)."]
        pub execution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isoPrepDone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when preparation is done and bot starts downloading files."]
        pub iso_prep_done: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent completing the command, in total."]
        pub overall: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stdout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent uploading the stdout logs."]
        pub stdout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spent uploading the output files."]
        pub upload: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when uploading the output files begins."]
        pub upload_start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildbotCommandDurations {
        pub fn builder() -> GoogleDevtoolsRemotebuildbotCommandDurationsBuilder {
            GoogleDevtoolsRemotebuildbotCommandDurationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CommandEvents contains counters for the number of warnings and errors that occurred during the execution of a command."]
    pub struct GoogleDevtoolsRemotebuildbotCommandEvents {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cmUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if and how Container Manager is being used for task execution."]
        pub cm_usage: ::std::option::Option<GoogleDevtoolsRemotebuildbotCommandEventsCmUsageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerCacheHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether we are using a cached Docker image (true) or had to pull the Docker image (false) for this command."]
        pub docker_cache_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerImageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Docker Image name."]
        pub docker_image_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputCacheMiss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input cache miss ratio."]
        pub input_cache_miss: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of errors reported."]
        pub num_errors: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numWarnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of warnings reported."]
        pub num_warnings: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usedAsyncContainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether an asynchronous container was used for execution."]
        pub used_async_container: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleDevtoolsRemotebuildbotCommandEvents {
        pub fn builder() -> GoogleDevtoolsRemotebuildbotCommandEventsBuilder {
            GoogleDevtoolsRemotebuildbotCommandEventsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates if and how Container Manager is being used for task execution."]
    pub enum GoogleDevtoolsRemotebuildbotCommandEventsCmUsageEnum {
        #[serde(rename = "NONE")]
        #[doc = "Container Manager is disabled or not running for this execution."]
        None,
        #[serde(rename = "CONFIG_MATCH")]
        #[doc = "Container Manager is enabled and there was a matching container available for use during execution."]
        ConfigMatch,
        #[serde(rename = "CONFIG_MISMATCH")]
        #[doc = "Container Manager is enabled, but there was no matching container available for execution."]
        ConfigMismatch,
    }
    impl ::std::default::Default for GoogleDevtoolsRemotebuildbotCommandEventsCmUsageEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The internal status of the command result."]
    pub struct GoogleDevtoolsRemotebuildbotCommandStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code."]
        pub code: ::std::option::Option<GoogleDevtoolsRemotebuildbotCommandStatusCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error message."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildbotCommandStatus {
        pub fn builder() -> GoogleDevtoolsRemotebuildbotCommandStatusBuilder {
            GoogleDevtoolsRemotebuildbotCommandStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status code."]
    pub enum GoogleDevtoolsRemotebuildbotCommandStatusCodeEnum {
        #[serde(rename = "OK")]
        #[doc = "The command succeeded."]
        Ok,
        #[serde(rename = "INVALID_ARGUMENT")]
        #[doc = "The command input was invalid."]
        InvalidArgument,
        #[serde(rename = "DEADLINE_EXCEEDED")]
        #[doc = "The command had passed its expiry time while it was still running."]
        DeadlineExceeded,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "The resources requested by the command were not found."]
        NotFound,
        #[serde(rename = "PERMISSION_DENIED")]
        #[doc = "The command failed due to permission errors."]
        PermissionDenied,
        #[serde(rename = "INTERNAL")]
        #[doc = "The command failed because of some invariants expected by the underlying system have been broken. This usually indicates a bug wit the system."]
        Internal,
        #[serde(rename = "ABORTED")]
        #[doc = "The command was aborted."]
        Aborted,
        #[serde(rename = "FAILED_PRECONDITION")]
        #[doc = "The command failed because the system is not in a state required for the command, e.g. the command inputs cannot be found on the server."]
        FailedPrecondition,
        #[serde(rename = "CLEANUP_ERROR")]
        #[doc = "The bot failed to do the cleanup, e.g. unable to delete the command working directory or the command process."]
        CleanupError,
        #[serde(rename = "DOWNLOAD_INPUTS_ERROR")]
        #[doc = "The bot failed to download the inputs."]
        DownloadInputsError,
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown error."]
        Unknown,
        #[serde(rename = "UPLOAD_OUTPUTS_ERROR")]
        #[doc = "The bot failed to upload the outputs."]
        UploadOutputsError,
        #[serde(rename = "UPLOAD_OUTPUTS_BYTES_LIMIT_EXCEEDED")]
        #[doc = "The bot tried to upload files having a total size that is too large."]
        UploadOutputsBytesLimitExceeded,
        #[serde(rename = "DOCKER_LOGIN_ERROR")]
        #[doc = "The bot failed to login to docker."]
        DockerLoginError,
        #[serde(rename = "DOCKER_IMAGE_PULL_ERROR")]
        #[doc = "The bot failed to pull docker image."]
        DockerImagePullError,
        #[serde(rename = "DOCKER_IMAGE_EXIST_ERROR")]
        #[doc = "The bot failed to check docker images."]
        DockerImageExistError,
        #[serde(rename = "DUPLICATE_INPUTS")]
        #[doc = "The inputs contain duplicate files."]
        DuplicateInputs,
        #[serde(rename = "DOCKER_IMAGE_PERMISSION_DENIED")]
        #[doc = "The bot doesn't have the permissions to pull docker images."]
        DockerImagePermissionDenied,
        #[serde(rename = "DOCKER_IMAGE_NOT_FOUND")]
        #[doc = "The docker image cannot be found."]
        DockerImageNotFound,
        #[serde(rename = "WORKING_DIR_NOT_FOUND")]
        #[doc = "Working directory is not found."]
        WorkingDirNotFound,
        #[serde(rename = "WORKING_DIR_NOT_IN_BASE_DIR")]
        #[doc = "Working directory is not under the base directory"]
        WorkingDirNotInBaseDir,
        #[serde(rename = "DOCKER_UNAVAILABLE")]
        #[doc = "There are issues with docker service/runtime."]
        DockerUnavailable,
        #[serde(rename = "NO_CUDA_CAPABLE_DEVICE")]
        #[doc = "The command failed with \"no cuda-capable device is detected\" error."]
        NoCudaCapableDevice,
        #[serde(rename = "REMOTE_CAS_DOWNLOAD_ERROR")]
        #[doc = "The bot encountered errors from remote CAS when downloading blobs."]
        RemoteCasDownloadError,
        #[serde(rename = "REMOTE_CAS_UPLOAD_ERROR")]
        #[doc = "The bot encountered errors from remote CAS when uploading blobs."]
        RemoteCasUploadError,
        #[serde(rename = "LOCAL_CASPROXY_NOT_RUNNING")]
        #[doc = "The local casproxy is not running."]
        LocalCasproxyNotRunning,
        #[serde(rename = "DOCKER_CREATE_CONTAINER_ERROR")]
        #[doc = "The bot couldn't start the container."]
        DockerCreateContainerError,
        #[serde(rename = "DOCKER_INVALID_ULIMIT")]
        #[doc = "The docker ulimit is not valid."]
        DockerInvalidUlimit,
        #[serde(rename = "DOCKER_UNKNOWN_RUNTIME")]
        #[doc = "The docker runtime is unknown."]
        DockerUnknownRuntime,
        #[serde(rename = "DOCKER_UNKNOWN_CAPABILITY")]
        #[doc = "The docker capability is unknown."]
        DockerUnknownCapability,
        #[serde(rename = "DOCKER_UNKNOWN_ERROR")]
        #[doc = "The command failed with unknown docker errors."]
        DockerUnknownError,
        #[serde(rename = "DOCKER_CREATE_COMPUTE_SYSTEM_ERROR")]
        #[doc = "Docker failed to run containers with CreateComputeSystem error."]
        DockerCreateComputeSystemError,
        #[serde(rename = "DOCKER_PREPARELAYER_ERROR")]
        #[doc = "Docker failed to run containers with hcsshim::PrepareLayer error."]
        DockerPreparelayerError,
        #[serde(rename = "DOCKER_INCOMPATIBLE_OS_ERROR")]
        #[doc = "Docker incompatible operating system error."]
        DockerIncompatibleOsError,
        #[serde(rename = "DOCKER_CREATE_RUNTIME_FILE_NOT_FOUND")]
        #[doc = "Docker failed to create OCI runtime because of file not found."]
        DockerCreateRuntimeFileNotFound,
        #[serde(rename = "DOCKER_CREATE_RUNTIME_PERMISSION_DENIED")]
        #[doc = "Docker failed to create OCI runtime because of permission denied."]
        DockerCreateRuntimePermissionDenied,
        #[serde(rename = "DOCKER_CREATE_PROCESS_FILE_NOT_FOUND")]
        #[doc = "Docker failed to create process because of file not found."]
        DockerCreateProcessFileNotFound,
        #[serde(rename = "DOCKER_CREATE_COMPUTE_SYSTEM_INCORRECT_PARAMETER_ERROR")]
        #[doc = "Docker failed to run containers with CreateComputeSystem error that involves an incorrect parameter (more specific version of DOCKER_CREATE_COMPUTE_SYSTEM_ERROR that is user-caused)."]
        DockerCreateComputeSystemIncorrectParameterError,
        #[serde(rename = "DOCKER_TOO_MANY_SYMBOLIC_LINK_LEVELS")]
        #[doc = "Docker failed to create an overlay mount because of too many levels of symbolic links."]
        DockerTooManySymbolicLinkLevels,
        #[serde(rename = "LOCAL_CONTAINER_MANAGER_NOT_RUNNING")]
        #[doc = "The local Container Manager is not running."]
        LocalContainerManagerNotRunning,
    }
    impl ::std::default::Default for GoogleDevtoolsRemotebuildbotCommandStatusCodeEnum {
        fn default() -> Self {
            Self::Ok
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ResourceUsage is the system resource usage of the host machine."]
    pub struct GoogleDevtoolsRemotebuildbotResourceUsage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuUsedPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_used_percent: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub disk_usage:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemotebuildbotResourceUsageStat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub memory_usage:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemotebuildbotResourceUsageStat>>,
    }
    impl GoogleDevtoolsRemotebuildbotResourceUsage {
        pub fn builder() -> GoogleDevtoolsRemotebuildbotResourceUsageBuilder {
            GoogleDevtoolsRemotebuildbotResourceUsageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleDevtoolsRemotebuildbotResourceUsageStat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "total")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "used")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub used: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildbotResourceUsageStat {
        pub fn builder() -> GoogleDevtoolsRemotebuildbotResourceUsageStatBuilder {
            GoogleDevtoolsRemotebuildbotResourceUsageStatBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AcceleratorConfig defines the accelerator cards to attach to the VM."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaAcceleratorConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acceleratorCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of guest accelerator cards exposed to each VM."]
        pub accelerator_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acceleratorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of accelerator to attach to each VM, e.g. \"nvidia-tesla-k80\" for nVidia Tesla K80."]
        pub accelerator_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaAcceleratorConfig {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaAcceleratorConfigBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaAcceleratorConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Autoscale defines the autoscaling policy of a worker pool."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaAutoscale {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximal number of workers. Must be equal to or greater than min_size."]
        pub max_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimal number of workers. Must be greater than 0."]
        pub min_size: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaAutoscale {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaAutoscaleBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaAutoscaleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for `CreateInstance`."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the instance to create. The name in the instance, if specified in the instance, is ignored."]
        pub instance: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstance>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the created instance. A valid `instance_id` must: be 6-50 characters long, contain only lowercase letters, digits, hyphens and underscores, start with a lowercase letter, and end with a lowercase letter or a digit."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the project containing the instance. Format: `projects/[PROJECT_ID]`."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateInstanceRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateInstanceRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for `CreateWorkerPool`."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateWorkerPoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the instance in which to create the new worker pool. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "poolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the created worker pool. A valid pool ID must: be 6-50 characters long, contain only lowercase letters, digits, hyphens and underscores, start with a lowercase letter, and end with a lowercase letter or a digit."]
        pub pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the worker pool to create. The name in the worker pool, if specified, is ignored."]
        pub worker_pool: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPool>,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateWorkerPoolRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateWorkerPoolRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaCreateWorkerPoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for `DeleteInstance`."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the instance to delete. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteInstanceRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteInstanceRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for DeleteWorkerPool."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteWorkerPoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the worker pool to delete. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteWorkerPoolRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteWorkerPoolRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaDeleteWorkerPoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "FeaturePolicy defines features allowed to be used on RBE instances, as well as instance-wide behavior changes that take effect without opt-in or opt-out at usage time."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerImageSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which container image sources are allowed. Currently only RBE-supported registry (gcr.io) is allowed. One can allow all repositories under a project or one specific repository only. E.g. container_image_sources { policy: RESTRICTED allowed_values: [ \"gcr.io/project-foo\", \"gcr.io/project-bar/repo-baz\", ] } will allow any repositories under \"gcr.io/project-foo\" plus the repository \"gcr.io/project-bar/repo-baz\". Default (UNSPECIFIED) is equivalent to any source is allowed."]
        pub container_image_sources: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerAddCapabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerAddCapabilities can be used or what capabilities are allowed."]
        pub docker_add_capabilities: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerChrootPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerChrootPath can be used."]
        pub docker_chroot_path: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerNetwork can be used or what network modes are allowed. E.g. one may allow `off` value only via `allowed_values`."]
        pub docker_network: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerPrivileged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerPrivileged can be used."]
        pub docker_privileged: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerRunAsRoot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerRunAsRoot can be used."]
        pub docker_run_as_root: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerRuntime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerRuntime is allowed to be set or what runtimes are allowed. Note linux_isolation takes precedence, and if set, docker_runtime values may be rejected if they are incompatible with the selected isolation."]
        pub docker_runtime: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerSiblingContainers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether dockerSiblingContainers can be used."]
        pub docker_sibling_containers: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linuxIsolation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "linux_isolation allows overriding the docker runtime used for containers started on Linux."]
        pub linux_isolation: ::std::option::Option<
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyLinuxIsolationEnum,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicy {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "linux_isolation allows overriding the docker runtime used for containers started on Linux."]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyLinuxIsolationEnum {
        #[serde(rename = "LINUX_ISOLATION_UNSPECIFIED")]
        #[doc = "Default value. Will be using Linux default runtime."]
        LinuxIsolationUnspecified,
        #[serde(rename = "GVISOR")]
        #[doc = "Use gVisor runsc runtime."]
        Gvisor,
        #[serde(rename = "OFF")]
        #[doc = "Use stardard Linux runtime. This has the same behaviour as unspecified, but it can be used to revert back from gVisor."]
        Off,
    }
    impl ::std::default::Default
        for GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyLinuxIsolationEnum
    {
        fn default() -> Self {
            Self::LinuxIsolationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines whether a feature can be used or what values are accepted."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of acceptable values. Only effective when the policy is `RESTRICTED`."]
        pub allowed_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy of the feature."]
        pub policy: ::std::option::Option<
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeaturePolicyEnum,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeature {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeatureBuilder
        {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeatureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The policy of the feature."]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeaturePolicyEnum {
        #[serde(rename = "POLICY_UNSPECIFIED")]
        #[doc = "Default value, if not explicitly set. Equivalent to FORBIDDEN, unless otherwise documented on a specific Feature."]
        PolicyUnspecified,
        #[serde(rename = "ALLOWED")]
        #[doc = "Feature is explicitly allowed."]
        Allowed,
        #[serde(rename = "FORBIDDEN")]
        #[doc = "Feature is forbidden. Requests attempting to leverage it will get an FailedPrecondition error, with a message like: \"Feature forbidden by FeaturePolicy: Feature on instance \""]
        Forbidden,
        #[serde(rename = "RESTRICTED")]
        #[doc = "Only the values specified in the `allowed_values` are allowed."]
        Restricted,
    }
    impl ::std::default::Default
        for GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicyFeaturePolicyEnum
    {
        fn default() -> Self {
            Self::PolicyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for `GetInstance`."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the instance to retrieve. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetInstanceRequest {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetInstanceRequestBuilder
        {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for GetWorkerPool."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetWorkerPoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the worker pool to retrieve. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetWorkerPoolRequest {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetWorkerPoolRequestBuilder
        {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaGetWorkerPoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instance conceptually encapsulates all Remote Build Execution resources for remote builds. An instance consists of storage and compute resources (for example, `ContentAddressableStorage`, `ActionCache`, `WorkerPools`) used for running remote builds. All Remote Build Execution API calls are scoped to an instance."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featurePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy to define whether or not RBE features can be used or how they can be used."]
        pub feature_policy: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaFeaturePolicy>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location is a GCP region. Currently only `us-central1` is supported."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loggingEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether stack driver logging is enabled for the instance."]
        pub logging_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Instance resource name formatted as: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`. Name should not be populated when creating an instance since it is provided in the `instance_id` field."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the instance."]
        pub state:
            ::std::option::Option<GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstanceStateEnum>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstance {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstanceBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the instance."]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstanceStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Not a valid state, but the default value of the enum."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "The instance is in state `CREATING` once `CreateInstance` is called and before the instance is ready for use."]
        Creating,
        #[serde(rename = "RUNNING")]
        #[doc = "The instance is in state `RUNNING` when it is ready for use."]
        Running,
        #[serde(rename = "INACTIVE")]
        #[doc = "An `INACTIVE` instance indicates that there is a problem that needs to be fixed. Such instances cannot be used for execution and instances that remain in this state for a significant period of time will be removed permanently."]
        Inactive,
    }
    impl ::std::default::Default for GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstanceStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the project. Format: `projects/[PROJECT_ID]`."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesRequest {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesRequestBuilder
        {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of instances in a given project."]
        pub instances: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstance>,
            >,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesResponse {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesResponseBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaListInstancesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. String values are case-insensitive. The comparison operator must be either `:`, `=`, `!=`, `>`, `>=`, `<=` or `<`. The `:` operator can be used with string fields to match substrings. For non-string fields it is equivalent to the `=` operator. The `:*` comparison can be used to test whether a key has been defined. You can also filter on nested fields. To filter on multiple expressions, you can separate expression using `AND` and `OR` operators, using parentheses to specify precedence. If neither operator is specified, `AND` is assumed. Examples: Include only pools with more than 100 reserved workers: `(worker_count > 100) (worker_config.reserved = true)` Include only pools with a certain label or machines of the e2-standard family: `worker_config.labels.key1 : * OR worker_config.machine_type: e2-standard`"]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the instance. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPools")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of worker pools in a given instance."]
        pub worker_pools: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPool>,
            >,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsResponse {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsResponseBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaListWorkerPoolsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for `UpdateInstance`."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the instance to update."]
        pub instance: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaInstance>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loggingEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated, use instance.logging_enabled instead. Whether to enable Stackdriver logging for this instance."]
        pub logging_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated, use instance.Name instead. Name of the instance to update. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The update mask applies to instance. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask If an empty update_mask is provided, only the non-default valued field in the worker pool field will be updated. Note that in order to update a field to the default value (zero, false, empty string) an explicit update_mask must be provided."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateInstanceRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateInstanceRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request used for UpdateWorkerPool."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateWorkerPoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The update mask applies to worker_pool. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask If an empty update_mask is provided, only the non-default valued field in the worker pool field will be updated. Note that in order to update a field to the default value (zero, false, empty string) an explicit update_mask must be provided."]
        pub update_mask: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the worker pool to update."]
        pub worker_pool: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPool>,
        >,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateWorkerPoolRequest {
        pub fn builder(
        ) -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateWorkerPoolRequestBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaUpdateWorkerPoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the configuration to be used for creating workers in the worker pool."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accelerator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The accelerator card attached to each VM."]
        pub accelerator: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaAcceleratorConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Size of the disk attached to the worker, in GB. See https://cloud.google.com/compute/docs/disks/"]
        pub disk_size_gb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Disk Type to use for the worker. See [Storage options](https://cloud.google.com/compute/docs/disks/#introduction). Currently only `pd-standard` and `pd-ssd` are supported."]
        pub disk_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels associated with the workers. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International letters are permitted. Label keys must start with a letter. Label values are optional. There can not be more than 64 labels per resource."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Machine type of the worker, such as `e2-standard-2`. See https://cloud.google.com/compute/docs/machine-types for a list of supported machine types. Note that `f1-micro` and `g1-small` are not yet supported."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxConcurrentActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of actions a worker can execute concurrently."]
        pub max_concurrent_actions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minCpuPlatform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum CPU platform to use when creating the worker. See [CPU Platforms](https://cloud.google.com/compute/docs/cpu-platforms)."]
        pub min_cpu_platform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines the type of network access granted to workers. Possible values: - \"public\": Workers can connect to the public internet. - \"private\": Workers can only connect to Google APIs and services. - \"restricted-private\": Workers can only connect to Google APIs that are reachable through `restricted.googleapis.com` (`199.36.153.4/30`)."]
        pub network_access: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reserved")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines whether the worker is reserved (equivalent to a Compute Engine on-demand VM and therefore won't be preempted). See [Preemptible VMs](https://cloud.google.com/preemptible-vms/) for more details."]
        pub reserved: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "soleTenantNodeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node type name to be used for sole-tenant nodes."]
        pub sole_tenant_node_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the image used by each VM."]
        pub vm_image: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerConfig {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerConfigBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A worker pool resource in the Remote Build Execution API."]
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPool {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The autoscale policy to apply on a pool."]
        pub autoscale: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaAutoscale>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel specifies the release channel of the pool."]
        pub channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "WorkerPool resource name formatted as: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`. name should not be populated when creating a worker pool since it is provided in the `poolId` field."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the worker pool."]
        pub state: ::std::option::Option<
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPoolStateEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the properties, such as machine type and disk size, used for creating workers in a worker pool."]
        pub worker_config: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired number of workers in the worker pool. Must be a value between 0 and 15000."]
        pub worker_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPool {
        pub fn builder() -> GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPoolBuilder {
            GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPoolBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the worker pool."]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPoolStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Not a valid state, but the default value of the enum."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "The worker pool is in state `CREATING` once `CreateWorkerPool` is called and before all requested workers are ready."]
        Creating,
        #[serde(rename = "RUNNING")]
        #[doc = "The worker pool is in state `RUNNING` when all its workers are ready for use."]
        Running,
        #[serde(rename = "UPDATING")]
        #[doc = "The worker pool is in state `UPDATING` once `UpdateWorkerPool` is called and before the new configuration has all the requested workers ready for use, and no older configuration has any workers. At that point the state transitions to `RUNNING`."]
        Updating,
        #[serde(rename = "DELETING")]
        #[doc = "The worker pool is in state `DELETING` once the `Delete` method is called and before the deletion completes."]
        Deleting,
        #[serde(rename = "INACTIVE")]
        #[doc = "The worker pool is in state `INACTIVE` when the instance hosting the worker pool in not running."]
        Inactive,
    }
    impl ::std::default::Default for GoogleDevtoolsRemotebuildexecutionAdminV1alphaWorkerPoolStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AdminTemp is a prelimiary set of administration tasks. It's called \"Temp\" because we do not yet know the best way to represent admin tasks; it's possible that this will be entirely replaced in later versions of this API. If this message proves to be sufficient, it will be renamed in the alpha or beta release of this API. This message (suitably marshalled into a protobuf.Any) can be used as the inline_assignment field in a lease; the lease assignment field should simply be `\"admin\"` in these cases. This message is heavily based on Swarming administration tasks from the LUCI project (http://github.com/luci/luci-py/appengine/swarming)."]
    pub struct GoogleDevtoolsRemoteworkersV1test2AdminTemp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The argument to the admin action; see `Command` for semantics."]
        pub arg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "command")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The admin action; see `Command` for legal values."]
        pub command: ::std::option::Option<GoogleDevtoolsRemoteworkersV1test2AdminTempCommandEnum>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2AdminTemp {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2AdminTempBuilder {
            GoogleDevtoolsRemoteworkersV1test2AdminTempBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The admin action; see `Command` for legal values."]
    pub enum GoogleDevtoolsRemoteworkersV1test2AdminTempCommandEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Illegal value."]
        Unspecified,
        #[serde(rename = "BOT_UPDATE")]
        #[doc = "Download and run a new version of the bot. `arg` will be a resource accessible via `ByteStream.Read` to obtain the new bot code."]
        BotUpdate,
        #[serde(rename = "BOT_RESTART")]
        #[doc = "Restart the bot without downloading a new version. `arg` will be a message to log."]
        BotRestart,
        #[serde(rename = "BOT_TERMINATE")]
        #[doc = "Shut down the bot. `arg` will be a task resource name (similar to those in tasks.proto) that the bot can use to tell the server that it is terminating."]
        BotTerminate,
        #[serde(rename = "HOST_RESTART")]
        #[doc = "Restart the host computer. `arg` will be a message to log."]
        HostRestart,
    }
    impl ::std::default::Default for GoogleDevtoolsRemoteworkersV1test2AdminTempCommandEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a blob of binary content with its digest."]
    pub struct GoogleDevtoolsRemoteworkersV1test2Blob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the blob."]
        pub contents: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of the blob. This should be verified by the receiver."]
        pub digest:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2Blob {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2BlobBuilder {
            GoogleDevtoolsRemoteworkersV1test2BlobBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED - use CommandResult instead. Describes the actual outputs from the task."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandOutputs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "exit_code is only fully reliable if the status' code is OK. If the task exceeded its deadline or was cancelled, the process may still produce an exit code as it is cancelled, and this will be populated, but a successful (zero) is unlikely to be correct unless the status code is OK."]
        pub exit_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output files. The blob referenced by the digest should contain one of the following (implementation-dependent): * A marshalled DirectoryMetadata of the returned filesystem * A LUCI-style .isolated file"]
        pub outputs:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandOutputs {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandOutputsBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandOutputsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED - use CommandResult instead. Can be used as part of CompleteRequest.metadata, or are part of a more sophisticated message."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandOverhead {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The elapsed time between calling Accept and Complete. The server will also have its own idea of what this should be, but this excludes the overhead of the RPCs and the bot response time."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overhead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time *not* spent executing the command (ie uploading/downloading files)."]
        pub overhead: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandOverhead {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandOverheadBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandOverheadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All information about the execution of a command, suitable for providing as the Bots interface's `Lease.result` field."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The elapsed time between calling Accept and Complete. The server will also have its own idea of what this should be, but this excludes the overhead of the RPCs and the bot response time."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exit code of the process. An exit code of \"0\" should only be trusted if `status` has a code of OK (otherwise it may simply be unset)."]
        pub exit_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Implementation-dependent metadata about the task. Both servers and bots may define messages which can be encoded here; bots are free to provide metadata in multiple formats, and servers are free to choose one or more of the values to process and ignore others. In particular, it is *not* considered an error for the bot to provide the server with a field that it doesn't know about."]
        pub metadata: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output files. The blob referenced by the digest should contain one of the following (implementation-dependent): * A marshalled DirectoryMetadata of the returned filesystem * A LUCI-style .isolated file"]
        pub outputs:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overhead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time *not* spent executing the command (ie uploading/downloading files)."]
        pub overhead: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An overall status for the command. For example, if the command timed out, this might have a code of DEADLINE_EXCEEDED; if it was killed by the OS for memory exhaustion, it might have a code of RESOURCE_EXHAUSTED."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandResult {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandResultBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a shell-style task to execute, suitable for providing as the Bots interface's `Lease.payload` field."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expectedOutputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expected outputs from the task."]
        pub expected_outputs: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2CommandTaskOutputs>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inputs to the task."]
        pub inputs: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2CommandTaskInputs>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeouts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timeouts of this task."]
        pub timeouts: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2CommandTaskTimeouts>,
        >,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandTask {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandTaskBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the inputs to a shell-style task."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandTaskInputs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The command itself to run (e.g., argv). This field should be passed directly to the underlying operating system, and so it must be sensible to that operating system. For example, on Windows, the first argument might be \"C:\\Windows\\System32\\ping.exe\" - that is, using drive letters and backslashes. A command for a *nix system, on the other hand, would use forward slashes. All other fields in the RWAPI must consistently use forward slashes, since those fields may be interpretted by both the service and the bot."]
        pub arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All environment variables required by the task."]
        pub environment_variables: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsEnvironmentVariable,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input filesystem to be set up prior to the task beginning. The contents should be a repeated set of FileMetadata messages though other formats are allowed if better for the implementation (eg, a LUCI-style .isolated file). This field is repeated since implementations might want to cache the metadata, in which case it may be useful to break up portions of the filesystem that change frequently (eg, specific input files) from those that don't (eg, standard header files)."]
        pub files: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inlineBlobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inline contents for blobs expected to be needed by the bot to execute the task. For example, contents of entries in `files` or blobs that are indirectly referenced by an entry there. The bot should check against this list before downloading required task inputs to reduce the number of communications between itself and the remote CAS server."]
        pub inline_blobs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Blob>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workingDirectory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Directory from which a command is executed. It is a relative directory with respect to the bot's working directory (i.e., \"./\"). If it is non-empty, then it must exist under \"./\". Otherwise, \"./\" will be used."]
        pub working_directory: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandTaskInputs {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An environment variable required by this task."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsEnvironmentVariable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The envvar name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The envvar value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsEnvironmentVariable {
        pub fn builder(
        ) -> GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsEnvironmentVariableBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandTaskInputsEnvironmentVariableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the expected outputs of the command."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandTaskOutputs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of expected directories, relative to the execution root. All paths MUST be delimited by forward slashes."]
        pub directories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of expected files, relative to the execution root. All paths MUST be delimited by forward slashes."]
        pub files: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stderrDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination to which any stderr should be sent. The method by which the bot should send the stream contents to that destination is not defined in this API. As examples, the destination could be a file referenced in the `files` field in this message, or it could be a URI that must be written via the ByteStream API."]
        pub stderr_destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stdoutDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination to which any stdout should be sent. The method by which the bot should send the stream contents to that destination is not defined in this API. As examples, the destination could be a file referenced in the `files` field in this message, or it could be a URI that must be written via the ByteStream API."]
        pub stdout_destination: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandTaskOutputs {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandTaskOutputsBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandTaskOutputsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the timeouts associated with this task."]
    pub struct GoogleDevtoolsRemoteworkersV1test2CommandTaskTimeouts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "execution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This specifies the maximum time that the task can run, excluding the time required to download inputs or upload outputs. That is, the worker will terminate the task if it runs longer than this."]
        pub execution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This specifies the maximum amount of time the task can be idle - that is, go without generating some output in either stdout or stderr. If the process is silent for more than the specified time, the worker will terminate the task."]
        pub idle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shutdown")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the execution or IO timeouts are exceeded, the worker will try to gracefully terminate the task and return any existing logs. However, tasks may be hard-frozen in which case this process will fail. This timeout specifies how long to wait for a terminated task to shut down gracefully (e.g. via SIGTERM) before we bring down the hammer (e.g. SIGKILL on *nix, CTRL_BREAK_EVENT on Windows)."]
        pub shutdown: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2CommandTaskTimeouts {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2CommandTaskTimeoutsBuilder {
            GoogleDevtoolsRemoteworkersV1test2CommandTaskTimeoutsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The CommandTask and CommandResult messages assume the existence of a service that can serve blobs of content, identified by a hash and size known as a \"digest.\" The method by which these blobs may be retrieved is not specified here, but a model implementation is in the Remote Execution API's \"ContentAddressibleStorage\" interface. In the context of the RWAPI, a Digest will virtually always refer to the contents of a file or a directory. The latter is represented by the byte-encoded Directory message."]
    pub struct GoogleDevtoolsRemoteworkersV1test2Digest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string-encoded hash (eg \"1a2b3c\", not the byte array [0x1a, 0x2b, 0x3c]) using an implementation-defined hash algorithm (eg SHA-256)."]
        pub hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the contents. While this is not strictly required as part of an identifier (after all, any given hash will have exactly one canonical size), it's useful in almost all cases when one might want to send or retrieve blobs of content and is included here for this reason."]
        pub size_bytes: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2Digest {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2DigestBuilder {
            GoogleDevtoolsRemoteworkersV1test2DigestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The contents of a directory. Similar to the equivalent message in the Remote Execution API."]
    pub struct GoogleDevtoolsRemoteworkersV1test2Directory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any subdirectories"]
        pub directories: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2DirectoryMetadata>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The files in this directory"]
        pub files: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2FileMetadata>>,
        >,
    }
    impl GoogleDevtoolsRemoteworkersV1test2Directory {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2DirectoryBuilder {
            GoogleDevtoolsRemoteworkersV1test2DirectoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for a directory. Similar to the equivalent message in the Remote Execution API."]
    pub struct GoogleDevtoolsRemoteworkersV1test2DirectoryMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pointer to the contents of the directory, in the form of a marshalled Directory message."]
        pub digest:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path of the directory, as in FileMetadata.path."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2DirectoryMetadata {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2DirectoryMetadataBuilder {
            GoogleDevtoolsRemoteworkersV1test2DirectoryMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for a file. Similar to the equivalent message in the Remote Execution API."]
    pub struct GoogleDevtoolsRemoteworkersV1test2FileMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file is small enough, its contents may also or alternatively be listed here."]
        pub contents: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pointer to the contents of the file. The method by which a client retrieves the contents from a CAS system is not defined here."]
        pub digest:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsRemoteworkersV1test2Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isExecutable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of the file"]
        pub is_executable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path of this file. If this message is part of the CommandOutputs.outputs fields, the path is relative to the execution root and must correspond to an entry in CommandTask.outputs.files. If this message is part of a Directory message, then the path is relative to the root of that directory. All paths MUST be delimited by forward slashes."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsRemoteworkersV1test2FileMetadata {
        pub fn builder() -> GoogleDevtoolsRemoteworkersV1test2FileMetadataBuilder {
            GoogleDevtoolsRemoteworkersV1test2FileMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct GoogleLongrunningCancelOperationRequest {}
    impl GoogleLongrunningCancelOperationRequest {
        pub fn builder() -> GoogleLongrunningCancelOperationRequestBuilder {
            GoogleLongrunningCancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
}
