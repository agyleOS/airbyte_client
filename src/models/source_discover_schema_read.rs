/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// SourceDiscoverSchemaRead : Returns the results of a discover catalog job. If the job was not successful, the catalog field will not be present. jobInfo will aways be present and its status be used to determine if the job was successful or not.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceDiscoverSchemaRead {
    #[serde(rename = "catalog", skip_serializing_if = "Option::is_none")]
    pub catalog: Option<Box<crate::models::AirbyteCatalog>>,
    #[serde(rename = "jobInfo")]
    pub job_info: Box<crate::models::SynchronousJobRead>,
    #[serde(rename = "catalogId", skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<uuid::Uuid>,
    #[serde(rename = "catalogDiff", skip_serializing_if = "Option::is_none")]
    pub catalog_diff: Option<Box<crate::models::CatalogDiff>>,
    #[serde(rename = "breakingChange", skip_serializing_if = "Option::is_none")]
    pub breaking_change: Option<bool>,
    #[serde(rename = "connectionStatus", skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<crate::models::ConnectionStatus>,
}

impl SourceDiscoverSchemaRead {
    /// Returns the results of a discover catalog job. If the job was not successful, the catalog field will not be present. jobInfo will aways be present and its status be used to determine if the job was successful or not.
    pub fn new(job_info: crate::models::SynchronousJobRead) -> SourceDiscoverSchemaRead {
        SourceDiscoverSchemaRead {
            catalog: None,
            job_info: Box::new(job_info),
            catalog_id: None,
            catalog_diff: None,
            breaking_change: None,
            connection_status: None,
        }
    }
}
