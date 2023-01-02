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
pub struct WebBackendConnectionRead {
    #[serde(rename = "connectionId")]
    pub connection_id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "namespaceDefinition",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_definition: Option<crate::models::NamespaceDefinitionType>,
    /// Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'.
    #[serde(rename = "namespaceFormat", skip_serializing_if = "Option::is_none")]
    pub namespace_format: Option<String>,
    /// Prefix that will be prepended to the name of each stream when it is written to the destination.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: uuid::Uuid,
    #[serde(rename = "destinationId")]
    pub destination_id: uuid::Uuid,
    #[serde(rename = "syncCatalog")]
    pub sync_catalog: Box<crate::models::AirbyteCatalog>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::ConnectionSchedule>>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<crate::models::ConnectionScheduleType>,
    #[serde(rename = "scheduleData", skip_serializing_if = "Option::is_none")]
    pub schedule_data: Option<Box<crate::models::ConnectionScheduleData>>,
    #[serde(rename = "status")]
    pub status: crate::models::ConnectionStatus,
    #[serde(rename = "operationIds", skip_serializing_if = "Option::is_none")]
    pub operation_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "source")]
    pub source: Box<crate::models::SourceRead>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::DestinationRead>,
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<crate::models::OperationRead>>,
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
    #[serde(
        rename = "resourceRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_requirements: Option<Box<crate::models::ResourceRequirements>>,
    #[serde(rename = "catalogId", skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<uuid::Uuid>,
    #[serde(rename = "catalogDiff", skip_serializing_if = "Option::is_none")]
    pub catalog_diff: Option<Box<crate::models::CatalogDiff>>,
    #[serde(rename = "geography", skip_serializing_if = "Option::is_none")]
    pub geography: Option<crate::models::Geography>,
    #[serde(rename = "schemaChange")]
    pub schema_change: crate::models::SchemaChange,
    #[serde(rename = "notifySchemaChanges")]
    pub notify_schema_changes: bool,
    #[serde(rename = "nonBreakingChangesPreference")]
    pub non_breaking_changes_preference: crate::models::NonBreakingChangesPreference,
}

impl WebBackendConnectionRead {
    pub fn new(
        connection_id: uuid::Uuid,
        name: String,
        source_id: uuid::Uuid,
        destination_id: uuid::Uuid,
        sync_catalog: crate::models::AirbyteCatalog,
        status: crate::models::ConnectionStatus,
        source: crate::models::SourceRead,
        destination: crate::models::DestinationRead,
        is_syncing: bool,
        schema_change: crate::models::SchemaChange,
        notify_schema_changes: bool,
        non_breaking_changes_preference: crate::models::NonBreakingChangesPreference,
    ) -> WebBackendConnectionRead {
        WebBackendConnectionRead {
            connection_id,
            name,
            namespace_definition: None,
            namespace_format: None,
            prefix: None,
            source_id,
            destination_id,
            sync_catalog: Box::new(sync_catalog),
            schedule: None,
            schedule_type: None,
            schedule_data: None,
            status,
            operation_ids: None,
            source: Box::new(source),
            destination: Box::new(destination),
            operations: None,
            latest_sync_job_created_at: None,
            latest_sync_job_status: None,
            is_syncing,
            resource_requirements: None,
            catalog_id: None,
            catalog_diff: None,
            geography: None,
            schema_change,
            notify_schema_changes,
            non_breaking_changes_preference,
        }
    }
}
