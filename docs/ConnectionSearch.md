# ConnectionSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**namespace_definition** | Option<[**crate::models::NamespaceDefinitionType**](NamespaceDefinitionType.md)> |  | [optional]
**namespace_format** | Option<**String**> | Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'. | [optional][default to null]
**prefix** | Option<**String**> | Prefix that will be prepended to the name of each stream when it is written to the destination. | [optional]
**source_id** | Option<**String**> |  | [optional]
**destination_id** | Option<**String**> |  | [optional]
**schedule** | Option<[**crate::models::ConnectionSchedule**](ConnectionSchedule.md)> |  | [optional]
**status** | Option<[**crate::models::ConnectionStatus**](ConnectionStatus.md)> |  | [optional]
**source** | Option<[**crate::models::SourceSearch**](SourceSearch.md)> |  | [optional]
**destination** | Option<[**crate::models::DestinationSearch**](DestinationSearch.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


