#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An alias to a repo revision."]
pub struct AliasContext {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias kind."]
    pub kind: ::std::option::Option<AliasContextKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alias kind."]
pub enum AliasContextKindEnum {
    #[serde(rename = "KIND_UNSPECIFIED")]
    #[doc = "Unknown."]
    KindUnspecified,
    #[serde(rename = "FIXED")]
    #[doc = "Git tag."]
    Fixed,
    #[serde(rename = "MOVABLE")]
    #[doc = "Git branch."]
    Movable,
    #[serde(rename = "OTHER")]
    #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named \"refs/foo/bar\"."]
    Other,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Artifact describes a build product."]
pub struct Artifact {
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
    pub checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a hash object for use in Materials and Products."]
pub struct ArtifactHashes {
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines an object to declare an in-toto artifact rule"]
pub struct ArtifactRule {
    #[serde(rename = "artifactRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub artifact_rule: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Occurrence that represents a single \"attestation\". The authenticity of an attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the authority to which this attestation is attached is primarily useful for look-up (how to find this attestation if you already know the authority and artifact to be verified) and intent (which authority was this attestation intended to sign for)."]
pub struct Attestation {
    #[serde(rename = "genericSignedAttestation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub generic_signed_attestation:
        ::std::option::Option<::std::boxed::Box<GenericSignedAttestation>>,
    #[serde(rename = "pgpSignedAttestation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A PGP signed attestation."]
    pub pgp_signed_attestation: ::std::option::Option<::std::boxed::Box<PgpSignedAttestation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Note kind that represents a logical attestation \"role\" or \"authority\". For example, an organization might have one `Authority` for \"QA\" and one for \"build\". This note is intended to act strictly as a grouping mechanism for the attached occurrences (Attestations). This grouping mechanism also provides a security boundary, since IAM ACLs gate the ability for a principle to attach an occurrence to a given note. It also provides a single point of lookup to find all attached attestation occurrences, even if they don't all live in the same project."]
pub struct Authority {
    #[serde(rename = "hint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hint hints at the purpose of the attestation authority."]
    pub hint: ::std::option::Option<::std::boxed::Box<Hint>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Basis describes the base image portion (Note) of the DockerImage relationship. Linked occurrences are derived from this or an equivalent image via: FROM Or an equivalent reference, e.g. a tag of the resource_url."]
pub struct Basis {
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The fingerprint of the base image."]
    pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
    #[serde(rename = "resourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The resource_url for the resource representing the basis of associated occurrence images."]
    pub resource_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to create notes in batch."]
pub struct BatchCreateNotesRequest {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The notes to create, the key is expected to be the note ID. Max allowed length is 1000."]
    pub notes: ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Note>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for creating notes in batch."]
pub struct BatchCreateNotesResponse {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notes that were created."]
    pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Note>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to create occurrences in batch."]
pub struct BatchCreateOccurrencesRequest {
    #[serde(rename = "occurrences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The occurrences to create. Max allowed length is 1000."]
    pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for creating occurrences in batch."]
pub struct BatchCreateOccurrencesResponse {
    #[serde(rename = "occurrences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The occurrences that were created."]
    pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Note holding the version of the provider's builder and the signature of the provenance message in the build details occurrence."]
pub struct Build {
    #[serde(rename = "builderVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Version of the builder which produced this build."]
    pub builder_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Signature of the build in occurrences pointing to this build note containing build details."]
    pub signature: ::std::option::Option<::std::boxed::Box<BuildSignature>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provenance of a build. Contains all information needed to verify the full details about the build from source to completion."]
pub struct BuildProvenance {
    #[serde(rename = "buildOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
    pub build_options:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "builderVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version string of the builder at the time this build was executed."]
    pub builder_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "builtArtifacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output of the build."]
    pub built_artifacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Artifact>>>,
    #[serde(rename = "commands")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Commands requested by the build."]
    pub commands: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Command>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the build was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
    pub creator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which execution of the build was finished."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique identifier of the build."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI where any logs for this provenance were written."]
    pub logs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceProvenance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the Source input to the build."]
    pub source_provenance: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which execution of the build was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message encapsulating the signature of the verified build."]
pub struct BuildSignature {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An ID for the key used to sign. This could be either an ID for the key stored in `public_key` (such as the ID or fingerprint for a PGP key, or the CN for a cert), or a reference to an external key (such as a reference to a key in Cloud Key Management Service)."]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the key, either stored in `public_key` or referenced in `key_id`."]
    pub key_type: ::std::option::Option<BuildSignatureKeyTypeEnum>,
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public key of the builder which can be used to verify that the related findings are valid and unchanged. If `key_type` is empty, this defaults to PEM encoded public keys. This field may be empty if `key_id` references an external key. For Cloud Build based signatures, this is a PEM encoded public key. To verify the Cloud Build signature, place the contents of this field into a file (public.pem). The signature field is base64-decoded into its binary representation in signature.bin, and the provenance bytes from `BuildDetails` are base64-decoded into a binary representation in signed.bin. OpenSSL can then verify the signature: `openssl sha256 -verify public.pem -signature signature.bin signed.bin`"]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Signature of the related `BuildProvenance`. In JSON, this is base-64 encoded."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the key, either stored in `public_key` or referenced in `key_id`."]
pub enum BuildSignatureKeyTypeEnum {
    #[serde(rename = "KEY_TYPE_UNSPECIFIED")]
    #[doc = "`KeyType` is not set."]
    KeyTypeUnspecified,
    #[serde(rename = "PGP_ASCII_ARMORED")]
    #[doc = "`PGP ASCII Armored` public key."]
    PgpAsciiArmored,
    #[serde(rename = "PKIX_PEM")]
    #[doc = "`PKIX PEM` public key."]
    PkixPem,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines an object for the byproducts field in in-toto links. The suggested fields are \"stderr\", \"stdout\", and \"return-value\"."]
pub struct ByProducts {
    #[serde(rename = "customValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_values:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Common Vulnerability Scoring System version 3. For details, see https://www.first.org/cvss/specification-document"]
pub struct CvsSv3 {
    #[serde(rename = "attackComplexity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attack_complexity: ::std::option::Option<CvsSv3AttackComplexityEnum>,
    #[serde(rename = "attackVector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments."]
    pub attack_vector: ::std::option::Option<CvsSv3AttackVectorEnum>,
    #[serde(rename = "availabilityImpact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub availability_impact: ::std::option::Option<CvsSv3AvailabilityImpactEnum>,
    #[serde(rename = "baseScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base score is a function of the base metric scores."]
    pub base_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "confidentialityImpact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub confidentiality_impact: ::std::option::Option<CvsSv3ConfidentialityImpactEnum>,
    #[serde(rename = "exploitabilityScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exploitability_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "impactScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub impact_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "integrityImpact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integrity_impact: ::std::option::Option<CvsSv3IntegrityImpactEnum>,
    #[serde(rename = "privilegesRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub privileges_required: ::std::option::Option<CvsSv3PrivilegesRequiredEnum>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<CvsSv3ScopeEnum>,
    #[serde(rename = "userInteraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_interaction: ::std::option::Option<CvsSv3UserInteractionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3AttackComplexityEnum {
    #[serde(rename = "ATTACK_COMPLEXITY_UNSPECIFIED")]
    #[doc = ""]
    AttackComplexityUnspecified,
    #[serde(rename = "ATTACK_COMPLEXITY_LOW")]
    #[doc = ""]
    AttackComplexityLow,
    #[serde(rename = "ATTACK_COMPLEXITY_HIGH")]
    #[doc = ""]
    AttackComplexityHigh,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments."]
pub enum CvsSv3AttackVectorEnum {
    #[serde(rename = "ATTACK_VECTOR_UNSPECIFIED")]
    #[doc = ""]
    AttackVectorUnspecified,
    #[serde(rename = "ATTACK_VECTOR_NETWORK")]
    #[doc = ""]
    AttackVectorNetwork,
    #[serde(rename = "ATTACK_VECTOR_ADJACENT")]
    #[doc = ""]
    AttackVectorAdjacent,
    #[serde(rename = "ATTACK_VECTOR_LOCAL")]
    #[doc = ""]
    AttackVectorLocal,
    #[serde(rename = "ATTACK_VECTOR_PHYSICAL")]
    #[doc = ""]
    AttackVectorPhysical,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3AvailabilityImpactEnum {
    #[serde(rename = "IMPACT_UNSPECIFIED")]
    #[doc = ""]
    ImpactUnspecified,
    #[serde(rename = "IMPACT_HIGH")]
    #[doc = ""]
    ImpactHigh,
    #[serde(rename = "IMPACT_LOW")]
    #[doc = ""]
    ImpactLow,
    #[serde(rename = "IMPACT_NONE")]
    #[doc = ""]
    ImpactNone,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3ConfidentialityImpactEnum {
    #[serde(rename = "IMPACT_UNSPECIFIED")]
    #[doc = ""]
    ImpactUnspecified,
    #[serde(rename = "IMPACT_HIGH")]
    #[doc = ""]
    ImpactHigh,
    #[serde(rename = "IMPACT_LOW")]
    #[doc = ""]
    ImpactLow,
    #[serde(rename = "IMPACT_NONE")]
    #[doc = ""]
    ImpactNone,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3IntegrityImpactEnum {
    #[serde(rename = "IMPACT_UNSPECIFIED")]
    #[doc = ""]
    ImpactUnspecified,
    #[serde(rename = "IMPACT_HIGH")]
    #[doc = ""]
    ImpactHigh,
    #[serde(rename = "IMPACT_LOW")]
    #[doc = ""]
    ImpactLow,
    #[serde(rename = "IMPACT_NONE")]
    #[doc = ""]
    ImpactNone,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3PrivilegesRequiredEnum {
    #[serde(rename = "PRIVILEGES_REQUIRED_UNSPECIFIED")]
    #[doc = ""]
    PrivilegesRequiredUnspecified,
    #[serde(rename = "PRIVILEGES_REQUIRED_NONE")]
    #[doc = ""]
    PrivilegesRequiredNone,
    #[serde(rename = "PRIVILEGES_REQUIRED_LOW")]
    #[doc = ""]
    PrivilegesRequiredLow,
    #[serde(rename = "PRIVILEGES_REQUIRED_HIGH")]
    #[doc = ""]
    PrivilegesRequiredHigh,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3ScopeEnum {
    #[serde(rename = "SCOPE_UNSPECIFIED")]
    #[doc = ""]
    ScopeUnspecified,
    #[serde(rename = "SCOPE_UNCHANGED")]
    #[doc = ""]
    ScopeUnchanged,
    #[serde(rename = "SCOPE_CHANGED")]
    #[doc = ""]
    ScopeChanged,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CvsSv3UserInteractionEnum {
    #[serde(rename = "USER_INTERACTION_UNSPECIFIED")]
    #[doc = ""]
    UserInteractionUnspecified,
    #[serde(rename = "USER_INTERACTION_NONE")]
    #[doc = ""]
    UserInteractionNone,
    #[serde(rename = "USER_INTERACTION_REQUIRED")]
    #[doc = ""]
    UserInteractionRequired,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo."]
pub struct CloudRepoSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "repoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the repo."]
    pub repo_id: ::std::option::Option<::std::boxed::Box<RepoId>>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Command describes a step performed as part of the build pipeline."]
pub struct Command {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Command-line arguments used when executing this command."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Working directory (relative to project source root) used when running this command."]
    pub dir: ::std::option::Option<::std::string::String>,
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment variables set before running this command."]
    pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional unique identifier for this command, used in wait_for to reference this command as a dependency."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "waitFor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID(s) of the command(s) that this command depends on."]
    pub wait_for: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An artifact that can be deployed in some runtime."]
pub struct Deployable {
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource URI for the artifact being deployed."]
    pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The period during which some deployable was active in a runtime."]
pub struct Deployment {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Address of the runtime element hosting this deployment."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration used to create this deployment."]
    pub config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Beginning of the lifetime of this deployment."]
    pub deploy_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform hosting this deployment."]
    pub platform: ::std::option::Option<DeploymentPlatformEnum>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
    pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "undeployTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End of the lifetime of this deployment."]
    pub undeploy_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identity of the user that triggered this deployment."]
    pub user_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Platform hosting this deployment."]
pub enum DeploymentPlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Unknown."]
    PlatformUnspecified,
    #[serde(rename = "GKE")]
    #[doc = "Google Container Engine."]
    Gke,
    #[serde(rename = "FLEX")]
    #[doc = "Google App Engine: Flexible Environment."]
    Flex,
    #[serde(rename = "CUSTOM")]
    #[doc = "Custom user-defined platform."]
    Custom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Derived describes the derived image portion (Occurrence) of the DockerImage relationship. This image would be produced from a Dockerfile with FROM ."]
pub struct Derived {
    #[serde(rename = "baseResourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
    pub base_resource_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "distance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
    pub distance: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fingerprint of the derived image."]
    pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
    #[serde(rename = "layerInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This contains layer-specific metadata, if populated it has length \"distance\" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer."]
    pub layer_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layer>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies all appearances of this vulnerability in the package for a specific distro/location. For example: glibc in cpe:/o:debian:debian_linux:8 for versions 2.1 - 2.2"]
pub struct Detail {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A vendor-specific description of this note."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fix for this specific package version."]
    pub fixed_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
    #[serde(rename = "isObsolete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this detail is obsolete. Occurrences are expected not to point to obsolete details."]
    pub is_obsolete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxAffectedVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The max version of the package in which the vulnerability exists."]
    pub max_affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
    #[serde(rename = "minAffectedVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The min version of the package in which the vulnerability exists."]
    pub min_affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the package where the vulnerability was found."]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc)."]
    pub package_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severityName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity (eg: distro assigned severity) for this vulnerability."]
    pub severity_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source from which the information in this Detail was obtained."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker."]
    pub source_update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of an attestation occurrence."]
pub struct Details {
    #[serde(rename = "attestation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Attestation for the resource."]
    pub attestation: ::std::option::Option<::std::boxed::Box<Attestation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides information about the analysis status of a discovered resource."]
pub struct Discovered {
    #[serde(rename = "analysisStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of discovery for the resource."]
    pub analysis_status: ::std::option::Option<DiscoveredAnalysisStatusEnum>,
    #[serde(rename = "analysisStatusError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API."]
    pub analysis_status_error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "continuousAnalysis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the resource is continuously analyzed."]
    pub continuous_analysis: ::std::option::Option<DiscoveredContinuousAnalysisEnum>,
    #[serde(rename = "lastAnalysisTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time continuous analysis was done for this resource. Deprecated, do not use."]
    pub last_analysis_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of discovery for the resource."]
pub enum DiscoveredAnalysisStatusEnum {
    #[serde(rename = "ANALYSIS_STATUS_UNSPECIFIED")]
    #[doc = "Unknown."]
    AnalysisStatusUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "Resource is known but no action has been taken yet."]
    Pending,
    #[serde(rename = "SCANNING")]
    #[doc = "Resource is being analyzed."]
    Scanning,
    #[serde(rename = "FINISHED_SUCCESS")]
    #[doc = "Analysis has finished successfully."]
    FinishedSuccess,
    #[serde(rename = "FINISHED_FAILED")]
    #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
    FinishedFailed,
    #[serde(rename = "FINISHED_UNSUPPORTED")]
    #[doc = "The resource is known not to be supported"]
    FinishedUnsupported,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the resource is continuously analyzed."]
pub enum DiscoveredContinuousAnalysisEnum {
    #[serde(rename = "CONTINUOUS_ANALYSIS_UNSPECIFIED")]
    #[doc = "Unknown."]
    ContinuousAnalysisUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The resource is continuously analyzed."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The resource is ignored for continuous analysis."]
    Inactive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A note that indicates a type of analysis a provider would perform. This note exists in a provider's project. A `Discovery` occurrence is created in a consumer's project at the start of analysis."]
pub struct Discovery {
    #[serde(rename = "analysisKind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The kind of analysis that is handled by this discovery."]
    pub analysis_kind: ::std::option::Option<DiscoveryAnalysisKindEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The kind of analysis that is handled by this discovery."]
pub enum DiscoveryAnalysisKindEnum {
    #[serde(rename = "NOTE_KIND_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    NoteKindUnspecified,
    #[serde(rename = "VULNERABILITY")]
    #[doc = "The note and occurrence represent a package vulnerability."]
    Vulnerability,
    #[serde(rename = "BUILD")]
    #[doc = "The note and occurrence assert build provenance."]
    Build,
    #[serde(rename = "IMAGE")]
    #[doc = "This represents an image basis relationship."]
    Image,
    #[serde(rename = "PACKAGE")]
    #[doc = "This represents a package installed via a package manager."]
    Package,
    #[serde(rename = "DEPLOYMENT")]
    #[doc = "The note and occurrence track deployment events."]
    Deployment,
    #[serde(rename = "DISCOVERY")]
    #[doc = "The note and occurrence track the initial discovery status of a resource."]
    Discovery,
    #[serde(rename = "ATTESTATION")]
    #[doc = "This represents a logical \"role\" that can attest to artifacts."]
    Attestation,
    #[serde(rename = "INTOTO")]
    #[doc = "This represents an in-toto link."]
    Intoto,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This represents a particular channel of distribution for a given package. E.g., Debian's jessie-backports dpkg mirror."]
pub struct Distribution {
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CPU architecture for which packages in this distribution channel were built."]
    pub architecture: ::std::option::Option<DistributionArchitectureEnum>,
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The distribution channel-specific description of this package."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest available version of this package in this distribution channel."]
    pub latest_version: ::std::option::Option<::std::boxed::Box<Version>>,
    #[serde(rename = "maintainer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A freeform string denoting the maintainer of this package."]
    pub maintainer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The distribution channel-specific homepage for this package."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The CPU architecture for which packages in this distribution channel were built."]
pub enum DistributionArchitectureEnum {
    #[serde(rename = "ARCHITECTURE_UNSPECIFIED")]
    #[doc = "Unknown architecture."]
    ArchitectureUnspecified,
    #[serde(rename = "X86")]
    #[doc = "X86 architecture."]
    X86,
    #[serde(rename = "X64")]
    #[doc = "X64 architecture."]
    X64,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines an object for the environment field in in-toto links. The suggested fields are \"variables\", \"filesystem\", and \"workdir\"."]
pub struct Environment {
    #[serde(rename = "customValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_values:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct Expr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container message for hashes of byte content of files, used in source messages to verify integrity of source input to the build."]
pub struct FileHashes {
    #[serde(rename = "fileHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Collection of file hashes."]
    pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hash>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of properties that uniquely identify a given Docker image."]
pub struct Fingerprint {
    #[serde(rename = "v1Name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The layer ID of the final layer in the Docker image's v1 representation."]
    pub v1_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "v2Blob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of v2 blobs that represent a given image."]
    pub v2_blob: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "v2Name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + \" \" + v2_name[N+1]) Only the name of the final blob is kept."]
    pub v2_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Per resource and severity counts of fixable and total vulnerabilities."]
pub struct FixableTotalByDigest {
    #[serde(rename = "fixableCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of fixable vulnerabilities associated with this resource."]
    pub fixable_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The affected resource."]
    pub resource: ::std::option::Option<::std::boxed::Box<Resource>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity for this count. SEVERITY_UNSPECIFIED indicates total across all severities."]
    pub severity: ::std::option::Option<FixableTotalByDigestSeverityEnum>,
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of vulnerabilities associated with this resource."]
    pub total_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity for this count. SEVERITY_UNSPECIFIED indicates total across all severities."]
pub enum FixableTotalByDigestSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An attestation wrapper that uses the Grafeas `Signature` message. This attestation must define the `serialized_payload` that the `signatures` verify and any metadata necessary to interpret that plaintext. The signatures should always be over the `serialized_payload` bytestring."]
pub struct GenericSignedAttestation {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
    pub content_type: ::std::option::Option<GenericSignedAttestationContentTypeEnum>,
    #[serde(rename = "serializedPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The serialized payload that is verified by one or more `signatures`. The encoding and semantic meaning of this payload must match what is set in `content_type`."]
    pub serialized_payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification."]
    pub signatures: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Signature>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
pub enum GenericSignedAttestationContentTypeEnum {
    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
    #[doc = "`ContentType` is not set."]
    ContentTypeUnspecified,
    #[serde(rename = "SIMPLE_SIGNING_JSON")]
    #[doc = "Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted in `plaintext` is a JSON blob conforming to the linked schema."]
    SimpleSigningJson,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext referring to a Gerrit project."]
pub struct GerritSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "gerritProject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is the hostURI/project."]
    pub gerrit_project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of a running Gerrit instance."]
    pub host_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision (commit) ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `GetIamPolicy` method."]
pub struct GetIamPolicyRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
    pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates settings provided to GetIamPolicy."]
pub struct GetPolicyOptions {
    #[serde(rename = "requestedPolicyVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub)."]
pub struct GitSourceContext {
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git commit hash."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git repository URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for all operations used and required for all operations that created by Container Analysis Providers"]
pub struct GoogleDevtoolsContaineranalysisV1alpha1OperationMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time that this operation was marked completed or failed."]
    pub end_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a build occurrence."]
pub struct GrafeasV1beta1BuildDetails {
    #[serde(rename = "provenance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The actual provenance for the build."]
    pub provenance: ::std::option::Option<::std::boxed::Box<BuildProvenance>>,
    #[serde(rename = "provenanceBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
    pub provenance_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a deployment occurrence."]
pub struct GrafeasV1beta1DeploymentDetails {
    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deployment history for the resource."]
    pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a discovery occurrence."]
pub struct GrafeasV1beta1DiscoveryDetails {
    #[serde(rename = "discovered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Analysis status for the discovered resource."]
    pub discovered: ::std::option::Option<::std::boxed::Box<Discovered>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of an image occurrence."]
pub struct GrafeasV1beta1ImageDetails {
    #[serde(rename = "derivedImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The child image derived from the base image."]
    pub derived_image: ::std::option::Option<::std::boxed::Box<Derived>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrafeasV1beta1IntotoArtifact {
    #[serde(rename = "hashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hashes: ::std::option::Option<::std::boxed::Box<ArtifactHashes>>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This corresponds to a signed in-toto link - it is made up of one or more signatures and the in-toto link itself. This is used for occurrences of a Grafeas in-toto note."]
pub struct GrafeasV1beta1IntotoDetails {
    #[serde(rename = "signatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub signatures:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GrafeasV1beta1IntotoSignature>>>,
    #[serde(rename = "signed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub signed: ::std::option::Option<::std::boxed::Box<Link>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A signature object consists of the KeyID used and the signature itself."]
pub struct GrafeasV1beta1IntotoSignature {
    #[serde(rename = "keyid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keyid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sig: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a package occurrence."]
pub struct GrafeasV1beta1PackageDetails {
    #[serde(rename = "installation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Where the package was installed."]
    pub installation: ::std::option::Option<::std::boxed::Box<Installation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a vulnerability Occurrence."]
pub struct GrafeasV1beta1VulnerabilityDetails {
    #[serde(rename = "cvssScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0-10 where 0 indicates low severity and 10 indicates high severity."]
    pub cvss_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "effectiveSeverity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The distro assigned severity for this vulnerability when it is available, and note provider assigned severity when distro has not yet assigned a severity for this vulnerability."]
    pub effective_severity:
        ::std::option::Option<GrafeasV1beta1VulnerabilityDetailsEffectiveSeverityEnum>,
    #[serde(rename = "longDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A detailed description of this vulnerability."]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageIssue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of affected locations and their fixes (if available) within the associated resource."]
    pub package_issue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageIssue>>>,
    #[serde(rename = "relatedUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URLs related to this vulnerability."]
    pub related_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedUrl>>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The note provider assigned Severity of the vulnerability."]
    pub severity: ::std::option::Option<GrafeasV1beta1VulnerabilityDetailsSeverityEnum>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A one sentence description of this vulnerability."]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc)"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The distro assigned severity for this vulnerability when it is available, and note provider assigned severity when distro has not yet assigned a severity for this vulnerability."]
pub enum GrafeasV1beta1VulnerabilityDetailsEffectiveSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The note provider assigned Severity of the vulnerability."]
pub enum GrafeasV1beta1VulnerabilityDetailsSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container message for hash values."]
pub struct Hash {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of hash that was performed."]
    pub _type: ::std::option::Option<HashTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The hash value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of hash that was performed."]
pub enum HashTypeEnum {
    #[serde(rename = "HASH_TYPE_UNSPECIFIED")]
    #[doc = "Unknown."]
    HashTypeUnspecified,
    #[serde(rename = "SHA256")]
    #[doc = "A SHA-256 hash."]
    Sha256,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This submessage provides human-readable hints about the purpose of the authority. Because the name of a note acts as its resource reference, it is important to disambiguate the canonical name of the Note (which might be a UUID for security purposes) from \"readable\" names more suitable for debug output. Note that these hints should not be used to look up authorities in security sensitive contexts, such as when looking up attestations to verify."]
pub struct Hint {
    #[serde(rename = "humanReadableName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The human readable name of this attestation authority, for example \"qa\"."]
    pub human_readable_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This contains the fields corresponding to the definition of a software supply chain step in an in-toto layout. This information goes into a Grafeas note."]
pub struct InToto {
    #[serde(rename = "expectedCommand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the expected command used to perform the step."]
    pub expected_command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "expectedMaterials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The following fields contain in-toto artifact rules identifying the artifacts that enter this supply chain step, and exit the supply chain step, i.e. materials and products of the step."]
    pub expected_materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ArtifactRule>>>,
    #[serde(rename = "expectedProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expected_products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ArtifactRule>>>,
    #[serde(rename = "signingKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the public keys that can be used to verify the signatures on the step metadata."]
    pub signing_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SigningKey>>>,
    #[serde(rename = "stepName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field identifies the name of the step in the supply chain."]
    pub step_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a value that indicates the minimum number of keys that need to be used to sign the step's in-toto link."]
    pub threshold: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This represents how a particular software package may be installed on a system."]
pub struct Installation {
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All of the places within the filesystem versions of this package have been found."]
    pub location: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the installed package."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeBase {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The KB name (generally of the form KB[0-9]+ i.e. KB123456)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the KB in the Windows update catalog - https://www.catalog.update.microsoft.com/"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Layer holds metadata specific to a layer of a Docker image."]
pub struct Layer {
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recovered arguments to the Dockerfile directive."]
    pub arguments: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The recovered Dockerfile directive used to construct this layer."]
    pub directive: ::std::option::Option<LayerDirectiveEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The recovered Dockerfile directive used to construct this layer."]
pub enum LayerDirectiveEnum {
    #[serde(rename = "DIRECTIVE_UNSPECIFIED")]
    #[doc = "Default value for unsupported/missing directive."]
    DirectiveUnspecified,
    #[serde(rename = "MAINTAINER")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Maintainer,
    #[serde(rename = "RUN")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Run,
    #[serde(rename = "CMD")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Cmd,
    #[serde(rename = "LABEL")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Label,
    #[serde(rename = "EXPOSE")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Expose,
    #[serde(rename = "ENV")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Env,
    #[serde(rename = "ADD")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Add,
    #[serde(rename = "COPY")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Copy,
    #[serde(rename = "ENTRYPOINT")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Entrypoint,
    #[serde(rename = "VOLUME")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Volume,
    #[serde(rename = "USER")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    User,
    #[serde(rename = "WORKDIR")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Workdir,
    #[serde(rename = "ARG")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Arg,
    #[serde(rename = "ONBUILD")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Onbuild,
    #[serde(rename = "STOPSIGNAL")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Stopsignal,
    #[serde(rename = "HEALTHCHECK")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Healthcheck,
    #[serde(rename = "SHELL")]
    #[doc = "https://docs.docker.com/engine/reference/builder/"]
    Shell,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This corresponds to an in-toto link."]
pub struct Link {
    #[serde(rename = "byproducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ByProducts are data generated as part of a software supply chain step, but are not the actual result of the step."]
    pub byproducts: ::std::option::Option<::std::boxed::Box<ByProducts>>,
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the full command executed for the step. This can also be empty if links are generated for operations that aren't directly mapped to a specific command. Each term in the command is an independent string in the list. An example of a command in the in-toto metadata field is: \"command\": [\"git\", \"clone\", \"https://github.com/in-toto/demo-project.git\"]"]
    pub command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a field that can be used to capture information about the environment. It is suggested for this field to contain information that details environment variables, filesystem information, and the present working directory. The recommended structure of this field is: \"environment\": { \"custom_values\": { \"variables\": \"\", \"filesystem\": \"\", \"workdir\": \"\", \"\": \"...\" } }"]
    pub environment: ::std::option::Option<::std::boxed::Box<Environment>>,
    #[serde(rename = "materials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Materials are the supply chain artifacts that go into the step and are used for the operation performed. The key of the map is the path of the artifact and the structure contains the recorded hash information. An example is: \"materials\": [ { \"resource_uri\": \"foo/bar\", \"hashes\": { \"sha256\": \"ebebf...\", : } } ]"]
    pub materials:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GrafeasV1beta1IntotoArtifact>>>,
    #[serde(rename = "products")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Products are the supply chain artifacts generated as a result of the step. The structure is identical to that of materials."]
    pub products:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GrafeasV1beta1IntotoArtifact>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing occurrences for a note."]
pub struct ListNoteOccurrencesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to provide to skip to a particular spot in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "occurrences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The occurrences attached to the specified note."]
    pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing notes."]
pub struct ListNotesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notes requested."]
    pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Note>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing occurrences."]
pub struct ListOccurrencesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "occurrences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The occurrences requested."]
    pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing scan configurations."]
pub struct ListScanConfigsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scanConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scan configurations requested."]
    pub scan_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanConfig>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An occurrence of a particular package installation found within a system's filesystem. E.g., glibc was found in `/var/lib/dpkg/status`."]
pub struct Location {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path from which we gathered that this package/version is installed."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version installed at this location."]
    pub version: ::std::option::Option<::std::boxed::Box<Version>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A type of analysis that can be done for a resource."]
pub struct Note {
    #[serde(rename = "attestationAuthority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing an attestation role."]
    pub attestation_authority: ::std::option::Option<::std::boxed::Box<Authority>>,
    #[serde(rename = "baseImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing a base image."]
    pub base_image: ::std::option::Option<::std::boxed::Box<Basis>>,
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing build provenance for a verifiable build."]
    pub build: ::std::option::Option<::std::boxed::Box<Build>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this note was created. This field can be used as a filter in list requests."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing something that can be deployed."]
    pub deployable: ::std::option::Option<::std::boxed::Box<Deployable>>,
    #[serde(rename = "discovery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing the initial analysis of a resource."]
    pub discovery: ::std::option::Option<::std::boxed::Box<Discovery>>,
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of expiration for this note. Empty if note does not expire."]
    pub expiration_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intoto")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing an in-toto link."]
    pub intoto: ::std::option::Option<::std::boxed::Box<InToto>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of analysis. This field can be used as a filter in list requests."]
    pub kind: ::std::option::Option<NoteKindEnum>,
    #[serde(rename = "longDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed description of this note."]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing a package hosted by various package managers."]
    pub package: ::std::option::Option<::std::boxed::Box<Package>>,
    #[serde(rename = "relatedNoteNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Other notes related to this note."]
    pub related_note_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "relatedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URLs associated with this note."]
    pub related_url: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedUrl>>>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A one sentence description of this note."]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this note was last updated. This field can be used as a filter in list requests."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vulnerability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A note describing a package vulnerability."]
    pub vulnerability: ::std::option::Option<::std::boxed::Box<Vulnerability>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of analysis. This field can be used as a filter in list requests."]
pub enum NoteKindEnum {
    #[serde(rename = "NOTE_KIND_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    NoteKindUnspecified,
    #[serde(rename = "VULNERABILITY")]
    #[doc = "The note and occurrence represent a package vulnerability."]
    Vulnerability,
    #[serde(rename = "BUILD")]
    #[doc = "The note and occurrence assert build provenance."]
    Build,
    #[serde(rename = "IMAGE")]
    #[doc = "This represents an image basis relationship."]
    Image,
    #[serde(rename = "PACKAGE")]
    #[doc = "This represents a package installed via a package manager."]
    Package,
    #[serde(rename = "DEPLOYMENT")]
    #[doc = "The note and occurrence track deployment events."]
    Deployment,
    #[serde(rename = "DISCOVERY")]
    #[doc = "The note and occurrence track the initial discovery status of a resource."]
    Discovery,
    #[serde(rename = "ATTESTATION")]
    #[doc = "This represents a logical \"role\" that can attest to artifacts."]
    Attestation,
    #[serde(rename = "INTOTO")]
    #[doc = "This represents an in-toto link."]
    Intoto,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An instance of an analysis type that has been found on a resource."]
pub struct Occurrence {
    #[serde(rename = "attestation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes an attestation of an artifact."]
    pub attestation: ::std::option::Option<::std::boxed::Box<Details>>,
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes a verifiable build."]
    pub build: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1BuildDetails>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this occurrence was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes the deployment of an artifact on a runtime."]
    pub deployment: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1DeploymentDetails>>,
    #[serde(rename = "derivedImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes how this resource derives from the basis in the associated note."]
    pub derived_image: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1ImageDetails>>,
    #[serde(rename = "discovered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes when a resource was discovered."]
    pub discovered: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1DiscoveryDetails>>,
    #[serde(rename = "installation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes the installation of a package on the linked resource."]
    pub installation: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1PackageDetails>>,
    #[serde(rename = "intoto")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes a specific in-toto link."]
    pub intoto: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1IntotoDetails>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
    pub kind: ::std::option::Option<OccurrenceKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "noteName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests."]
    pub note_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remediation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of actions that can be taken to remedy the note."]
    pub remediation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The resource for which the occurrence applies."]
    pub resource: ::std::option::Option<::std::boxed::Box<Resource>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this occurrence was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vulnerability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes a security vulnerability."]
    pub vulnerability: ::std::option::Option<::std::boxed::Box<GrafeasV1beta1VulnerabilityDetails>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
pub enum OccurrenceKindEnum {
    #[serde(rename = "NOTE_KIND_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    NoteKindUnspecified,
    #[serde(rename = "VULNERABILITY")]
    #[doc = "The note and occurrence represent a package vulnerability."]
    Vulnerability,
    #[serde(rename = "BUILD")]
    #[doc = "The note and occurrence assert build provenance."]
    Build,
    #[serde(rename = "IMAGE")]
    #[doc = "This represents an image basis relationship."]
    Image,
    #[serde(rename = "PACKAGE")]
    #[doc = "This represents a package installed via a package manager."]
    Package,
    #[serde(rename = "DEPLOYMENT")]
    #[doc = "The note and occurrence track deployment events."]
    Deployment,
    #[serde(rename = "DISCOVERY")]
    #[doc = "The note and occurrence track the initial discovery status of a resource."]
    Discovery,
    #[serde(rename = "ATTESTATION")]
    #[doc = "This represents a logical \"role\" that can attest to artifacts."]
    Attestation,
    #[serde(rename = "INTOTO")]
    #[doc = "This represents an in-toto link."]
    Intoto,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This represents a particular package that is distributed over various channels. E.g., glibc (aka libc6) is distributed by many, at various versions."]
pub struct Package {
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The various channels by which a package is distributed."]
    pub distribution: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Distribution>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The name of the package."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message wraps a location affected by a vulnerability and its associated fix (if one is available)."]
pub struct PackageIssue {
    #[serde(rename = "affectedLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The location of the vulnerability."]
    pub affected_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
    #[serde(rename = "fixedLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the available fix for vulnerability."]
    pub fixed_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
    #[serde(rename = "severityName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated, use Details.effective_severity instead The severity (e.g., distro assigned severity) for this vulnerability."]
    pub severity_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An attestation wrapper with a PGP-compatible signature. This message only supports `ATTACHED` signatures, where the payload that is signed is included alongside the signature itself in the same file."]
pub struct PgpSignedAttestation {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
    pub content_type: ::std::option::Option<PgpSignedAttestationContentTypeEnum>,
    #[serde(rename = "pgpKeyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cryptographic fingerprint of the key used to generate the signature, as output by, e.g. `gpg --list-keys`. This should be the version 4, full 160-bit fingerprint, expressed as a 40 character hexidecimal string. See https://tools.ietf.org/html/rfc4880#section-12.2 for details. Implementations may choose to acknowledge \"LONG\", \"SHORT\", or other abbreviated key IDs, but only the full fingerprint is guaranteed to work. In gpg, the full fingerprint can be retrieved from the `fpr` field returned when calling --list-keys with --with-colons. For example: ``` gpg --with-colons --with-fingerprint --force-v4-certs \\ --list-keys attester@example.com tru::1:1513631572:0:3:1:5 pub:...... fpr:::::::::24FF6481B76AC91E66A00AC657A93A81EF3AE6FB: ``` Above, the fingerprint is `24FF6481B76AC91E66A00AC657A93A81EF3AE6FB`."]
    pub pgp_key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The raw content of the signature, as output by GNU Privacy Guard (GPG) or equivalent. Since this message only supports attached signatures, the payload that was signed must be attached. While the signature format supported is dependent on the verification implementation, currently only ASCII-armored (`--armor` to gpg), non-clearsigned (`--sign` rather than `--clearsign` to gpg) are supported. Concretely, `gpg --sign --armor --output=signature.gpg payload.json` will create the signature content expected in this field in `signature.gpg` for the `payload.json` attestation payload."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
pub enum PgpSignedAttestationContentTypeEnum {
    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
    #[doc = "`ContentType` is not set."]
    ContentTypeUnspecified,
    #[serde(rename = "SIMPLE_SIGNING_JSON")]
    #[doc = "Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted from `signature` is a JSON blob conforming to the linked schema."]
    SimpleSigningJson,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project."]
pub struct ProjectRepoId {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repoName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the repo. Leave empty for the default repo."]
    pub repo_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for any related URL information."]
pub struct RelatedUrl {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label to describe usage of the URL."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specific URL associated with the resource."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for a Cloud Repo."]
pub struct RepoId {
    #[serde(rename = "projectRepoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A combination of a project ID and a repo name."]
    pub project_repo_id: ::std::option::Option<::std::boxed::Box<ProjectRepoId>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-assigned, globally unique identifier."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An entity that can have metadata. For example, a Docker image."]
pub struct Resource {
    #[serde(rename = "contentHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated, do not use. Use uri instead. The hash of the resource content. For example, the Docker digest."]
    pub content_hash: ::std::option::Option<::std::boxed::Box<Hash>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated, do not use. Use uri instead. The name of the resource. For example, the name of a Docker image - \"Debian\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique URI of the resource. For example, `https://gcr.io/project/image@sha256:foo` for a Docker image."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A scan configuration specifies whether Cloud components in a project have a particular type of analysis being run. For example, it can configure whether vulnerability scanning is being done on Docker images or not."]
pub struct ScanConfig {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this scan config was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A human-readable description of what the scan configuration does."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the scan is enabled."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the scan configuration in the form of `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this scan config was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Verifiers (e.g. Kritis implementations) MUST verify signatures with respect to the trust anchors defined in policy (e.g. a Kritis policy). Typically this means that the verifier has been configured with a map from `public_key_id` to public key material (and any required parameters, e.g. signing algorithm). In particular, verification implementations MUST NOT treat the signature `public_key_id` as anything more than a key lookup hint. The `public_key_id` DOES NOT validate or authenticate a public key; it only provides a mechanism for quickly selecting a public key ALREADY CONFIGURED on the verifier through a trusted channel. Verification implementations MUST reject signatures in any of the following circumstances: * The `public_key_id` is not recognized by the verifier. * The public key that `public_key_id` refers to does not verify the signature with respect to the payload. The `signature` contents SHOULD NOT be \"attached\" (where the payload is included with the serialized `signature` bytes). Verifiers MUST ignore any \"attached\" payload and only verify signatures with respect to explicitly provided payload (e.g. a `payload` field on the proto message that holds this Signature, or the canonical serialization of the proto message that holds this signature)."]
pub struct Signature {
    #[serde(rename = "publicKeyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * \"openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA\" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\" * \"nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5\""]
    pub public_key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This defines the format used to record keys used in the software supply chain. An in-toto link is attested using one or more keys defined in the in-toto layout. An example of this is: { \"key_id\": \"776a00e29f3559e0141b3b096f696abc6cfb0c657ab40f441132b345b0...\", \"key_type\": \"rsa\", \"public_key_value\": \"-----BEGIN PUBLIC KEY-----\\nMIIBojANBgkqhkiG9w0B...\", \"key_scheme\": \"rsassa-pss-sha256\" } The format for in-toto's key definition can be found in section 4.2 of the in-toto specification."]
pub struct SigningKey {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "key_id is an identifier for the signing key."]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyScheme")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the corresponding signature scheme. Eg: \"rsassa-pss-sha256\"."]
    pub key_scheme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field identifies the specific signing method. Eg: \"rsa\", \"ed25519\", and \"ecdsa\"."]
    pub key_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicKeyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the actual public key."]
    pub public_key_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source describes the location of the source used for the build."]
pub struct Source {
    #[serde(rename = "additionalContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
    pub additional_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceContext>>>,
    #[serde(rename = "artifactStorageSourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, the input binary artifacts for the build came from this location."]
    pub artifact_storage_source_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, the source code used for the build came from this location."]
    pub context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "fileHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
    pub file_hashes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<FileHashes>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory."]
pub struct SourceContext {
    #[serde(rename = "cloudRepo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
    pub cloud_repo: ::std::option::Option<::std::boxed::Box<CloudRepoSourceContext>>,
    #[serde(rename = "gerrit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a Gerrit project."]
    pub gerrit: ::std::option::Option<::std::boxed::Box<GerritSourceContext>>,
    #[serde(rename = "git")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
    pub git: ::std::option::Option<::std::boxed::Box<GitSourceContext>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels with user defined metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
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
#[doc = "Request message for `TestIamPermissions` method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Version contains structured information about the version of a package."]
pub struct Version {
    #[serde(rename = "epoch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to correct mistakes in the version numbering scheme."]
    pub epoch: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "inclusive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range."]
    pub inclusive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
    pub kind: ::std::option::Option<VersionKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required only when version kind is NORMAL. The main part of the version name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The iteration of the package build from the above version."]
    pub revision: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
pub enum VersionKindEnum {
    #[serde(rename = "VERSION_KIND_UNSPECIFIED")]
    #[doc = "Unknown."]
    VersionKindUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "A standard package version."]
    Normal,
    #[serde(rename = "MINIMUM")]
    #[doc = "A special version representing negative infinity."]
    Minimum,
    #[serde(rename = "MAXIMUM")]
    #[doc = "A special version representing positive infinity."]
    Maximum,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Vulnerability provides metadata about a security vulnerability in a Note."]
pub struct Vulnerability {
    #[serde(rename = "cvssScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CVSS score for this vulnerability."]
    pub cvss_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "cvssV3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full description of the CVSSv3."]
    pub cvss_v3: ::std::option::Option<::std::boxed::Box<CvsSv3>>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All information about the package to specifically identify this vulnerability. One entry per (version range and cpe_uri) the package vulnerability has manifested in."]
    pub details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Detail>>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Note provider assigned impact of the vulnerability."]
    pub severity: ::std::option::Option<VulnerabilitySeverityEnum>,
    #[serde(rename = "sourceUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker."]
    pub source_update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "windowsDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Windows details get their own format because the information format and model don't match a normal detail. Specifically Windows updates are done as patches, thus Windows vulnerabilities really are a missing package, rather than a package being at an incorrect version."]
    pub windows_details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WindowsDetail>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Note provider assigned impact of the vulnerability."]
pub enum VulnerabilitySeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The location of the vulnerability."]
pub struct VulnerabilityLocation {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) format. Examples include distro or storage location for vulnerable jar."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The package being described."]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of the package being described."]
    pub version: ::std::option::Option<::std::boxed::Box<Version>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A summary of how many vulnerability occurrences there are per resource and severity type."]
pub struct VulnerabilityOccurrencesSummary {
    #[serde(rename = "counts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A listing by resource of the number of fixable and total vulnerabilities."]
    pub counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FixableTotalByDigest>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowsDetail {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the vulnerability."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixingKbs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The names of the KBs which have hotfixes to mitigate this vulnerability. Note that there may be multiple hotfixes (and thus multiple KBs) that mitigate a given vulnerability. Currently any listed kb's presence is considered a fix."]
    pub fixing_kbs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KnowledgeBase>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the vulnerability."]
    pub name: ::std::option::Option<::std::string::String>,
}
