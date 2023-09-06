# DestinationDefinitionRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**docker_repository** | **String** |  | 
**docker_image_tag** | **String** |  | 
**documentation_url** | **String** |  | 
**icon** | Option<**String**> |  | [optional]
**protocol_version** | Option<**String**> | The Airbyte Protocol version supported by the connector | [optional]
**release_stage** | Option<[**crate::models::ReleaseStage**](ReleaseStage.md)> |  | [optional]
**release_date** | Option<[**String**](string.md)> | The date when this connector was first released, in yyyy-mm-dd format. | [optional]
**resource_requirements** | Option<[**crate::models::ActorDefinitionResourceRequirements**](ActorDefinitionResourceRequirements.md)> |  | [optional]
**supports_dbt** | **bool** | an optional flag indicating whether DBT is used in the normalization. If the flag value is NULL - DBT is not used. | 
**normalization_config** | [**crate::models::NormalizationDestinationDefinitionConfig**](NormalizationDestinationDefinitionConfig.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


