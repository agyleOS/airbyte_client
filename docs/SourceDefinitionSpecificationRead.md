# SourceDefinitionSpecificationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**documentation_url** | Option<**String**> |  | [optional]
**connection_specification** | Option<[**serde_json::Value**](.md)> | The specification for what values are required to configure the sourceDefinition. | [optional]
**advanced_auth** | Option<[**crate::models::AdvancedAuth**](AdvancedAuth.md)> |  | [optional]
**job_info** | [**crate::models::SynchronousJobRead**](SynchronousJobRead.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


