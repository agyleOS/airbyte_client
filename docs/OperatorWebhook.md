# OperatorWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_config_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the webhook configs to use from the workspace. | [optional]
**webhook_type** | Option<**String**> |  | [optional]
**dbt_cloud** | Option<[**crate::models::OperatorWebhookDbtCloud**](OperatorWebhook_dbtCloud.md)> |  | [optional]
**execution_url** | Option<**String**> | DEPRECATED. Populate dbtCloud instead. | [optional]
**execution_body** | Option<**String**> | DEPRECATED. Populate dbtCloud instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


