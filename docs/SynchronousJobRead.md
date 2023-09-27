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
**connector_configuration_updated** | Option<**bool**> |  | [optional][default to false]
**logs** | Option<[**crate::models::LogRead**](LogRead.md)> |  | [optional]
**failure_reason** | Option<[**crate::models::FailureReason**](FailureReason.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


