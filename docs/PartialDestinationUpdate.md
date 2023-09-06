# PartialDestinationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**connection_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure the destination. The schema for this must match the schema return by destination_definition_specifications/get for the destinationDefinition. | [optional]
**name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


