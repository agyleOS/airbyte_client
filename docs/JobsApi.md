# \JobsApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobsApi.md#cancel_job) | **POST** /v1/jobs/cancel | Cancels a job
[**get_job_info**](JobsApi.md#get_job_info) | **POST** /v1/jobs/get | Get information about a job
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

