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
pub struct DestinationCreate {
    #[serde(rename = "workspaceId")]
    pub workspace_id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "destinationDefinitionId")]
    pub destination_definition_id: uuid::Uuid,
    /// The values required to configure the destination. The schema for this must match the schema return by destination_definition_specifications/get for the destinationDefinition.
    #[serde(
        rename = "connectionConfiguration",
        deserialize_with = "Option::deserialize"
    )]
    pub connection_configuration: Option<serde_json::Value>,
}

impl DestinationCreate {
    pub fn new(
        workspace_id: uuid::Uuid,
        name: String,
        destination_definition_id: uuid::Uuid,
        connection_configuration: Option<serde_json::Value>,
    ) -> DestinationCreate {
        DestinationCreate {
            workspace_id,
            name,
            destination_definition_id,
            connection_configuration,
        }
    }
}
