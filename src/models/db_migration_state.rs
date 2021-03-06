/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints. * All `web_backend` APIs are not considered public APIs and are not guaranteeing backwards compatibility.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DbMigrationState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "above_target")]
    AboveTarget,
    #[serde(rename = "below_baseline")]
    BelowBaseline,
    #[serde(rename = "baseline")]
    Baseline,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "missing_success")]
    MissingSuccess,
    #[serde(rename = "missing_failed")]
    MissingFailed,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "undone")]
    Undone,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "out_of_order")]
    OutOfOrder,
    #[serde(rename = "future_success")]
    FutureSuccess,
    #[serde(rename = "future_failed")]
    FutureFailed,
    #[serde(rename = "outdated")]
    Outdated,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "deleted")]
    Deleted,
}

impl ToString for DbMigrationState {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::AboveTarget => String::from("above_target"),
            Self::BelowBaseline => String::from("below_baseline"),
            Self::Baseline => String::from("baseline"),
            Self::Ignored => String::from("ignored"),
            Self::MissingSuccess => String::from("missing_success"),
            Self::MissingFailed => String::from("missing_failed"),
            Self::Success => String::from("success"),
            Self::Undone => String::from("undone"),
            Self::Available => String::from("available"),
            Self::Failed => String::from("failed"),
            Self::OutOfOrder => String::from("out_of_order"),
            Self::FutureSuccess => String::from("future_success"),
            Self::FutureFailed => String::from("future_failed"),
            Self::Outdated => String::from("outdated"),
            Self::Superseded => String::from("superseded"),
            Self::Deleted => String::from("deleted"),
        }
    }
}

impl Default for DbMigrationState {
    fn default() -> DbMigrationState {
        Self::Pending
    }
}
