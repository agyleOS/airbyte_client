# WebBackendConnectionRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**namespace_definition** | Option<[**crate::models::NamespaceDefinitionType**](NamespaceDefinitionType.md)> |  | [optional]
**namespace_format** | Option<**String**> | Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'. | [optional]
**prefix** | Option<**String**> | Prefix that will be prepended to the name of each stream when it is written to the destination. | [optional]
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**destination_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**sync_catalog** | [**crate::models::AirbyteCatalog**](AirbyteCatalog.md) |  | 
**schedule** | Option<[**crate::models::ConnectionSchedule**](ConnectionSchedule.md)> |  | [optional]
**schedule_type** | Option<[**crate::models::ConnectionScheduleType**](ConnectionScheduleType.md)> |  | [optional]
**schedule_data** | Option<[**crate::models::ConnectionScheduleData**](ConnectionScheduleData.md)> |  | [optional]
**status** | [**crate::models::ConnectionStatus**](ConnectionStatus.md) |  | 
**operation_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**source** | [**crate::models::SourceRead**](SourceRead.md) |  | 
**destination** | [**crate::models::DestinationRead**](DestinationRead.md) |  | 
**operations** | Option<[**Vec<crate::models::OperationRead>**](OperationRead.md)> |  | [optional]
**latest_sync_job_created_at** | Option<**i64**> | epoch time of the latest sync job. null if no sync job has taken place. | [optional]
**latest_sync_job_status** | Option<[**crate::models::JobStatus**](JobStatus.md)> |  | [optional]
**is_syncing** | **bool** |  | 
**resource_requirements** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]
**catalog_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**catalog_diff** | Option<[**crate::models::CatalogDiff**](CatalogDiff.md)> |  | [optional]
**geography** | Option<[**crate::models::Geography**](Geography.md)> |  | [optional]
**schema_change** | [**crate::models::SchemaChange**](SchemaChange.md) |  | 
**notify_schema_changes** | **bool** |  | 
**notify_schema_changes_by_email** | **bool** |  | 
**non_breaking_changes_preference** | [**crate::models::NonBreakingChangesPreference**](NonBreakingChangesPreference.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


