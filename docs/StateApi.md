# \StateApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_state**](StateApi.md#create_or_update_state) | **POST** /v1/state/create_or_update | Create or update the state for a connection.
[**get_state**](StateApi.md#get_state) | **POST** /v1/state/get | Fetch the current state for a connection.



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


## get_state

> crate::models::ConnectionState get_state(connection_id_request_body)
Fetch the current state for a connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id_request_body** | [**ConnectionIdRequestBody**](ConnectionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::ConnectionState**](ConnectionState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

