# \DestinationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_connection_to_destination**](DestinationApi.md#check_connection_to_destination) | **POST** /v1/destinations/check_connection | Check connection to the destination
[**check_connection_to_destination_for_update**](DestinationApi.md#check_connection_to_destination_for_update) | **POST** /v1/destinations/check_connection_for_update | Check connection for a proposed update to a destination
[**clone_destination**](DestinationApi.md#clone_destination) | **POST** /v1/destinations/clone | Clone destination
[**create_destination**](DestinationApi.md#create_destination) | **POST** /v1/destinations/create | Create a destination
[**delete_destination**](DestinationApi.md#delete_destination) | **POST** /v1/destinations/delete | Delete the destination
[**get_destination**](DestinationApi.md#get_destination) | **POST** /v1/destinations/get | Get configured destination
[**list_destinations_for_workspace**](DestinationApi.md#list_destinations_for_workspace) | **POST** /v1/destinations/list | List configured destinations for a workspace
[**partial_update_destination**](DestinationApi.md#partial_update_destination) | **POST** /v1/destinations/partial_update | Update a destination partially
[**search_destinations**](DestinationApi.md#search_destinations) | **POST** /v1/destinations/search | Search destinations
[**update_destination**](DestinationApi.md#update_destination) | **POST** /v1/destinations/update | Update a destination



## check_connection_to_destination

> crate::models::CheckConnectionRead check_connection_to_destination(destination_id_request_body)
Check connection to the destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id_request_body** | [**DestinationIdRequestBody**](DestinationIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_connection_to_destination_for_update

> crate::models::CheckConnectionRead check_connection_to_destination_for_update(destination_update)
Check connection for a proposed update to a destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_update** | [**DestinationUpdate**](DestinationUpdate.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_destination

> crate::models::DestinationRead clone_destination(destination_clone_request_body)
Clone destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_clone_request_body** | [**DestinationCloneRequestBody**](DestinationCloneRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationRead**](DestinationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_destination

> crate::models::DestinationRead create_destination(destination_create)
Create a destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_create** | [**DestinationCreate**](DestinationCreate.md) |  | [required] |

### Return type

[**crate::models::DestinationRead**](DestinationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_destination

> delete_destination(destination_id_request_body)
Delete the destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id_request_body** | [**DestinationIdRequestBody**](DestinationIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destination

> crate::models::DestinationRead get_destination(destination_id_request_body)
Get configured destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id_request_body** | [**DestinationIdRequestBody**](DestinationIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationRead**](DestinationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_destinations_for_workspace

> crate::models::DestinationReadList list_destinations_for_workspace(workspace_id_request_body)
List configured destinations for a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationReadList**](DestinationReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partial_update_destination

> crate::models::DestinationRead partial_update_destination(partial_destination_update)
Update a destination partially

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partial_destination_update** | [**PartialDestinationUpdate**](PartialDestinationUpdate.md) |  | [required] |

### Return type

[**crate::models::DestinationRead**](DestinationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_destinations

> crate::models::DestinationReadList search_destinations(destination_search)
Search destinations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_search** | [**DestinationSearch**](DestinationSearch.md) |  | [required] |

### Return type

[**crate::models::DestinationReadList**](DestinationReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_destination

> crate::models::DestinationRead update_destination(destination_update)
Update a destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_update** | [**DestinationUpdate**](DestinationUpdate.md) |  | [required] |

### Return type

[**crate::models::DestinationRead**](DestinationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

