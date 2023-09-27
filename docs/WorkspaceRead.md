# WorkspaceRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**email** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**slug** | **String** |  | 
**initial_setup_complete** | **bool** |  | 
**display_setup_wizard** | Option<**bool**> |  | [optional]
**anonymous_data_collection** | Option<**bool**> |  | [optional]
**news** | Option<**bool**> |  | [optional]
**security_updates** | Option<**bool**> |  | [optional]
**notifications** | Option<[**Vec<crate::models::Notification>**](Notification.md)> |  | [optional]
**notification_settings** | Option<[**crate::models::NotificationSettings**](NotificationSettings.md)> |  | [optional]
**first_completed_sync** | Option<**bool**> |  | [optional]
**feedback_done** | Option<**bool**> |  | [optional]
**default_geography** | Option<[**crate::models::Geography**](Geography.md)> |  | [optional]
**webhook_configs** | Option<[**Vec<crate::models::WebhookConfigRead>**](WebhookConfigRead.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


