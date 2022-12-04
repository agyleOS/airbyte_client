# ConnectionState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state_type** | [**crate::models::ConnectionStateType**](ConnectionStateType.md) |  | 
**connection_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**state** | Option<[**serde_json::Value**](.md)> |  | [optional]
**stream_state** | Option<[**Vec<crate::models::StreamState>**](StreamState.md)> |  | [optional]
**global_state** | Option<[**crate::models::GlobalState**](GlobalState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


