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
pub struct AttemptFailureReason {
    #[serde(rename = "failureOrigin", skip_serializing_if = "Option::is_none")]
    pub failure_origin: Option<crate::models::AttemptFailureOrigin>,
    #[serde(rename = "failureType", skip_serializing_if = "Option::is_none")]
    pub failure_type: Option<crate::models::AttemptFailureType>,
    #[serde(rename = "externalMessage", skip_serializing_if = "Option::is_none")]
    pub external_message: Option<String>,
    #[serde(rename = "stacktrace", skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
    /// True if it is known that retrying may succeed, e.g. for a transient failure. False if it is known that a retry will not succeed, e.g. for a configuration issue. If not set, retryable status is not well known.
    #[serde(rename = "retryable", skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

impl AttemptFailureReason {
    pub fn new(timestamp: i64) -> AttemptFailureReason {
        AttemptFailureReason {
            failure_origin: None,
            failure_type: None,
            external_message: None,
            stacktrace: None,
            retryable: None,
            timestamp,
        }
    }
}
