# AttemptSyncConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the source. The schema for this must match the schema return by source_definition_specifications/get for the source. | 
**destination_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the destination. The schema for this must match the schema return by destination_definition_specifications/get for the destinationDefinition. | 
**state** | Option<[**crate::models::ConnectionState**](ConnectionState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


