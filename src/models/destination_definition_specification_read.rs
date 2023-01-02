/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinationDefinitionSpecificationRead {
    #[serde(rename = "destinationDefinitionId")]
    pub destination_definition_id: uuid::Uuid,
    #[serde(rename = "documentationUrl", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// The specification for what values are required to configure the destinationDefinition.
    #[serde(
        rename = "connectionSpecification",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_specification: Option<Option<serde_json::Value>>,
    #[serde(rename = "authSpecification", skip_serializing_if = "Option::is_none")]
    pub auth_specification: Option<Box<crate::models::AuthSpecification>>,
    #[serde(rename = "advancedAuth", skip_serializing_if = "Option::is_none")]
    pub advanced_auth: Option<Box<crate::models::AdvancedAuth>>,
    #[serde(rename = "jobInfo")]
    pub job_info: Box<crate::models::SynchronousJobRead>,
    #[serde(
        rename = "supportedDestinationSyncModes",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_destination_sync_modes: Option<Vec<crate::models::DestinationSyncMode>>,
    #[serde(rename = "supportsDbt", skip_serializing_if = "Option::is_none")]
    pub supports_dbt: Option<bool>,
    #[serde(
        rename = "supportsNormalization",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_normalization: Option<bool>,
}

impl DestinationDefinitionSpecificationRead {
    pub fn new(
        destination_definition_id: uuid::Uuid,
        job_info: crate::models::SynchronousJobRead,
    ) -> DestinationDefinitionSpecificationRead {
        DestinationDefinitionSpecificationRead {
            destination_definition_id,
            documentation_url: None,
            connection_specification: None,
            auth_specification: None,
            advanced_auth: None,
            job_info: Box::new(job_info),
            supported_destination_sync_modes: None,
            supports_dbt: None,
            supports_normalization: None,
        }
    }
}
