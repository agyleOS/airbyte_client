/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  The Configuration API is an internal Airbyte API that is designed for communications between different Airbyte components. * Its main purpose is to enable the Airbyte Engineering team to configure the internal state of [Airbyte Cloud](https://airbyte.com/airbyte-cloud) * It is also sometimes used by OSS users to configure their own Self-Hosted Airbyte deployment (internal state, etc)  WARNING * Airbyte does NOT have active commitments to support this API long-term. * OSS users can utilize the Configuration API, but at their own risk. * This API is utilized internally by the Airbyte Engineering team and may be modified in the future if the need arises. * Modifications by the Airbyte Engineering team could create breaking changes and OSS users would need to update their code to catch up to any backwards incompatible changes in the API.  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/api/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/api/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OperatorWebhook {
    /// The id of the webhook configs to use from the workspace.
    #[serde(rename = "webhookConfigId", skip_serializing_if = "Option::is_none")]
    pub webhook_config_id: Option<uuid::Uuid>,
    #[serde(rename = "webhookType", skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<WebhookType>,
    #[serde(rename = "dbtCloud", skip_serializing_if = "Option::is_none")]
    pub dbt_cloud: Option<Box<crate::models::OperatorWebhookDbtCloud>>,
    /// DEPRECATED. Populate dbtCloud instead.
    #[serde(rename = "executionUrl", skip_serializing_if = "Option::is_none")]
    pub execution_url: Option<String>,
    /// DEPRECATED. Populate dbtCloud instead.
    #[serde(rename = "executionBody", skip_serializing_if = "Option::is_none")]
    pub execution_body: Option<String>,
}

impl OperatorWebhook {
    pub fn new() -> OperatorWebhook {
        OperatorWebhook {
            webhook_config_id: None,
            webhook_type: None,
            dbt_cloud: None,
            execution_url: None,
            execution_body: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookType {
    #[serde(rename = "dbtCloud")]
    DbtCloud,
}

impl Default for WebhookType {
    fn default() -> WebhookType {
        Self::DbtCloud
    }
}
