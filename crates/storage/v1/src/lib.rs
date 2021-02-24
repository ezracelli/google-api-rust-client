#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A bucket."]
pub struct Bucket {
    #[serde(rename = "acl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access controls on the bucket."]
    pub acl: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BucketAccessControl>>>,
    #[serde(rename = "billing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's billing configuration."]
    pub billing: ::std::option::Option<BucketBilling>,
    #[serde(rename = "cors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub cors: ::std::option::Option<::std::vec::Vec<BucketCors>>,
    #[serde(rename = "defaultEventBasedHold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed."]
    pub default_event_based_hold: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "defaultObjectAcl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default access controls to apply to new objects when no ACL is provided."]
    pub default_object_acl:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encryption configuration for a bucket."]
    pub encryption: ::std::option::Option<BucketEncryption>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for the bucket."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iamConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's IAM configuration."]
    pub iam_configuration: ::std::option::Option<BucketIamConfiguration>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the bucket. For buckets, the id and name properties are the same."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "bucket_defaults :: kind")]
    #[doc = "The kind of item this is. For buckets, this is always storage#bucket."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-provided labels, in key/value pairs."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "lifecycle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's lifecycle configuration. See lifecycle management for more information."]
    pub lifecycle: ::std::option::Option<BucketLifecycle>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the bucket location."]
    pub location_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs."]
    pub logging: ::std::option::Option<BucketLogging>,
    #[serde(rename = "metageneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metadata generation of this bucket."]
    pub metageneration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the bucket."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The owner of the bucket. This is always the project team's owner group."]
    pub owner: ::std::option::Option<BucketOwner>,
    #[serde(rename = "projectNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project number of the project the bucket belongs to."]
    pub project_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "retentionPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error."]
    pub retention_policy: ::std::option::Option<BucketRetentionPolicy>,
    #[serde(rename = "satisfiesPZS")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reserved for future use."]
    pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of this bucket."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes."]
    pub storage_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time of the bucket in RFC 3339 format."]
    pub time_created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The modification time of the bucket in RFC 3339 format."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versioning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's versioning configuration."]
    pub versioning: ::std::option::Option<BucketVersioning>,
    #[serde(rename = "website")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information."]
    pub website: ::std::option::Option<BucketWebsite>,
    #[serde(rename = "zoneAffinity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zone or zones from which the bucket is intended to use zonal quota. Requests for data from outside the specified affinities are still allowed but won't be able to use zonal quota. The zone or zones need to be within the bucket location otherwise the requests will fail with a 400 Bad Request response."]
    pub zone_affinity: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod bucket_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#bucket")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's billing configuration."]
