# StreamStatusRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attempt_number** | **i32** |  | 
**connection_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**job_id** | **i64** |  | 
**incomplete_run_cause** | Option<[**crate::models::StreamStatusIncompleteRunCause**](StreamStatusIncompleteRunCause.md)> |  | [optional]
**job_type** | [**crate::models::StreamStatusJobType**](StreamStatusJobType.md) |  | 
**run_state** | [**crate::models::StreamStatusRunState**](StreamStatusRunState.md) |  | 
**stream_name** | **String** |  | 
**stream_namespace** | **String** |  | 
**transitioned_at** | **i64** |  | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


