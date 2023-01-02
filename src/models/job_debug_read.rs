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
pub struct JobDebugRead {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "configType")]
    pub config_type: crate::models::JobConfigType,
    #[serde(rename = "configId")]
    pub config_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::JobStatus,
    #[serde(rename = "airbyteVersion")]
    pub airbyte_version: String,
    #[serde(rename = "sourceDefinition")]
    pub source_definition: Box<crate::models::SourceDefinitionRead>,
    #[serde(rename = "destinationDefinition")]
    pub destination_definition: Box<crate::models::DestinationDefinitionRead>,
}

impl JobDebugRead {
    pub fn new(
        id: i64,
        config_type: crate::models::JobConfigType,
        config_id: String,
        status: crate::models::JobStatus,
        airbyte_version: String,
        source_definition: crate::models::SourceDefinitionRead,
        destination_definition: crate::models::DestinationDefinitionRead,
    ) -> JobDebugRead {
        JobDebugRead {
            id,
            config_type,
            config_id,
            status,
            airbyte_version,
            source_definition: Box::new(source_definition),
            destination_definition: Box::new(destination_definition),
        }
    }
}
