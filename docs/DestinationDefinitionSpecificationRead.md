# DestinationDefinitionSpecificationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**documentation_url** | Option<**String**> |  | [optional]
**connection_specification** | Option<[**serde_json::Value**](.md)> | The specification for what values are required to configure the destinationDefinition. | [optional]
**auth_specification** | Option<[**crate::models::AuthSpecification**](AuthSpecification.md)> |  | [optional]
**advanced_auth** | Option<[**crate::models::AdvancedAuth**](AdvancedAuth.md)> |  | [optional]
**job_info** | [**crate::models::SynchronousJobRead**](SynchronousJobRead.md) |  | 
**supported_destination_sync_modes** | Option<[**Vec<crate::models::DestinationSyncMode>**](DestinationSyncMode.md)> |  | [optional]
**supports_dbt** | Option<**bool**> |  | [optional]
**supports_normalization** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


