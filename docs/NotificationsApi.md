# \NotificationsApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**try_notification_config**](NotificationsApi.md#try_notification_config) | **POST** /v1/notifications/try | Try sending a notifications



## try_notification_config

> crate::models::NotificationRead try_notification_config(notification)
Try sending a notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification** | [**Notification**](Notification.md) |  | [required] |

### Return type

[**crate::models::NotificationRead**](NotificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

