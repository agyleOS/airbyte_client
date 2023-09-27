# SourceCoreConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**source_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**connection_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the source. The schema for this must match the schema return by source_definition_specifications/get for the source. | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


