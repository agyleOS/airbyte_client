# WebBackendConnectionListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**destination_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**schedule_type** | Option<[**crate::models::ConnectionScheduleType**](ConnectionScheduleType.md)> |  | [optional]
**schedule_data** | Option<[**crate::models::ConnectionScheduleData**](ConnectionScheduleData.md)> |  | [optional]
**status** | [**crate::models::ConnectionStatus**](ConnectionStatus.md) |  | 
**source** | [**crate::models::SourceRead**](SourceRead.md) |  | 
**destination** | [**crate::models::DestinationRead**](DestinationRead.md) |  | 
**latest_sync_job_created_at** | Option<**i64**> | epoch time of the latest sync job. null if no sync job has taken place. | [optional]
**latest_sync_job_status** | Option<[**crate::models::JobStatus**](JobStatus.md)> |  | [optional]
**is_syncing** | **bool** |  | 
**schema_change** | [**crate::models::SchemaChange**](SchemaChange.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


