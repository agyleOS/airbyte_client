# WorkspaceUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**email** | Option<**String**> |  | [optional]
**initial_setup_complete** | Option<**bool**> |  | [optional]
**display_setup_wizard** | Option<**bool**> |  | [optional]
**anonymous_data_collection** | Option<**bool**> |  | [optional]
**news** | Option<**bool**> |  | [optional]
**security_updates** | Option<**bool**> |  | [optional]
**notifications** | Option<[**Vec<crate::models::Notification>**](Notification.md)> |  | [optional]
**notification_settings** | Option<[**crate::models::NotificationSettings**](NotificationSettings.md)> |  | [optional]
**default_geography** | Option<[**crate::models::Geography**](Geography.md)> |  | [optional]
**webhook_configs** | Option<[**Vec<crate::models::WebhookConfigWrite>**](WebhookConfigWrite.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


