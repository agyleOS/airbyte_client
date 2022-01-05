/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotFoundKnownExceptionInfo {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "exceptionClassName", skip_serializing_if = "Option::is_none")]
    pub exception_class_name: Option<String>,
    #[serde(rename = "exceptionStack", skip_serializing_if = "Option::is_none")]
    pub exception_stack: Option<Vec<String>>,
    #[serde(rename = "rootCauseExceptionClassName", skip_serializing_if = "Option::is_none")]
    pub root_cause_exception_class_name: Option<String>,
    #[serde(rename = "rootCauseExceptionStack", skip_serializing_if = "Option::is_none")]
    pub root_cause_exception_stack: Option<Vec<String>>,
}

impl NotFoundKnownExceptionInfo {
    pub fn new(message: String) -> NotFoundKnownExceptionInfo {
        NotFoundKnownExceptionInfo {
            id: None,
            message,
            exception_class_name: None,
            exception_stack: None,
            root_cause_exception_class_name: None,
            root_cause_exception_stack: None,
        }
    }
}

