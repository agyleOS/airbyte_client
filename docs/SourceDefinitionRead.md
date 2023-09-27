# SourceDefinitionRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**docker_repository** | **String** |  | 
**docker_image_tag** | **String** |  | 
**documentation_url** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**protocol_version** | Option<**String**> | The Airbyte Protocol version supported by the connector | [optional]
**release_stage** | Option<[**crate::models::ReleaseStage**](ReleaseStage.md)> |  | [optional]
**release_date** | Option<[**String**](string.md)> | The date when this connector was first released, in yyyy-mm-dd format. | [optional]
**source_type** | Option<**String**> |  | [optional]
**resource_requirements** | Option<[**crate::models::ActorDefinitionResourceRequirements**](ActorDefinitionResourceRequirements.md)> |  | [optional]
**max_seconds_between_messages** | Option<**i64**> | Number of seconds allowed between 2 airbyte protocol messages. The source will timeout if this delay is reach | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


