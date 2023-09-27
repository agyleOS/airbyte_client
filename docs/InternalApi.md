# \InternalApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_state**](InternalApi.md#create_or_update_state) | **POST** /v1/state/create_or_update | Create or update the state for a connection.
[**get_attempt_normalization_statuses_for_job**](InternalApi.md#get_attempt_normalization_statuses_for_job) | **POST** /v1/jobs/get_normalization_status | Get normalization status to determine if we can bypass normalization phase
[**save_stats**](InternalApi.md#save_stats) | **POST** /v1/attempt/save_stats | For worker to set sync stats of a running attempt.
[**save_sync_config**](InternalApi.md#save_sync_config) | **POST** /v1/attempt/save_sync_config | For worker to save the AttemptSyncConfig for an attempt.
[**set_workflow_in_attempt**](InternalApi.md#set_workflow_in_attempt) | **POST** /v1/attempt/set_workflow_in_attempt | For worker to register the workflow id in attempt.
[**write_discover_catalog_result**](InternalApi.md#write_discover_catalog_result) | **POST** /v1/sources/write_discover_catalog_result | Should only called from worker, to write result from discover activity back to DB.



## create_or_update_state

> crate::models::ConnectionState create_or_update_state(connection_state_create_or_update)
Create or update the state for a connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_state_create_or_update** | [**ConnectionStateCreateOrUpdate**](ConnectionStateCreateOrUpdate.md) |  | [required] |

### Return type

[**crate::models::ConnectionState**](ConnectionState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attempt_normalization_statuses_for_job

> crate::models::AttemptNormalizationStatusReadList get_attempt_normalization_statuses_for_job(job_id_request_body)
Get normalization status to determine if we can bypass normalization phase

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | Option<[**JobIdRequestBody**](JobIdRequestBody.md)> |  |  |

### Return type

[**crate::models::AttemptNormalizationStatusReadList**](AttemptNormalizationStatusReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## write_discover_catalog_result

> crate::models::DiscoverCatalogResult write_discover_catalog_result(source_discover_schema_write_request_body)
Should only called from worker, to write result from discover activity back to DB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_discover_schema_write_request_body** | [**SourceDiscoverSchemaWriteRequestBody**](SourceDiscoverSchemaWriteRequestBody.md) |  | [required] |

### Return type

[**crate::models::DiscoverCatalogResult**](DiscoverCatalogResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

