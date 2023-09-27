# JobRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**config_type** | [**crate::models::JobConfigType**](JobConfigType.md) |  | 
**config_id** | **String** |  | 
**enabled_streams** | Option<[**Vec<crate::models::StreamDescriptor>**](StreamDescriptor.md)> |  | [optional]
**created_at** | **i64** |  | 
**updated_at** | **i64** |  | 
**started_at** | Option<**i64**> |  | [optional]
**status** | [**crate::models::JobStatus**](JobStatus.md) |  | 
**reset_config** | Option<[**crate::models::ResetConfig**](ResetConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


