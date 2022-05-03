# AttemptRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**status** | [**crate::models::AttemptStatus**](AttemptStatus.md) |  | 
**created_at** | **i64** |  | 
**updated_at** | **i64** |  | 
**ended_at** | Option<**i64**> |  | [optional]
**bytes_synced** | Option<**i64**> |  | [optional]
**records_synced** | Option<**i64**> |  | [optional]
**total_stats** | Option<[**crate::models::AttemptStats**](AttemptStats.md)> |  | [optional]
**stream_stats** | Option<[**Vec<crate::models::AttemptStreamStats>**](AttemptStreamStats.md)> |  | [optional]
**failure_summary** | Option<[**crate::models::AttemptFailureSummary**](AttemptFailureSummary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


