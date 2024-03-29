/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  The Configuration API is an internal Airbyte API that is designed for communications between different Airbyte components. * Its main purpose is to enable the Airbyte Engineering team to configure the internal state of [Airbyte Cloud](https://airbyte.com/airbyte-cloud) * It is also sometimes used by OSS users to configure their own Self-Hosted Airbyte deployment (internal state, etc)  WARNING * Airbyte does NOT have active commitments to support this API long-term. * OSS users can utilize the Configuration API, but at their own risk. * This API is utilized internally by the Airbyte Engineering team and may be modified in the future if the need arises. * Modifications by the Airbyte Engineering team could create breaking changes and OSS users would need to update their code to catch up to any backwards incompatible changes in the API.  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/api/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/api/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// AirbyteStream : the immutable schema defined by the source

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AirbyteStream {
    /// Stream's name.
    #[serde(rename = "name")]
    pub name: String,
    /// Stream schema using Json Schema specs.
    #[serde(rename = "jsonSchema", skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    #[serde(rename = "supportedSyncModes", skip_serializing_if = "Option::is_none")]
    pub supported_sync_modes: Option<Vec<crate::models::SyncMode>>,
    /// If the source defines the cursor field, then any other cursor field inputs will be ignored. If it does not, either the user_provided one is used, or the default one is used as a backup.
    #[serde(
        rename = "sourceDefinedCursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_defined_cursor: Option<bool>,
    /// Path to the field that will be used to determine if a record is new or modified since the last sync. If not provided by the source, the end user will have to specify the comparable themselves.
    #[serde(rename = "defaultCursorField", skip_serializing_if = "Option::is_none")]
    pub default_cursor_field: Option<Vec<String>>,
    /// If the source defines the primary key, paths to the fields that will be used as a primary key. If not provided by the source, the end user will have to specify the primary key themselves.
    #[serde(
        rename = "sourceDefinedPrimaryKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_defined_primary_key: Option<Vec<Vec<String>>>,
    /// Optional Source-defined namespace. Airbyte streams from the same sources should have the same namespace. Currently only used by JDBC destinations to determine what schema to write to.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl AirbyteStream {
    /// the immutable schema defined by the source
    pub fn new(name: String) -> AirbyteStream {
        AirbyteStream {
            name,
            json_schema: None,
            supported_sync_modes: None,
            source_defined_cursor: None,
            default_cursor_field: None,
            source_defined_primary_key: None,
            namespace: None,
        }
    }
}
