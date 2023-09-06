# AirbyteStreamConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sync_mode** | [**crate::models::SyncMode**](SyncMode.md) |  | 
**cursor_field** | Option<**Vec<String>**> | Path to the field that will be used to determine if a record is new or modified since the last sync. This field is REQUIRED if `sync_mode` is `incremental`. Otherwise it is ignored. | [optional]
**destination_sync_mode** | [**crate::models::DestinationSyncMode**](DestinationSyncMode.md) |  | 
**primary_key** | Option<[**Vec<Vec<String>>**](array.md)> | Paths to the fields that will be used as primary key. This field is REQUIRED if `destination_sync_mode` is `*_dedup`. Otherwise it is ignored. | [optional]
**alias_name** | Option<**String**> | Alias name to the stream to be used in the destination | [optional]
**selected** | Option<**bool**> | If this is true, the stream is selected with all of its properties. For new connections, this considers if the stream is suggested or not | [optional]
**suggested** | Option<**bool**> | Does the connector suggest that this stream be enabled by default? | [optional]
**field_selection_enabled** | Option<**bool**> | Whether field selection should be enabled. If this is true, only the properties in `selectedFields` will be included. | [optional]
**selected_fields** | Option<[**Vec<crate::models::SelectedFieldInfo>**](SelectedFieldInfo.md)> | Paths to the fields that will be included in the configured catalog. This must be set if `fieldSelectedEnabled` is set. An empty list indicates that no properties will be included. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


