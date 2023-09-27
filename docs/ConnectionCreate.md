# ConnectionCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Optional name of the connection | [optional]
**namespace_definition** | Option<[**crate::models::NamespaceDefinitionType**](NamespaceDefinitionType.md)> |  | [optional]
**namespace_format** | Option<**String**> | Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'. | [optional]
**prefix** | Option<**String**> | Prefix that will be prepended to the name of each stream when it is written to the destination. | [optional]
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**destination_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**operation_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**sync_catalog** | Option<[**crate::models::AirbyteCatalog**](AirbyteCatalog.md)> |  | [optional]
**schedule** | Option<[**crate::models::ConnectionSchedule**](ConnectionSchedule.md)> |  | [optional]
**schedule_type** | Option<[**crate::models::ConnectionScheduleType**](ConnectionScheduleType.md)> |  | [optional]
**schedule_data** | Option<[**crate::models::ConnectionScheduleData**](ConnectionScheduleData.md)> |  | [optional]
**status** | [**crate::models::ConnectionStatus**](ConnectionStatus.md) |  | 
**resource_requirements** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]
**source_catalog_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**geography** | Option<[**crate::models::Geography**](Geography.md)> |  | [optional]
**notify_schema_changes** | Option<**bool**> |  | [optional]
**notify_schema_changes_by_email** | Option<**bool**> |  | [optional]
**non_breaking_changes_preference** | Option<[**crate::models::NonBreakingChangesPreference**](NonBreakingChangesPreference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


