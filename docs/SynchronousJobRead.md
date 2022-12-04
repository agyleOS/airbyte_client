# SynchronousJobRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**config_type** | [**crate::models::JobConfigType**](JobConfigType.md) |  | 
**config_id** | Option<**String**> | only present if a config id was provided. | [optional]
**created_at** | **i64** |  | 
**ended_at** | **i64** |  | 
**succeeded** | **bool** |  | 
**logs** | Option<[**crate::models::LogRead**](LogRead.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


