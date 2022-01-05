# \OperationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_operation**](OperationApi.md#check_operation) | **POST** /v1/operations/check | Check if an operation to be created is valid
[**create_operation**](OperationApi.md#create_operation) | **POST** /v1/operations/create | Create an operation to be applied as part of a connection pipeline
[**delete_operation**](OperationApi.md#delete_operation) | **POST** /v1/operations/delete | Delete an operation
[**get_operation**](OperationApi.md#get_operation) | **POST** /v1/operations/get | Returns an operation
[**list_operations_for_connection**](OperationApi.md#list_operations_for_connection) | **POST** /v1/operations/list | Returns all operations for a connection.
[**update_operation**](OperationApi.md#update_operation) | **POST** /v1/operations/update | Update an operation



## check_operation

> crate::models::CheckOperationRead check_operation(operator_configuration)
Check if an operation to be created is valid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operator_configuration** | [**OperatorConfiguration**](OperatorConfiguration.md) |  | [required] |

### Return type

[**crate::models::CheckOperationRead**](CheckOperationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_operation

> crate::models::OperationRead create_operation(operation_create)
Create an operation to be applied as part of a connection pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operation_create** | [**OperationCreate**](OperationCreate.md) |  | [required] |

### Return type

[**crate::models::OperationRead**](OperationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_operation

> delete_operation(operation_id_request_body)
Delete an operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operation_id_request_body** | [**OperationIdRequestBody**](OperationIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_operation

> crate::models::OperationRead get_operation(operation_id_request_body)
Returns an operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operation_id_request_body** | [**OperationIdRequestBody**](OperationIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::OperationRead**](OperationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_operations_for_connection

> crate::models::OperationReadList list_operations_for_connection(connection_id_request_body)
Returns all operations for a connection.

List operations for connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id_request_body** | [**ConnectionIdRequestBody**](ConnectionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::OperationReadList**](OperationReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_operation

> crate::models::OperationRead update_operation(operation_update)
Update an operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operation_update** | [**OperationUpdate**](OperationUpdate.md) |  | [required] |

### Return type

[**crate::models::OperationRead**](OperationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

