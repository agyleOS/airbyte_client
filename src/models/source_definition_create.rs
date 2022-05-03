/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints. * All `web_backend` APIs are not considered public APIs and are not guaranteeing backwards compatibility.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceDefinitionCreate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dockerRepository")]
    pub docker_repository: String,
    #[serde(rename = "dockerImageTag")]
    pub docker_image_tag: String,
    #[serde(rename = "documentationUrl")]
    pub documentation_url: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(
        rename = "resourceRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_requirements: Option<Box<crate::models::ActorDefinitionResourceRequirements>>,
}

impl SourceDefinitionCreate {
    pub fn new(
        name: String,
        docker_repository: String,
        docker_image_tag: String,
        documentation_url: String,
    ) -> SourceDefinitionCreate {
        SourceDefinitionCreate {
            name,
            docker_repository,
            docker_image_tag,
            documentation_url,
            icon: None,
            resource_requirements: None,
        }
    }
}
