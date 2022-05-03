/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints. * All `web_backend` APIs are not considered public APIs and are not guaranteeing backwards compatibility.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// AttemptFailureType : Categorizes well known errors into types for programmatic handling. If not set, the type of error is not well known.

/// Categorizes well known errors into types for programmatic handling. If not set, the type of error is not well known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttemptFailureType {
    #[serde(rename = "config_error")]
    ConfigError,
    #[serde(rename = "system_error")]
    SystemError,
    #[serde(rename = "manual_cancellation")]
    ManualCancellation,
}

impl ToString for AttemptFailureType {
    fn to_string(&self) -> String {
        match self {
            Self::ConfigError => String::from("config_error"),
            Self::SystemError => String::from("system_error"),
            Self::ManualCancellation => String::from("manual_cancellation"),
        }
    }
}

impl Default for AttemptFailureType {
    fn default() -> AttemptFailureType {
        Self::ConfigError
    }
}
