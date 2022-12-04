# DestinationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**destination_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**connection_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the destination. The schema for this must match the schema return by destination_definition_specifications/get for the destinationDefinition. | 
**name** | **String** |  | 
**destination_name** | **String** |  | 
**icon** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


