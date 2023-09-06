# \AttemptApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**save_stats**](AttemptApi.md#save_stats) | **POST** /v1/attempt/save_stats | For worker to set sync stats of a running attempt.
[**save_sync_config**](AttemptApi.md#save_sync_config) | **POST** /v1/attempt/save_sync_config | For worker to save the AttemptSyncConfig for an attempt.
[**set_workflow_in_attempt**](AttemptApi.md#set_workflow_in_attempt) | **POST** /v1/attempt/set_workflow_in_attempt | For worker to register the workflow id in attempt.



## save_stats

> crate::models::InternalOperationResult save_stats(save_stats_request_body)
For worker to set sync stats of a running attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_stats_request_body** | [**SaveStatsRequestBody**](SaveStatsRequestBody.md) |  | [required] |

### Return type

[**crate::models::InternalOperationResult**](InternalOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_sync_config

> crate::models::InternalOperationResult save_sync_config(save_attempt_sync_config_request_body)
For worker to save the AttemptSyncConfig for an attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_attempt_sync_config_request_body** | [**SaveAttemptSyncConfigRequestBody**](SaveAttemptSyncConfigRequestBody.md) |  | [required] |

### Return type

[**crate::models::InternalOperationResult**](InternalOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_workflow_in_attempt

> crate::models::InternalOperationResult set_workflow_in_attempt(set_workflow_in_attempt_request_body)
For worker to register the workflow id in attempt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_workflow_in_attempt_request_body** | [**SetWorkflowInAttemptRequestBody**](SetWorkflowInAttemptRequestBody.md) |  | [required] |

### Return type

[**crate::models::InternalOperationResult**](InternalOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