pub struct BucketBilling {
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When set to true, Requester Pays is enabled for this bucket."]
    pub requester_pays: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BucketCors {
    #[serde(rename = "maxAgeSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses."]
    pub max_age_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
    pub method: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
    pub origin: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "responseHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
    pub response_header: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encryption configuration for a bucket."]
pub struct BucketEncryption {
    #[serde(rename = "defaultKmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified."]
    pub default_kms_key_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's IAM configuration."]
pub struct BucketIamConfiguration {
    #[serde(rename = "bucketPolicyOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature."]
    pub bucket_policy_only: ::std::option::Option<BucketIamConfigurationBucketPolicyOnly>,
    #[serde(rename = "publicAccessPrevention")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's Public Access Prevention configuration. Currently, 'unspecified' and 'enforced' are supported."]
    pub public_access_prevention: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniformBucketLevelAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bucket's uniform bucket-level access configuration."]
    pub uniform_bucket_level_access:
        ::std::option::Option<BucketIamConfigurationUniformBucketLevelAccess>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature."]
pub struct BucketIamConfigurationBucketPolicyOnly {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lockedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable."]
    pub locked_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's uniform bucket-level access configuration."]
pub struct BucketIamConfigurationUniformBucketLevelAccess {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lockedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable."]
    pub locked_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's lifecycle configuration. See lifecycle management for more information."]
pub struct BucketLifecycle {
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken."]
    pub rule: ::std::option::Option<::std::vec::Vec<BucketLifecycleRule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BucketLifecycleRule {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action to take."]
    pub action: ::std::option::Option<BucketLifecycleRuleAction>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition(s) under which the action will be taken."]
    pub condition: ::std::option::Option<BucketLifecycleRuleCondition>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The action to take."]
pub struct BucketLifecycleRuleAction {
    #[serde(rename = "storageClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target storage class. Required iff the type of the action is SetStorageClass."]
    pub storage_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the action. Currently, only Delete and SetStorageClass are supported."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The condition(s) under which the action will be taken."]
pub struct BucketLifecycleRuleCondition {
    #[serde(rename = "age")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Age of an object (in days). This condition is satisfied when an object reaches the specified age."]
    pub age: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "createdBefore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when an object is created before midnight of the specified date in UTC."]
    pub created_before: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customTimeBefore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when the custom time on an object is before this date in UTC."]
    pub custom_time_before: ::std::option::Option<::std::string::String>,
    #[serde(rename = "daysSinceCustomTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of days elapsed since the user-specified timestamp set on an object. The condition is satisfied if the days elapsed is at least this number. If no custom timestamp is specified on an object, the condition does not apply."]
    pub days_since_custom_time: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "daysSinceNoncurrentTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of days elapsed since the noncurrent timestamp of an object. The condition is satisfied if the days elapsed is at least this number. This condition is relevant only for versioned objects. The value of the field must be a nonnegative integer. If it's zero, the object version will become eligible for Lifecycle action as soon as it becomes noncurrent."]
    pub days_since_noncurrent_time: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "isLive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Relevant only for versioned objects. If the value is true, this condition matches live objects; if the value is false, it matches archived objects."]
    pub is_live: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "matchesPattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A regular expression that satisfies the RE2 syntax. This condition is satisfied when the name of the object matches the RE2 pattern. Note: This feature is currently in the \"Early Access\" launch stage and is only available to a whitelisted set of users; that means that this feature may be changed in backward-incompatible ways and that it is not guaranteed to be released."]
    pub matches_pattern: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchesStorageClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, and DURABLE_REDUCED_AVAILABILITY."]
    pub matches_storage_class: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "noncurrentTimeBefore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when the noncurrent time on an object is before this date in UTC. This condition is relevant only for versioned objects."]
    pub noncurrent_time_before: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numNewerVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object."]
    pub num_newer_versions: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs."]
pub struct BucketLogging {
    #[serde(rename = "logBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination bucket where the current bucket's logs should be placed."]
    pub log_bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logObjectPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A prefix for log object names."]
    pub log_object_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The owner of the bucket. This is always the project team's owner group."]
pub struct BucketOwner {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity, in the form project-owner-projectId."]
    pub entity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the entity."]
    pub entity_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error."]
pub struct BucketRetentionPolicy {
    #[serde(rename = "effectiveTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format."]
    pub effective_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isLocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Once locked, an object retention policy cannot be modified."]
    pub is_locked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes."]
    pub retention_period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's versioning configuration."]
pub struct BucketVersioning {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "While set to true, versioning is fully enabled for this bucket."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information."]
pub struct BucketWebsite {
    #[serde(rename = "mainPageSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the requested object path is missing, the service will ensure the path has a trailing '/', append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages."]
    pub main_page_suffix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notFoundPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result."]
    pub not_found_page: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An access-control entry."]
pub struct BucketAccessControl {
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the bucket."]
    pub bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain associated with the entity, if any."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address associated with the entity, if any."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
    pub entity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the entity, if any."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the access-control entry."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "bucket_access_control_defaults :: kind")]
    #[doc = "The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl."]
    pub kind: ::std::string::String,
    #[serde(rename = "projectTeam")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project team associated with the entity, if any."]
    pub project_team: ::std::option::Option<BucketAccessControlProjectTeam>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The access permission for the entity."]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to this access-control entry."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod bucket_access_control_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#bucketAccessControl")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The project team associated with the entity, if any."]
pub struct BucketAccessControlProjectTeam {
    #[serde(rename = "projectNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project number."]
    pub project_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The team."]
    pub team: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An access-control list."]
pub struct BucketAccessControls {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BucketAccessControl>>>,
    #[serde(rename = "kind")]
    #[serde(default = "bucket_access_controls_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls."]
    pub kind: ::std::string::String,
}
mod bucket_access_controls_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#bucketAccessControls")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of buckets."]
pub struct Buckets {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bucket>>>,
    #[serde(rename = "kind")]
    #[serde(default = "buckets_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of buckets, this is always storage#buckets."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod buckets_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#buckets")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An notification channel used to watch for resource changes."]
pub struct Channel {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address where notifications are delivered for this channel."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
    pub expiration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A UUID or similar unique string that identifies this channel."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "channel_defaults :: kind")]
    #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
    pub payload: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A version-specific identifier for the watched resource."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delivery mechanism used for this channel."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("api#channel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Compose request."]
pub struct ComposeRequest {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties of the resulting object."]
    pub destination: ::std::option::Option<::std::boxed::Box<Object>>,
    #[serde(rename = "kind")]
    #[serde(default = "compose_request_defaults :: kind")]
    #[doc = "The kind of item this is."]
    pub kind: ::std::string::String,
    #[serde(rename = "sourceObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of source objects that will be concatenated into a single object."]
    pub source_objects: ::std::option::Option<::std::vec::Vec<ComposeRequestSourceObjects>>,
}
mod compose_request_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#composeRequest")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposeRequestSourceObjects {
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generation of this object to use as the source."]
    pub generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source object's name. All source objects must reside in the same bucket."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectPreconditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions that must be met for this operation to execute."]
    pub object_preconditions: ::std::option::Option<ComposeRequestSourceObjectsObjectPreconditions>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Conditions that must be met for this operation to execute."]
pub struct ComposeRequestSourceObjectsObjectPreconditions {
    #[serde(rename = "ifGenerationMatch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail."]
    pub if_generation_match: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an expression text. Example: title: \"User account presence\" description: \"Determines whether the request has a user account\" expression: \"size(request.user) > 0\""]
pub struct Expr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template to produce a JSON-style HMAC Key resource for Create responses."]
pub struct HmacKey {
    #[serde(rename = "kind")]
    #[serde(default = "hmac_key_defaults :: kind")]
    #[doc = "The kind of item this is. For HMAC keys, this is always storage#hmacKey."]
    pub kind: ::std::string::String,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key metadata."]
    pub metadata: ::std::option::Option<::std::boxed::Box<HmacKeyMetadata>>,
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HMAC secret key material."]
    pub secret: ::std::option::Option<::std::string::String>,
}
mod hmac_key_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#hmacKey")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template to produce a JSON-style HMAC Key metadata resource."]
pub struct HmacKeyMetadata {
    #[serde(rename = "accessId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the HMAC Key."]
    pub access_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for the HMAC key."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the HMAC key, including the Project ID and the Access ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "hmac_key_metadata_defaults :: kind")]
    #[doc = "The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata."]
    pub kind: ::std::string::String,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID owning the service account to which the key authenticates."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to this resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the key's associated service account."]
    pub service_account_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time of the HMAC key in RFC 3339 format."]
    pub time_created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last modification time of the HMAC key metadata in RFC 3339 format."]
    pub updated: ::std::option::Option<::std::string::String>,
}
mod hmac_key_metadata_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#hmacKeyMetadata")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of hmacKeys."]
pub struct HmacKeysMetadata {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HmacKeyMetadata>>>,
    #[serde(rename = "kind")]
    #[serde(default = "hmac_keys_metadata_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of hmacKeys, this is always storage#hmacKeysMetadata."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod hmac_keys_metadata_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#hmacKeysMetadata")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A subscription to receive Google PubSub notifications."]
pub struct Notification {
    #[serde(rename = "custom_attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription."]
    pub custom_attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for this subscription notification."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "event_types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, only send notifications about listed event types. If empty, sent notifications for all event types."]
    pub event_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the notification."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "notification_defaults :: kind")]
    #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
    pub kind: ::std::string::String,
    #[serde(rename = "object_name_prefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, only apply this notification configuration to object names that begin with this prefix."]
    pub object_name_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "payload_format")]
    #[serde(default = "notification_defaults :: payload_format")]
    #[doc = "The desired content of the Payload."]
    pub payload_format: ::std::string::String,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical URL of this notification."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'"]
    pub topic: ::std::option::Option<::std::string::String>,
}
mod notification_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#notification")
    }
    pub fn payload_format() -> ::std::string::String {
        String::from("JSON_API_V1")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of notification subscriptions."]
pub struct Notifications {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Notification>>>,
    #[serde(rename = "kind")]
    #[serde(default = "notifications_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of notifications, this is always storage#notifications."]
    pub kind: ::std::string::String,
}
mod notifications_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#notifications")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object."]
pub struct Object {
    #[serde(rename = "acl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access controls on the object."]
    pub acl: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the bucket containing this object."]
    pub bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cacheControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600."]
    pub cache_control: ::std::option::Option<::std::string::String>,
    #[serde(rename = "componentCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of underlying components that make up this object. Components are accumulated by compose operations."]
    pub component_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "contentDisposition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content-Disposition of the object data."]
    pub content_disposition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content-Encoding of the object data."]
    pub content_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content-Language of the object data."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crc32c")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices."]
    pub crc32c: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A timestamp in RFC 3339 format specified by the user for an object."]
    pub custom_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerEncryption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata of customer-supplied encryption key, if the object is encrypted by such a key."]
    pub customer_encryption: ::std::option::Option<ObjectCustomerEncryption>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for the object."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventBasedHold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false."]
    pub event_based_hold: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content generation of this object. Used for object versioning."]
    pub generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the object, including the bucket name, object name, and generation number."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "object_defaults :: kind")]
    #[doc = "The kind of item this is. For objects, this is always storage#object."]
    pub kind: ::std::string::String,
    #[serde(rename = "kmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Not currently supported. Specifying the parameter causes the request to fail with status code 400 - Bad Request."]
    pub kms_key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices."]
    pub md5_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Media download link."]
    pub media_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-provided metadata, in key/value pairs."]
    pub metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "metageneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object."]
    pub metageneration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the object. Required if not specified by URL parameter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The owner of the object. This will always be the uploader of the object."]
    pub owner: ::std::option::Option<ObjectOwner>,
    #[serde(rename = "retentionExpirationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold)."]
    pub retention_expiration_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to this object."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content-Length of the data in bytes."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Storage class of the object."]
    pub storage_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "temporaryHold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object."]
    pub temporary_hold: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time of the object in RFC 3339 format."]
    pub time_created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeDeleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted."]
    pub time_deleted: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeStorageClassUpdated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated."]
    pub time_storage_class_updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The modification time of the object metadata in RFC 3339 format."]
    pub updated: ::std::option::Option<::std::string::String>,
}
mod object_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#object")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata of customer-supplied encryption key, if the object is encrypted by such a key."]
pub struct ObjectCustomerEncryption {
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encryption algorithm."]
    pub encryption_algorithm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keySha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "SHA256 hash value of the encryption key."]
    pub key_sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The owner of the object. This will always be the uploader of the object."]
pub struct ObjectOwner {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity, in the form user-userId."]
    pub entity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the entity."]
    pub entity_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An access-control entry."]
pub struct ObjectAccessControl {
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the bucket."]
    pub bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain associated with the entity, if any."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address associated with the entity, if any."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
    pub entity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID for the entity, if any."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content generation of the object, if applied to an object."]
    pub generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the access-control entry."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "object_access_control_defaults :: kind")]
    #[doc = "The kind of item this is. For object access control entries, this is always storage#objectAccessControl."]
    pub kind: ::std::string::String,
    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the object, if applied to an object."]
    pub object: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectTeam")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project team associated with the entity, if any."]
    pub project_team: ::std::option::Option<ObjectAccessControlProjectTeam>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The access permission for the entity."]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to this access-control entry."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
mod object_access_control_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#objectAccessControl")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The project team associated with the entity, if any."]
pub struct ObjectAccessControlProjectTeam {
    #[serde(rename = "projectNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project number."]
    pub project_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The team."]
    pub team: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An access-control list."]
pub struct ObjectAccessControls {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
    #[serde(rename = "kind")]
    #[serde(default = "object_access_controls_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls."]
    pub kind: ::std::string::String,
}
mod object_access_controls_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#objectAccessControls")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of objects."]
pub struct Objects {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Object>>>,
    #[serde(rename = "kind")]
    #[serde(default = "objects_defaults :: kind")]
    #[doc = "The kind of item this is. For lists of objects, this is always storage#objects."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prefixes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter."]
    pub prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod objects_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#objects")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A bucket/object IAM policy."]
pub struct Policy {
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An association between a role, which comes with a set of permissions, and members who may assume that role."]
    pub bindings: ::std::option::Option<::std::vec::Vec<PolicyBindings>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP 1.1  Entity tag for the policy."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "policy_defaults :: kind")]
    #[doc = "The kind of item this is. For policies, this is always storage#policy. This field is ignored on input."]
    pub kind: ::std::string::String,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the resource to which this policy belongs. Will be of the form projects/_/buckets/bucket for buckets, and projects/_/buckets/bucket/objects/object for objects. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IAM policy format version."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
mod policy_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#policy")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyBindings {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. NOTE: an unsatisfied condition will not allow user access via current binding. Different bindings, including their conditions, are examined independently."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of identifiers for members who may assume the provided role. Recognized identifiers are as follows:  \n- allUsers — A special identifier that represents anyone on the internet; with or without a Google account.  \n- allAuthenticatedUsers — A special identifier that represents anyone who is authenticated with a Google account or a service account.  \n- user:emailid — An email address that represents a specific account. For example, user:alice@gmail.com or user:joe@example.com.  \n- serviceAccount:emailid — An email address that represents a service account. For example,  serviceAccount:my-other-app@appspot.gserviceaccount.com .  \n- group:emailid — An email address that represents a Google group. For example, group:admins@example.com.  \n- domain:domain — A Google Apps domain name that represents all the users of that domain. For example, domain:google.com or domain:example.com.  \n- projectOwner:projectid — Owners of the given project. For example, projectOwner:my-example-project  \n- projectEditor:projectid — Editors of the given project. For example, projectEditor:my-example-project  \n- projectViewer:projectid — Viewers of the given project. For example, projectViewer:my-example-project"]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role to which members belong. Two types of roles are supported: new IAM roles, which grant permissions that do not map directly to those provided by ACLs, and legacy IAM roles, which do map directly to ACL permissions. All roles are of the format roles/storage.specificRole.\nThe new IAM roles are:  \n- roles/storage.admin — Full control of Google Cloud Storage resources.  \n- roles/storage.objectViewer — Read-Only access to Google Cloud Storage objects.  \n- roles/storage.objectCreator — Access to create objects in Google Cloud Storage.  \n- roles/storage.objectAdmin — Full control of Google Cloud Storage objects.   The legacy IAM roles are:  \n- roles/storage.legacyObjectReader — Read-only access to objects without listing. Equivalent to an ACL entry on an object with the READER role.  \n- roles/storage.legacyObjectOwner — Read/write access to existing objects without listing. Equivalent to an ACL entry on an object with the OWNER role.  \n- roles/storage.legacyBucketReader — Read access to buckets with object listing. Equivalent to an ACL entry on a bucket with the READER role.  \n- roles/storage.legacyBucketWriter — Read access to buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the WRITER role.  \n- roles/storage.legacyBucketOwner — Read and write access to existing buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the OWNER role."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A rewrite response."]
pub struct RewriteResponse {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "rewrite_response_defaults :: kind")]
    #[doc = "The kind of item this is."]
    pub kind: ::std::string::String,
    #[serde(rename = "objectSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total size of the object being copied in bytes. This property is always present in the response."]
    pub object_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes."]
    pub resource: ::std::option::Option<::std::boxed::Box<Object>>,
    #[serde(rename = "rewriteToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy."]
    pub rewrite_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalBytesRewritten")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response."]
    pub total_bytes_rewritten: ::std::option::Option<::std::string::String>,
}
mod rewrite_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#rewriteResponse")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A subscription to receive Google PubSub notifications."]
pub struct ServiceAccount {
    #[serde(rename = "email_address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the notification."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "service_account_defaults :: kind")]
    #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
    pub kind: ::std::string::String,
}
mod service_account_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#serviceAccount")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A storage.(buckets|objects).testIamPermissions response."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "kind")]
    #[serde(default = "test_iam_permissions_response_defaults :: kind")]
    #[doc = "The kind of item this is."]
    pub kind: ::std::string::String,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets or objects. The supported permissions are as follows:  \n- storage.buckets.delete — Delete bucket.  \n- storage.buckets.get — Read bucket metadata.  \n- storage.buckets.getIamPolicy — Read bucket IAM policy.  \n- storage.buckets.create — Create bucket.  \n- storage.buckets.list — List buckets.  \n- storage.buckets.setIamPolicy — Update bucket IAM policy.  \n- storage.buckets.update — Update bucket metadata.  \n- storage.objects.delete — Delete object.  \n- storage.objects.get — Read object data and metadata.  \n- storage.objects.getIamPolicy — Read object IAM policy.  \n- storage.objects.create — Create object.  \n- storage.objects.list — List objects.  \n- storage.objects.setIamPolicy — Update object IAM policy.  \n- storage.objects.update — Update object metadata."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod test_iam_permissions_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("storage#testIamPermissionsResponse")
    }
}
