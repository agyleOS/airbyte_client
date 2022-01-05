# WebBackendConnectionRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | **String** |  | 
**name** | **String** |  | 
**namespace_definition** | Option<[**crate::models::NamespaceDefinitionType**](NamespaceDefinitionType.md)> |  | [optional]
**namespace_format** | Option<**String**> | Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'. | [optional][default to null]
**prefix** | Option<**String**> | Prefix that will be prepended to the name of each stream when it is written to the destination. | [optional]
**source_id** | **String** |  | 
**destination_id** | **String** |  | 
**sync_catalog** | [**crate::models::AirbyteCatalog**](AirbyteCatalog.md) |  | 
**schedule** | Option<[**crate::models::ConnectionSchedule**](ConnectionSchedule.md)> |  | [optional]
**status** | [**crate::models::ConnectionStatus**](ConnectionStatus.md) |  | 
**operation_ids** | Option<**Vec<String>**> |  | [optional]
**source** | [**crate::models::SourceRead**](SourceRead.md) |  | 
**destination** | [**crate::models::DestinationRead**](DestinationRead.md) |  | 
**operations** | Option<[**Vec<crate::models::OperationRead>**](OperationRead.md)> |  | [optional]
**latest_sync_job_created_at** | Option<**i64**> | epoch time of the latest sync job. null if no sync job has taken place. | [optional]
**latest_sync_job_status** | Option<[**crate::models::JobStatus**](JobStatus.md)> |  | [optional]
**is_syncing** | **bool** |  | 
**resource_requirements** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


