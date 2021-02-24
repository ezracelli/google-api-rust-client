#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Announcement created by a teacher for students of the course"]
pub struct Announcement {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assigneeMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`."]
    pub assignee_mode: ::std::option::Option<AnnouncementAssigneeModeEnum>,
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this announcement was created. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the user that created the announcement. Read-only."]
    pub creator_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Classroom-assigned identifier of this announcement, unique per course. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "individualStudentsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the announcement."]
    pub individual_students_options:
        ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
    #[serde(rename = "materials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional materials. Announcements must have no more than 20 material items."]
    pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
    #[serde(rename = "scheduledTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional timestamp when this announcement is scheduled to be published."]
    pub scheduled_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this announcement. If unspecified, the default state is `DRAFT`."]
    pub state: ::std::option::Option<AnnouncementStateEnum>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the most recent change to this announcement. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`."]
pub enum AnnouncementAssigneeModeEnum {
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    #[doc = "No mode specified. This is never returned."]
    AssigneeModeUnspecified,
    #[serde(rename = "ALL_STUDENTS")]
    #[doc = "All students can see the item. This is the default state."]
    AllStudents,
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    #[doc = "A subset of the students can see the item."]
    IndividualStudents,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of this announcement. If unspecified, the default state is `DRAFT`."]
pub enum AnnouncementStateEnum {
    #[serde(rename = "ANNOUNCEMENT_STATE_UNSPECIFIED")]
    #[doc = "No state specified. This is never returned."]
    AnnouncementStateUnspecified,
    #[serde(rename = "PUBLISHED")]
    #[doc = "Status for announcement that has been published. This is the default state."]
    Published,
    #[serde(rename = "DRAFT")]
    #[doc = "Status for an announcement that is not yet published. Announcement in this state is visible only to course teachers and domain administrators."]
    Draft,
    #[serde(rename = "DELETED")]
    #[doc = "Status for announcement that was published but is now deleted. Announcement in this state is visible only to course teachers and domain administrators. Announcement in this state is deleted after some time."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional details for assignments."]
pub struct Assignment {
    #[serde(rename = "studentWorkFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Drive folder where attachments from student submissions are placed. This is only populated for course teachers and administrators."]
    pub student_work_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Student work for an assignment."]
pub struct AssignmentSubmission {
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attachments added by the student. Drive files that correspond to materials with a share mode of STUDENT_COPY may not exist yet if the student has not accessed the assignment in Classroom. Some attachment metadata is only populated if the requesting user has permission to access it. Identifier and alternate_link fields are always available, but others (for example, title) may not be."]
    pub attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Attachment added to student assignment work. When creating attachments, setting the `form` field is not supported."]
pub struct Attachment {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Drive file attachment."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Forms attachment."]
    pub form: ::std::option::Option<::std::boxed::Box<Form>>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link attachment."]
    pub link: ::std::option::Option<::std::boxed::Box<Link>>,
    #[serde(rename = "youTubeVideo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Youtube video attachment."]
    pub you_tube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a Cloud Pub/Sub topic. To register for notifications, the owner of the topic must grant `classroom-notifications@system.gserviceaccount.com` the `projects.topics.publish` permission."]
pub struct CloudPubsubTopic {
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `name` field of a Cloud Pub/Sub [Topic](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic)."]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Course in Classroom."]
pub struct Course {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute link to this course in the Classroom web UI. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "calendarId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. Read-only."]
    pub calendar_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "courseGroupEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only."]
    pub course_group_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "courseMaterialSets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets of materials that appear on the \"about\" page of this course. Read-only."]
    pub course_material_sets:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseMaterialSet>>>,
    #[serde(rename = "courseState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the course. If unspecified, the default state is `PROVISIONED`."]
    pub course_state: ::std::option::Option<CourseCourseStateEnum>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of the course. Specifying this field in a course update mask results in an error. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional description. For example, \"We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!\" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "descriptionHeading")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional heading for the description. For example, \"Welcome to 10th Grade Biology.\" If set, this field must be a valid UTF-8 string and no longer than 3600 characters."]
    pub description_heading: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enrollmentCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only."]
    pub enrollment_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "guardiansEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not guardian notifications are enabled for this course. Read-only."]
    pub guardians_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The `id` is still assigned by Classroom and cannot be updated after the course is created. Specifying this field in a course update mask results in an error."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the course. For example, \"10th Grade Biology\". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user This must be set in a create request. Admins can also specify this field in a patch course request to transfer ownership. In other contexts, it is read-only."]
    pub owner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "room")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional room location. For example, \"301\". If set, this field must be a valid UTF-8 string and no longer than 650 characters."]
    pub room: ::std::option::Option<::std::string::String>,
    #[serde(rename = "section")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Section of the course. For example, \"Period 2\". If set, this field must be a valid UTF-8 string and no longer than 2800 characters."]
    pub section: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teacherFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only."]
    pub teacher_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
    #[serde(rename = "teacherGroupEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only."]
    pub teacher_group_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of the course. If unspecified, the default state is `PROVISIONED`."]
