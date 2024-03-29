/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// OAuth2Specification : An object containing any metadata needed to describe this connector's Oauth flow

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OAuth2Specification {
    /// A list of strings representing a pointer to the root object which contains any oauth parameters in the ConnectorSpecification. Examples: if oauth parameters were contained inside the top level, rootObject=[] If they were nested inside another object {'credentials': {'app_id' etc...}, rootObject=['credentials'] If they were inside a oneOf {'switch': {oneOf: [{client_id...}, {non_oauth_param]}},  rootObject=['switch', 0]
    #[serde(rename = "rootObject")]
    pub root_object: Vec<serde_json::Value>,
    /// Pointers to the fields in the rootObject needed to obtain the initial refresh/access tokens for the OAuth flow. Each inner array represents the path in the rootObject of the referenced field. For example. Assume the rootObject contains params 'app_secret', 'app_id' which are needed to get the initial refresh token. If they are not nested in the rootObject, then the array would look like this [['app_secret'], ['app_id']] If they are nested inside an object called 'auth_params' then this array would be [['auth_params', 'app_secret'], ['auth_params', 'app_id']]
    #[serde(rename = "oauthFlowInitParameters")]
    pub oauth_flow_init_parameters: Vec<Vec<String>>,
    /// Pointers to the fields in the rootObject which can be populated from successfully completing the oauth flow using the init parameters. This is typically a refresh/access token. Each inner array represents the path in the rootObject of the referenced field.
    #[serde(rename = "oauthFlowOutputParameters")]
    pub oauth_flow_output_parameters: Vec<Vec<String>>,
}

impl OAuth2Specification {
    /// An object containing any metadata needed to describe this connector's Oauth flow
    pub fn new(
        root_object: Vec<serde_json::Value>,
        oauth_flow_init_parameters: Vec<Vec<String>>,
        oauth_flow_output_parameters: Vec<Vec<String>>,
    ) -> OAuth2Specification {
        OAuth2Specification {
            root_object,
            oauth_flow_init_parameters,
            oauth_flow_output_parameters,
        }
    }
}
