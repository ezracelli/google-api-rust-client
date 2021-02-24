use super::rest_method::RestMethod;

#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct RestResource {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "methods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Methods on this resource."]
    pub methods:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub-resources on this resource."]
    pub resources: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<RestResource>>,
    >,
}
impl RestResource {
    pub fn builder() -> RestResourceBuilder {
        RestResourceBuilder::default()
    }
}
