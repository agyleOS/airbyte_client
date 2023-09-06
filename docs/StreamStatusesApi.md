# \StreamStatusesApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stream_status**](StreamStatusesApi.md#create_stream_status) | **POST** /v1/stream_statuses/create | Creates a stream status.
[**get_stream_statuses**](StreamStatusesApi.md#get_stream_statuses) | **POST** /v1/stream_statuses/list | Gets a list of stream statuses filtered by parameters (with AND semantics).
[**update_stream_status**](StreamStatusesApi.md#update_stream_status) | **POST** /v1/stream_statuses/update | Updates a stream status.



## create_stream_status

> crate::models::StreamStatusRead create_stream_status(stream_status_create_request_body)
Creates a stream status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_status_create_request_body** | Option<[**StreamStatusCreateRequestBody**](StreamStatusCreateRequestBody.md)> |  |  |

### Return type

[**crate::models::StreamStatusRead**](StreamStatusRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_statuses

> crate::models::StreamStatusReadList get_stream_statuses(stream_status_list_request_body)
Gets a list of stream statuses filtered by parameters (with AND semantics).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_status_list_request_body** | Option<[**StreamStatusListRequestBody**](StreamStatusListRequestBody.md)> |  |  |

### Return type

[**crate::models::StreamStatusReadList**](StreamStatusReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stream_status

> crate::models::StreamStatusRead update_stream_status(stream_status_update_request_body)
Updates a stream status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_status_update_request_body** | Option<[**StreamStatusUpdateRequestBody**](StreamStatusUpdateRequestBody.md)> |  |  |

### Return type

[**crate::models::StreamStatusRead**](StreamStatusRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

