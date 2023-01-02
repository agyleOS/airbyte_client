/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// WebBackendConnectionListItem : Information about a connection that shows up in the connection list view.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebBackendConnectionListItem {
    #[serde(rename = "connectionId")]
    pub connection_id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sourceId")]
    pub source_id: uuid::Uuid,
    #[serde(rename = "destinationId")]
    pub destination_id: uuid::Uuid,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<crate::models::ConnectionScheduleType>,
    #[serde(rename = "scheduleData", skip_serializing_if = "Option::is_none")]
    pub schedule_data: Option<Box<crate::models::ConnectionScheduleData>>,
    #[serde(rename = "status")]
    pub status: crate::models::ConnectionStatus,
    #[serde(rename = "source")]
    pub source: Box<crate::models::SourceRead>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::DestinationRead>,
    /// epoch time of the latest sync job. null if no sync job has taken place.
    #[serde(
        rename = "latestSyncJobCreatedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_sync_job_created_at: Option<i64>,
    #[serde(
        rename = "latestSyncJobStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_sync_job_status: Option<crate::models::JobStatus>,
    #[serde(rename = "isSyncing")]
    pub is_syncing: bool,
    #[serde(rename = "schemaChange")]
    pub schema_change: crate::models::SchemaChange,
}

impl WebBackendConnectionListItem {
    /// Information about a connection that shows up in the connection list view.
    pub fn new(
        connection_id: uuid::Uuid,
        name: String,
        source_id: uuid::Uuid,
        destination_id: uuid::Uuid,
        status: crate::models::ConnectionStatus,
        source: crate::models::SourceRead,
        destination: crate::models::DestinationRead,
        is_syncing: bool,
        schema_change: crate::models::SchemaChange,
    ) -> WebBackendConnectionListItem {
        WebBackendConnectionListItem {
            connection_id,
            name,
            source_id,
            destination_id,
            schedule_type: None,
            schedule_data: None,
            status,
            source: Box::new(source),
            destination: Box::new(destination),
            latest_sync_job_created_at: None,
            latest_sync_job_status: None,
            is_syncing,
            schema_change,
        }
    }
}