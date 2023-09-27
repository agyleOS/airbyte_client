# WebBackendConnectionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name that will be set to the connection | [optional]
**connection_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**namespace_definition** | Option<[**crate::models::NamespaceDefinitionType**](NamespaceDefinitionType.md)> |  | [optional]
**namespace_format** | Option<**String**> | Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'. | [optional]
**prefix** | Option<**String**> | Prefix that will be prepended to the name of each stream when it is written to the destination. | [optional]
**sync_catalog** | Option<[**crate::models::AirbyteCatalog**](AirbyteCatalog.md)> |  | [optional]
**schedule** | Option<[**crate::models::ConnectionSchedule**](ConnectionSchedule.md)> |  | [optional]
**schedule_type** | Option<[**crate::models::ConnectionScheduleType**](ConnectionScheduleType.md)> |  | [optional]
**schedule_data** | Option<[**crate::models::ConnectionScheduleData**](ConnectionScheduleData.md)> |  | [optional]
**status** | Option<[**crate::models::ConnectionStatus**](ConnectionStatus.md)> |  | [optional]
**resource_requirements** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]
**skip_reset** | Option<**bool**> |  | [optional]
**operations** | Option<[**Vec<crate::models::WebBackendOperationCreateOrUpdate>**](WebBackendOperationCreateOrUpdate.md)> |  | [optional]
**source_catalog_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**geography** | Option<[**crate::models::Geography**](Geography.md)> |  | [optional]
**notify_schema_changes** | Option<**bool**> |  | [optional]
**notify_schema_changes_by_email** | Option<**bool**> |  | [optional]
**non_breaking_changes_preference** | Option<[**crate::models::NonBreakingChangesPreference**](NonBreakingChangesPreference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


