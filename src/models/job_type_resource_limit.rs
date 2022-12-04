/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints. * All `web_backend` APIs are not considered public APIs and are not guaranteeing backwards compatibility.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// JobTypeResourceLimit : sets resource requirements for a specific job type for an actor definition. these values override the default, if both are set.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobTypeResourceLimit {
    #[serde(rename = "jobType")]
    pub job_type: crate::models::JobType,
    #[serde(rename = "resourceRequirements")]
    pub resource_requirements: Box<crate::models::ResourceRequirements>,
}

impl JobTypeResourceLimit {
    /// sets resource requirements for a specific job type for an actor definition. these values override the default, if both are set.
    pub fn new(
        job_type: crate::models::JobType,
        resource_requirements: crate::models::ResourceRequirements,
    ) -> JobTypeResourceLimit {
        JobTypeResourceLimit {
            job_type,
            resource_requirements: Box::new(resource_requirements),
        }
    }
}
