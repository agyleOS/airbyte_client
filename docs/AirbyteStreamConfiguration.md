# AirbyteStreamConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sync_mode** | [**crate::models::SyncMode**](SyncMode.md) |  | 
**cursor_field** | Option<**Vec<String>**> | Path to the field that will be used to determine if a record is new or modified since the last sync. This field is REQUIRED if `sync_mode` is `incremental`. Otherwise it is ignored. | [optional]
**destination_sync_mode** | [**crate::models::DestinationSyncMode**](DestinationSyncMode.md) |  | 
**primary_key** | Option<[**Vec<Vec<String>>**](array.md)> | Paths to the fields that will be used as primary key. This field is REQUIRED if `destination_sync_mode` is `*_dedup`. Otherwise it is ignored. | [optional]
**alias_name** | Option<**String**> | Alias name to the stream to be used in the destination | [optional]
**selected** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


