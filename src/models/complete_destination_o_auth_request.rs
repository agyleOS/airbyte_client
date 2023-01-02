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
pub struct CompleteDestinationOAuthRequest {
    #[serde(rename = "destinationDefinitionId")]
    pub destination_definition_id: uuid::Uuid,
    #[serde(rename = "workspaceId")]
    pub workspace_id: uuid::Uuid,
    /// When completing OAuth flow to gain an access token, some API sometimes requires to verify that the app re-send the redirectUrl that was used when consent was given.
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// The query parameters present in the redirect URL after a user granted consent e.g auth code
    #[serde(rename = "queryParams", skip_serializing_if = "Option::is_none")]
    pub query_params: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The values required to configure OAuth flows. The schema for this must match the `OAuthConfigSpecification.oauthUserInputFromConnectorConfigSpecification` schema.
    #[serde(
        rename = "oAuthInputConfiguration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o_auth_input_configuration: Option<Option<serde_json::Value>>,
    #[serde(rename = "destinationId", skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<uuid::Uuid>,
}

impl CompleteDestinationOAuthRequest {
    pub fn new(
        destination_definition_id: uuid::Uuid,
        workspace_id: uuid::Uuid,
    ) -> CompleteDestinationOAuthRequest {
        CompleteDestinationOAuthRequest {
            destination_definition_id,
            workspace_id,
            redirect_url: None,
            query_params: None,
            o_auth_input_configuration: None,
            destination_id: None,
        }
    }
}
