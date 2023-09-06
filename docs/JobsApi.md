# \JobsApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobsApi.md#cancel_job) | **POST** /v1/jobs/cancel | Cancels a job
[**get_attempt_normalization_statuses_for_job**](JobsApi.md#get_attempt_normalization_statuses_for_job) | **POST** /v1/jobs/get_normalization_status | Get normalization status to determine if we can bypass normalization phase
[**get_job_debug_info**](JobsApi.md#get_job_debug_info) | **POST** /v1/jobs/get_debug_info | Gets all information needed to debug this job
[**get_job_info**](JobsApi.md#get_job_info) | **POST** /v1/jobs/get | Get information about a job
[**get_job_info_light**](JobsApi.md#get_job_info_light) | **POST** /v1/jobs/get_light | Get information about a job excluding attempt info and logs
[**get_job_info_without_logs**](JobsApi.md#get_job_info_without_logs) | **POST** /v1/jobs/get_without_logs | Get information about a job excluding logs
[**get_last_replication_job**](JobsApi.md#get_last_replication_job) | **POST** /v1/jobs/get_last_replication_job | 
[**list_jobs_for**](JobsApi.md#list_jobs_for) | **POST** /v1/jobs/list | Returns recent jobs for a connection. Jobs are returned in descending order by createdAt.



## cancel_job

> crate::models::JobInfoRead cancel_job(job_id_request_body)
Cancels a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | [**JobIdRequestBody**](JobIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobInfoRead**](JobInfoRead.md)

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


## get_job_debug_info

> crate::models::JobDebugInfoRead get_job_debug_info(job_id_request_body)
Gets all information needed to debug this job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | [**JobIdRequestBody**](JobIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobDebugInfoRead**](JobDebugInfoRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_info

> crate::models::JobInfoRead get_job_info(job_id_request_body)
Get information about a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | [**JobIdRequestBody**](JobIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobInfoRead**](JobInfoRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_info_light

> crate::models::JobInfoLightRead get_job_info_light(job_id_request_body)
Get information about a job excluding attempt info and logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | [**JobIdRequestBody**](JobIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobInfoLightRead**](JobInfoLightRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_info_without_logs

> crate::models::JobInfoRead get_job_info_without_logs(job_id_request_body)
Get information about a job excluding logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id_request_body** | [**JobIdRequestBody**](JobIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobInfoRead**](JobInfoRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_replication_job

> crate::models::JobOptionalRead get_last_replication_job(connection_id_request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id_request_body** | [**ConnectionIdRequestBody**](ConnectionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobOptionalRead**](JobOptionalRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs_for

> crate::models::JobReadList list_jobs_for(job_list_request_body)
Returns recent jobs for a connection. Jobs are returned in descending order by createdAt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_list_request_body** | [**JobListRequestBody**](JobListRequestBody.md) |  | [required] |

### Return type

[**crate::models::JobReadList**](JobReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

