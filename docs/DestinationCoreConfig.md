# DestinationCoreConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**destination_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**connection_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the destination. The schema for this must match the schema return by destination_definition_specifications/get for the destinationDefinition. | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