pub enum CourseCourseStateEnum {
    #[serde(rename = "COURSE_STATE_UNSPECIFIED")]
    #[doc = "No course state. No returned Course message will use this value."]
    CourseStateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The course is active."]
    Active,
    #[serde(rename = "ARCHIVED")]
    #[doc = "The course has been archived. You cannot modify it except to change it to a different state."]
    Archived,
    #[serde(rename = "PROVISIONED")]
    #[doc = "The course has been created, but not yet activated. It is accessible by the primary teacher and domain administrators, who may modify it or change it to the `ACTIVE` or `DECLINED` states. A course may only be changed to `PROVISIONED` if it is in the `DECLINED` state."]
    Provisioned,
    #[serde(rename = "DECLINED")]
    #[doc = "The course has been created, but declined. It is accessible by the course owner and domain administrators, though it will not be displayed in the web UI. You cannot modify the course except to change it to the `PROVISIONED` state. A course may only be changed to `DECLINED` if it is in the `PROVISIONED` state."]
    Declined,
    #[serde(rename = "SUSPENDED")]
    #[doc = "The course has been suspended. You cannot modify the course, and only the user identified by the `owner_id` can view the course. A course may be placed in this state if it potentially violates the Terms of Service."]
    Suspended,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative identifier for a course. An alias uniquely identifies a course. It must be unique within one of the following scopes: * domain: A domain-scoped alias is visible to all users within the alias creator's domain and can be created only by a domain admin. A domain-scoped alias is often used when a course has an identifier external to Classroom. * project: A project-scoped alias is visible to any request from an application using the Developer Console project ID that created the alias and can be created by any project. A project-scoped alias is often used when an application has alternative identifiers. A random value can also be used to avoid duplicate courses in the event of transmission failures, as retrying a request will return `ALREADY_EXISTS` if a previous one has succeeded."]
pub struct CourseAlias {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Alias string. The format of the string indicates the desired alias scoping. * `d:` indicates a domain-scoped alias. Example: `d:math_101` * `p:` indicates a project-scoped alias. Example: `p:abc123` This field has a maximum length of 256 characters."]
    pub alias: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A material attached to a course as part of a material set."]
pub struct CourseMaterial {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Drive file attachment."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Forms attachment."]
    pub form: ::std::option::Option<::std::boxed::Box<Form>>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link atatchment."]
    pub link: ::std::option::Option<::std::boxed::Box<Link>>,
    #[serde(rename = "youTubeVideo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Youtube video attachment."]
    pub you_tube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of materials that appears on the \"About\" page of the course. These materials might include a syllabus, schedule, or other background information relating to the course as a whole."]
pub struct CourseMaterialSet {
    #[serde(rename = "materials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Materials attached to this set."]
    pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseMaterial>>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title for this set."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`."]
pub struct CourseRosterChangesInfo {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `course_id` of the course to subscribe to roster changes for."]
    pub course_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Course work created by a teacher for students of the course."]
pub struct CourseWork {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assigneeMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`."]
    pub assignee_mode: ::std::option::Option<CourseWorkAssigneeModeEnum>,
    #[serde(rename = "assignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only."]
    pub assignment: ::std::option::Option<::std::boxed::Box<Assignment>>,
    #[serde(rename = "associatedWithDeveloper")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only."]
    pub associated_with_developer: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this course work was created. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the user that created the coursework. Read-only."]
    pub creator_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified."]
    pub due_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "dueTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified."]
    pub due_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Classroom-assigned identifier of this course work, unique per course. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "individualStudentsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field are assigned the coursework."]
    pub individual_students_options:
        ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
    #[serde(rename = "materials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional materials. CourseWork must have no more than 20 material items."]
    pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
    #[serde(rename = "maxPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value."]
    pub max_points: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "multipleChoiceQuestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. For write operations, this field must be specified when creating course work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be set otherwise."]
    pub multiple_choice_question: ::std::option::Option<::std::boxed::Box<MultipleChoiceQuestion>>,
    #[serde(rename = "scheduledTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional timestamp when this course work is scheduled to be published."]
    pub scheduled_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this course work. If unspecified, the default state is `DRAFT`."]
    pub state: ::std::option::Option<CourseWorkStateEnum>,
    #[serde(rename = "submissionModificationMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`."]
    pub submission_modification_mode:
        ::std::option::Option<CourseWorkSubmissionModificationModeEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the topic that this coursework is associated with. Must match an existing topic in the course."]
    pub topic_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the most recent change to this course work. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this course work. The type is set when the course work is created and cannot be changed."]
    pub work_type: ::std::option::Option<CourseWorkWorkTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`."]
pub enum CourseWorkAssigneeModeEnum {
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    #[doc = "No mode specified. This is never returned."]
    AssigneeModeUnspecified,
    #[serde(rename = "ALL_STUDENTS")]
    #[doc = "All students can see the item. This is the default state."]
    AllStudents,
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    #[doc = "A subset of the students can see the item."]
    IndividualStudents,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of this course work. If unspecified, the default state is `DRAFT`."]
pub enum CourseWorkStateEnum {
    #[serde(rename = "COURSE_WORK_STATE_UNSPECIFIED")]
    #[doc = "No state specified. This is never returned."]
    CourseWorkStateUnspecified,
    #[serde(rename = "PUBLISHED")]
    #[doc = "Status for work that has been published. This is the default state."]
    Published,
    #[serde(rename = "DRAFT")]
    #[doc = "Status for work that is not yet published. Work in this state is visible only to course teachers and domain administrators."]
    Draft,
    #[serde(rename = "DELETED")]
    #[doc = "Status for work that was published but is now deleted. Work in this state is visible only to course teachers and domain administrators. Work in this state is deleted after some time."]
    Deleted,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`."]
pub enum CourseWorkSubmissionModificationModeEnum {
    #[serde(rename = "SUBMISSION_MODIFICATION_MODE_UNSPECIFIED")]
    #[doc = "No modification mode specified. This is never returned."]
    SubmissionModificationModeUnspecified,
    #[serde(rename = "MODIFIABLE_UNTIL_TURNED_IN")]
    #[doc = "Submissions can be modified before being turned in."]
    ModifiableUntilTurnedIn,
    #[serde(rename = "MODIFIABLE")]
    #[doc = "Submissions can be modified at any time."]
    Modifiable,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of this course work. The type is set when the course work is created and cannot be changed."]
pub enum CourseWorkWorkTypeEnum {
    #[serde(rename = "COURSE_WORK_TYPE_UNSPECIFIED")]
    #[doc = "No work type specified. This is never returned."]
    CourseWorkTypeUnspecified,
    #[serde(rename = "ASSIGNMENT")]
    #[doc = "An assignment."]
    Assignment,
    #[serde(rename = "SHORT_ANSWER_QUESTION")]
    #[doc = "A short answer question."]
    ShortAnswerQuestion,
    #[serde(rename = "MULTIPLE_CHOICE_QUESTION")]
    #[doc = "A multiple-choice question."]
    MultipleChoiceQuestion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`."]
pub struct CourseWorkChangesInfo {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `course_id` of the course to subscribe to work changes for."]
    pub course_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Course work material created by a teacher for students of the course"]
pub struct CourseWorkMaterial {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assigneeMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`."]
    pub assignee_mode: ::std::option::Option<CourseWorkMaterialAssigneeModeEnum>,
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when this course work material was created. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the user that created the course work material. Read-only."]
    pub creator_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Classroom-assigned identifier of this course work material, unique per course. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "individualStudentsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the course work material."]
    pub individual_students_options:
        ::std::option::Option<::std::boxed::Box<IndividualStudentsOptions>>,
    #[serde(rename = "materials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional materials. A course work material must have no more than 20 material items."]
    pub materials: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Material>>>,
    #[serde(rename = "scheduledTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional timestamp when this course work material is scheduled to be published."]
    pub scheduled_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this course work material. If unspecified, the default state is `DRAFT`."]
    pub state: ::std::option::Option<CourseWorkMaterialStateEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the topic that this course work material is associated with. Must match an existing topic in the course."]
    pub topic_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the most recent change to this course work material. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`."]
pub enum CourseWorkMaterialAssigneeModeEnum {
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    #[doc = "No mode specified. This is never returned."]
    AssigneeModeUnspecified,
    #[serde(rename = "ALL_STUDENTS")]
    #[doc = "All students can see the item. This is the default state."]
    AllStudents,
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    #[doc = "A subset of the students can see the item."]
    IndividualStudents,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of this course work material. If unspecified, the default state is `DRAFT`."]
pub enum CourseWorkMaterialStateEnum {
    #[serde(rename = "COURSEWORK_MATERIAL_STATE_UNSPECIFIED")]
    #[doc = "No state specified. This is never returned."]
    CourseworkMaterialStateUnspecified,
    #[serde(rename = "PUBLISHED")]
    #[doc = "Status for course work material that has been published. This is the default state."]
    Published,
    #[serde(rename = "DRAFT")]
    #[doc = "Status for an course work material that is not yet published. Course work material in this state is visible only to course teachers and domain administrators."]
    Draft,
    #[serde(rename = "DELETED")]
    #[doc = "Status for course work material that was published but is now deleted. Course work material in this state is visible only to course teachers and domain administrators. Course work material in this state is deleted after some time."]
    Deleted,
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
#[doc = "Representation of a Google Drive file."]
pub struct DriveFile {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL that can be used to access the Drive item. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Drive API resource ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a thumbnail image of the Drive item. Read-only."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the Drive item. Read-only."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a Google Drive folder."]
pub struct DriveFolder {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL that can be used to access the Drive folder. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Drive API resource ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the Drive folder. Read-only."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A class of notifications that an application can register to receive. For example: \"all roster changes for a domain\"."]
pub struct Feed {
    #[serde(rename = "courseRosterChangesInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`. This field must be specified if `feed_type` is `COURSE_ROSTER_CHANGES`."]
    pub course_roster_changes_info:
        ::std::option::Option<::std::boxed::Box<CourseRosterChangesInfo>>,
    #[serde(rename = "courseWorkChangesInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`. This field must be specified if `feed_type` is `COURSE_WORK_CHANGES`."]
    pub course_work_changes_info: ::std::option::Option<::std::boxed::Box<CourseWorkChangesInfo>>,
    #[serde(rename = "feedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of feed."]
    pub feed_type: ::std::option::Option<FeedFeedTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of feed."]
pub enum FeedFeedTypeEnum {
    #[serde(rename = "FEED_TYPE_UNSPECIFIED")]
    #[doc = "Should never be returned or provided."]
    FeedTypeUnspecified,
    #[serde(rename = "DOMAIN_ROSTER_CHANGES")]
    #[doc = "All roster changes for a particular domain. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation."]
    DomainRosterChanges,
    #[serde(rename = "COURSE_ROSTER_CHANGES")]
    #[doc = "All roster changes for a particular course. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation."]
    CourseRosterChanges,
    #[serde(rename = "COURSE_WORK_CHANGES")]
    #[doc = "All course work activity for a particular course. Notifications will be generated when a CourseWork or StudentSubmission object is created or modified. No notification will be generated when a StudentSubmission object is created in connection with the creation or modification of its parent CourseWork object (but a notification will be generated for that CourseWork object's creation or modification)."]
    CourseWorkChanges,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Forms item."]
pub struct Form {
    #[serde(rename = "formUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the form."]
    pub form_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "responseUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the form responses document. Only set if respsonses have been recorded and only when the requesting user is an editor of the form. Read-only."]
    pub response_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a thumbnail image of the Form. Read-only."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the Form. Read-only."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Global user permission description."]
pub struct GlobalPermission {
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Permission value."]
    pub permission: ::std::option::Option<GlobalPermissionPermissionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Permission value."]
pub enum GlobalPermissionPermissionEnum {
    #[serde(rename = "PERMISSION_UNSPECIFIED")]
    #[doc = "No permission is specified. This is not returned and is not a valid value."]
    PermissionUnspecified,
    #[serde(rename = "CREATE_COURSE")]
    #[doc = "User is permitted to create a course."]
    CreateCourse,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The history of each grade on this submission."]
pub struct GradeHistory {
    #[serde(rename = "actorUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The teacher who made the grade change."]
    pub actor_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gradeChangeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of grade change at this time in the submission grade history."]
    pub grade_change_type: ::std::option::Option<GradeHistoryGradeChangeTypeEnum>,
    #[serde(rename = "gradeTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the grade of the submission was changed."]
    pub grade_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The denominator of the grade at this time in the submission grade history."]
    pub max_points: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pointsEarned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numerator of the grade at this time in the submission grade history."]
    pub points_earned: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of grade change at this time in the submission grade history."]
pub enum GradeHistoryGradeChangeTypeEnum {
    #[serde(rename = "UNKNOWN_GRADE_CHANGE_TYPE")]
    #[doc = "No grade change type specified. This should never be returned."]
    UnknownGradeChangeType,
    #[serde(rename = "DRAFT_GRADE_POINTS_EARNED_CHANGE")]
    #[doc = "A change in the numerator of the draft grade."]
    DraftGradePointsEarnedChange,
    #[serde(rename = "ASSIGNED_GRADE_POINTS_EARNED_CHANGE")]
    #[doc = "A change in the numerator of the assigned grade."]
    AssignedGradePointsEarnedChange,
    #[serde(rename = "MAX_POINTS_CHANGE")]
    #[doc = "A change in the denominator of the grade."]
    MaxPointsChange,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Association between a student and a guardian of that student. The guardian may receive information about the student's course work."]
pub struct Guardian {
    #[serde(rename = "guardianId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the guardian."]
    pub guardian_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "guardianProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User profile for the guardian."]
    pub guardian_profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
    #[serde(rename = "invitedEmailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address to which the initial guardian invitation was sent. This field is only visible to domain administrators."]
    pub invited_email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "studentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the student to whom the guardian relationship applies."]
    pub student_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An invitation to become the guardian of a specified user, sent to a specified email address."]
pub struct GuardianInvitation {
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that this invitation was created. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "invitationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for this invitation. Read-only."]
    pub invitation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "invitedEmailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address that the invitation was sent to. This field is only visible to domain administrators."]
    pub invited_email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state that this invitation is in."]
    pub state: ::std::option::Option<GuardianInvitationStateEnum>,
    #[serde(rename = "studentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the student (in standard format)"]
    pub student_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state that this invitation is in."]
pub enum GuardianInvitationStateEnum {
    #[serde(rename = "GUARDIAN_INVITATION_STATE_UNSPECIFIED")]
    #[doc = "Should never be returned."]
    GuardianInvitationStateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The invitation is active and awaiting a response."]
    Pending,
    #[serde(rename = "COMPLETE")]
    #[doc = "The invitation is no longer active. It may have been accepted, declined, withdrawn or it may have expired."]
    Complete,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assignee details about a coursework/announcement. This field is set if and only if `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
pub struct IndividualStudentsOptions {
    #[serde(rename = "studentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifiers for the students that have access to the coursework/announcement."]
    pub student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An invitation to join a course."]
pub struct Invitation {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course to invite the user to."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier assigned by Classroom. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`."]
    pub role: ::std::option::Option<InvitationRoleEnum>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the invited user. When specified as a parameter of a request, this identifier can be set to one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`."]
pub enum InvitationRoleEnum {
    #[serde(rename = "COURSE_ROLE_UNSPECIFIED")]
    #[doc = "No course role."]
    CourseRoleUnspecified,
    #[serde(rename = "STUDENT")]
    #[doc = "Student in the course."]
    Student,
    #[serde(rename = "TEACHER")]
    #[doc = "Teacher of the course."]
    Teacher,
    #[serde(rename = "OWNER")]
    #[doc = "Owner of the course."]
    Owner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "URL item."]
pub struct Link {
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a thumbnail image of the target URL. Read-only."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the target of the URL. Read-only."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to link to. This must be a valid UTF-8 string containing between 1 and 2024 characters."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing course work."]
pub struct ListAnnouncementsResponse {
    #[serde(rename = "announcements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Announcement items that match the request."]
    pub announcements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Announcement>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing course aliases."]
pub struct ListCourseAliasesResponse {
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The course aliases."]
    pub aliases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseAlias>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing course work material."]
pub struct ListCourseWorkMaterialResponse {
    #[serde(rename = "courseWorkMaterial")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Course work material items that match the request."]
    pub course_work_material:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseWorkMaterial>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing course work."]
pub struct ListCourseWorkResponse {
    #[serde(rename = "courseWork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Course work items that match the request."]
    pub course_work: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CourseWork>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing courses."]
pub struct ListCoursesResponse {
    #[serde(rename = "courses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Courses that match the list request."]
    pub courses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Course>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing guardian invitations."]
pub struct ListGuardianInvitationsResponse {
    #[serde(rename = "guardianInvitations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Guardian invitations that matched the list request."]
    pub guardian_invitations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GuardianInvitation>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing guardians."]
pub struct ListGuardiansResponse {
    #[serde(rename = "guardians")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Guardians on this page of results that met the criteria specified in the request."]
    pub guardians: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Guardian>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing invitations."]
pub struct ListInvitationsResponse {
    #[serde(rename = "invitations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Invitations that match the list request."]
    pub invitations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Invitation>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing student submissions."]
pub struct ListStudentSubmissionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "studentSubmissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Student work that matches the request."]
    pub student_submissions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StudentSubmission>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing students."]
pub struct ListStudentsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "students")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Students who match the list request."]
    pub students: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Student>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing teachers."]
pub struct ListTeachersResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teachers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Teachers who match the list request."]
    pub teachers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Teacher>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when listing topics."]
pub struct ListTopicResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token identifying the next page of results to return. If empty, no further results are available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topic items that match the request."]
    pub topic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Material attached to course work. When creating attachments, setting the `form` field is not supported."]
pub struct Material {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Drive file material."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<SharedDriveFile>>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Forms material."]
    pub form: ::std::option::Option<::std::boxed::Box<Form>>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link material. On creation, this is upgraded to a more appropriate type if possible, and this is reflected in the response."]
    pub link: ::std::option::Option<::std::boxed::Box<Link>>,
    #[serde(rename = "youtubeVideo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "YouTube video material."]
    pub youtube_video: ::std::option::Option<::std::boxed::Box<YouTubeVideo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to modify assignee mode and options of an announcement."]
pub struct ModifyAnnouncementAssigneesRequest {
    #[serde(rename = "assigneeMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mode of the announcement describing whether it is accessible by all students or specified individual students."]
    pub assignee_mode: ::std::option::Option<ModifyAnnouncementAssigneesRequestAssigneeModeEnum>,
    #[serde(rename = "modifyIndividualStudentsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set which students can view or cannot view the announcement. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
    pub modify_individual_students_options:
        ::std::option::Option<::std::boxed::Box<ModifyIndividualStudentsOptions>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mode of the announcement describing whether it is accessible by all students or specified individual students."]
pub enum ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    #[doc = "No mode specified. This is never returned."]
    AssigneeModeUnspecified,
    #[serde(rename = "ALL_STUDENTS")]
    #[doc = "All students can see the item. This is the default state."]
    AllStudents,
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    #[doc = "A subset of the students can see the item."]
    IndividualStudents,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to modify the attachments of a student submission."]
pub struct ModifyAttachmentsRequest {
    #[serde(rename = "addAttachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attachments to add. A student submission may not have more than 20 attachments. Form attachments are not supported."]
    pub add_attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to modify assignee mode and options of a coursework."]
pub struct ModifyCourseWorkAssigneesRequest {
    #[serde(rename = "assigneeMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mode of the coursework describing whether it will be assigned to all students or specified individual students."]
    pub assignee_mode: ::std::option::Option<ModifyCourseWorkAssigneesRequestAssigneeModeEnum>,
    #[serde(rename = "modifyIndividualStudentsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set which students are assigned or not assigned to the coursework. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`."]
    pub modify_individual_students_options:
        ::std::option::Option<::std::boxed::Box<ModifyIndividualStudentsOptions>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mode of the coursework describing whether it will be assigned to all students or specified individual students."]
pub enum ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    #[doc = "No mode specified. This is never returned."]
    AssigneeModeUnspecified,
    #[serde(rename = "ALL_STUDENTS")]
    #[doc = "All students can see the item. This is the default state."]
    AllStudents,
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    #[doc = "A subset of the students can see the item."]
    IndividualStudents,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains fields to add or remove students from a course work or announcement where the `assigneeMode` is set to `INDIVIDUAL_STUDENTS`."]
pub struct ModifyIndividualStudentsOptions {
    #[serde(rename = "addStudentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of students to be added as having access to this coursework/announcement."]
    pub add_student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "removeStudentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of students to be removed from having access to this coursework/announcement."]
    pub remove_student_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional details for multiple-choice questions."]
pub struct MultipleChoiceQuestion {
    #[serde(rename = "choices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Possible choices."]
    pub choices: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Student work for a multiple-choice question."]
pub struct MultipleChoiceSubmission {
    #[serde(rename = "answer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Student's select choice."]
    pub answer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the user's name."]
pub struct Name {
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's last name. Read-only."]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's full name formed by concatenating the first and last name values. Read-only."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's first name. Read-only."]
    pub given_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to reclaim a student submission."]
pub struct ReclaimStudentSubmissionRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An instruction to Classroom to send notifications from the `feed` to the provided destination."]
pub struct Registration {
    #[serde(rename = "cloudPubsubTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Pub/Sub topic that notifications are to be sent to."]
    pub cloud_pubsub_topic: ::std::option::Option<::std::boxed::Box<CloudPubsubTopic>>,
    #[serde(rename = "expiryTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time until which the `Registration` is effective. This is a read-only field assigned by the server."]
    pub expiry_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "feed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specification for the class of notifications that Classroom should deliver to the destination."]
    pub feed: ::std::option::Option<::std::boxed::Box<Feed>>,
    #[serde(rename = "registrationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-generated unique identifier for this `Registration`. Read-only."]
    pub registration_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to return a student submission."]
pub struct ReturnStudentSubmissionRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Drive file that is used as material for course work."]
pub struct SharedDriveFile {
    #[serde(rename = "driveFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Drive file details."]
    pub drive_file: ::std::option::Option<::std::boxed::Box<DriveFile>>,
    #[serde(rename = "shareMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mechanism by which students access the Drive item."]
    pub share_mode: ::std::option::Option<SharedDriveFileShareModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mechanism by which students access the Drive item."]
pub enum SharedDriveFileShareModeEnum {
    #[serde(rename = "UNKNOWN_SHARE_MODE")]
    #[doc = "No sharing mode specified. This should never be returned."]
    UnknownShareMode,
    #[serde(rename = "VIEW")]
    #[doc = "Students can view the shared file."]
    View,
    #[serde(rename = "EDIT")]
    #[doc = "Students can edit the shared file."]
    Edit,
    #[serde(rename = "STUDENT_COPY")]
    #[doc = "Students have a personal copy of the shared file."]
    StudentCopy,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Student work for a short answer question."]
pub struct ShortAnswerSubmission {
    #[serde(rename = "answer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Student response to a short-answer question."]
    pub answer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The history of each state this submission has been in."]
pub struct StateHistory {
    #[serde(rename = "actorUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The teacher or student who made the change."]
    pub actor_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The workflow pipeline stage."]
    pub state: ::std::option::Option<StateHistoryStateEnum>,
    #[serde(rename = "stateTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the submission entered this state."]
    pub state_timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The workflow pipeline stage."]
pub enum StateHistoryStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "No state specified. This should never be returned."]
    StateUnspecified,
    #[serde(rename = "CREATED")]
    #[doc = "The Submission has been created."]
    Created,
    #[serde(rename = "TURNED_IN")]
    #[doc = "The student has turned in an assigned document, which may or may not be a template."]
    TurnedIn,
    #[serde(rename = "RETURNED")]
    #[doc = "The teacher has returned the assigned document to the student."]
    Returned,
    #[serde(rename = "RECLAIMED_BY_STUDENT")]
    #[doc = "The student turned in the assigned document, and then chose to \"unsubmit\" the assignment, giving the student control again as the owner."]
    ReclaimedByStudent,
    #[serde(rename = "STUDENT_EDITED_AFTER_TURN_IN")]
    #[doc = "The student edited their submission after turning it in. Currently, only used by Questions, when the student edits their answer."]
    StudentEditedAfterTurnIn,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Student in a course."]
pub struct Student {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global user information for the student. Read-only."]
    pub profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
    #[serde(rename = "studentWorkFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only."]
    pub student_work_folder: ::std::option::Option<::std::boxed::Box<DriveFolder>>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Student submission for course work. StudentSubmission items are generated when a CourseWork item is created. StudentSubmissions that have never been accessed (i.e. with `state` = NEW) may not have a creation time or update time."]
pub struct StudentSubmission {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Absolute link to the submission in the Classroom web UI. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assignedGrade")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This may be modified only by course teachers."]
    pub assigned_grade: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "assignmentSubmission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Submission content when course_work_type is ASSIGNMENT. Students can modify this content using ModifyAttachments."]
    pub assignment_submission: ::std::option::Option<::std::boxed::Box<AssignmentSubmission>>,
    #[serde(rename = "associatedWithDeveloper")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this student submission is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only."]
    pub associated_with_developer: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "courseWorkId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the course work this corresponds to. Read-only."]
    pub course_work_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "courseWorkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of course work this submission is for. Read-only."]
    pub course_work_type: ::std::option::Option<StudentSubmissionCourseWorkTypeEnum>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of this submission. This may be unset if the student has not accessed this item. Read-only."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "draftGrade")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional pending grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This is only visible to and modifiable by course teachers."]
    pub draft_grade: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Classroom-assigned Identifier for the student submission. This is unique among submissions for the relevant course work. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "late")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this submission is late. Read-only."]
    pub late: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "multipleChoiceSubmission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Submission content when course_work_type is MULTIPLE_CHOICE_QUESTION."]
    pub multiple_choice_submission:
        ::std::option::Option<::std::boxed::Box<MultipleChoiceSubmission>>,
    #[serde(rename = "shortAnswerSubmission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Submission content when course_work_type is SHORT_ANSWER_QUESTION."]
    pub short_answer_submission: ::std::option::Option<::std::boxed::Box<ShortAnswerSubmission>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of this submission. Read-only."]
    pub state: ::std::option::Option<StudentSubmissionStateEnum>,
    #[serde(rename = "submissionHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The history of the submission (includes state and grade histories). Read-only."]
    pub submission_history:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SubmissionHistory>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last update time of this submission. This may be unset if the student has not accessed this item. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for the student that owns this submission. Read-only."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of course work this submission is for. Read-only."]
pub enum StudentSubmissionCourseWorkTypeEnum {
    #[serde(rename = "COURSE_WORK_TYPE_UNSPECIFIED")]
    #[doc = "No work type specified. This is never returned."]
    CourseWorkTypeUnspecified,
    #[serde(rename = "ASSIGNMENT")]
    #[doc = "An assignment."]
    Assignment,
    #[serde(rename = "SHORT_ANSWER_QUESTION")]
    #[doc = "A short answer question."]
    ShortAnswerQuestion,
    #[serde(rename = "MULTIPLE_CHOICE_QUESTION")]
    #[doc = "A multiple-choice question."]
    MultipleChoiceQuestion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of this submission. Read-only."]
pub enum StudentSubmissionStateEnum {
    #[serde(rename = "SUBMISSION_STATE_UNSPECIFIED")]
    #[doc = "No state specified. This should never be returned."]
    SubmissionStateUnspecified,
    #[serde(rename = "NEW")]
    #[doc = "The student has never accessed this submission. Attachments are not returned and timestamps is not set."]
    New,
    #[serde(rename = "CREATED")]
    #[doc = "Has been created."]
    Created,
    #[serde(rename = "TURNED_IN")]
    #[doc = "Has been turned in to the teacher."]
    TurnedIn,
    #[serde(rename = "RETURNED")]
    #[doc = "Has been returned to the student."]
    Returned,
    #[serde(rename = "RECLAIMED_BY_STUDENT")]
    #[doc = "Student chose to \"unsubmit\" the assignment."]
    ReclaimedByStudent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The history of the submission. This currently includes state and grade histories."]
pub struct SubmissionHistory {
    #[serde(rename = "gradeHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The grade history information of the submission, if present."]
    pub grade_history: ::std::option::Option<::std::boxed::Box<GradeHistory>>,
    #[serde(rename = "stateHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state history information of the submission, if present."]
    pub state_history: ::std::option::Option<::std::boxed::Box<StateHistory>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Teacher of a course."]
pub struct Teacher {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global user information for the teacher. Read-only."]
    pub profile: ::std::option::Option<::std::boxed::Box<UserProfile>>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `\"me\"`, indicating the requesting user"]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
pub struct TimeOfDay {
    #[serde(rename = "hours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub seconds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Topic created by a teacher for the course"]
pub struct Topic {
    #[serde(rename = "courseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the course. Read-only."]
    pub course_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. The result must be a non-empty string. Topic names are case sensitive, and must be no longer than 100 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the topic. Read-only."]
    pub topic_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the topic was last updated by the system. Read-only."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to turn in a student submission."]
pub struct TurnInStudentSubmissionRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Global information for a user."]
pub struct UserProfile {
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the user. Read-only."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the user. Read-only."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the user. Read-only."]
    pub name: ::std::option::Option<::std::boxed::Box<Name>>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global permissions of the user. Read-only."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GlobalPermission>>>,
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of user's profile photo. Read-only."]
    pub photo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verifiedTeacher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents whether a G Suite for Education user's domain administrator has explicitly verified them as being a teacher. If the user is not a member of a G Suite for Education domain, than this field is always false. Read-only"]
    pub verified_teacher: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "YouTube video item."]
pub struct YouTubeVideo {
    #[serde(rename = "alternateLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL that can be used to view the YouTube video. Read-only."]
    pub alternate_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "YouTube API resource ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of a thumbnail image of the YouTube video. Read-only."]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the YouTube video. Read-only."]
    pub title: ::std::option::Option<::std::string::String>,
}
