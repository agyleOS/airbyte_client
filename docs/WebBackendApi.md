# \WebBackendApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**web_backend_create_connection**](WebBackendApi.md#web_backend_create_connection) | **POST** /v1/web_backend/connections/create | Create a connection
[**web_backend_get_connection**](WebBackendApi.md#web_backend_get_connection) | **POST** /v1/web_backend/connections/get | Get a connection
[**web_backend_get_workspace_state**](WebBackendApi.md#web_backend_get_workspace_state) | **POST** /v1/web_backend/workspace/state | Returns the current state of a workspace
[**web_backend_list_all_connections_for_workspace**](WebBackendApi.md#web_backend_list_all_connections_for_workspace) | **POST** /v1/web_backend/connections/list_all | Returns all connections for a workspace.
[**web_backend_list_connections_for_workspace**](WebBackendApi.md#web_backend_list_connections_for_workspace) | **POST** /v1/web_backend/connections/list | Returns all non-deleted connections for a workspace.
[**web_backend_search_connections**](WebBackendApi.md#web_backend_search_connections) | **POST** /v1/web_backend/connections/search | Search connections
[**web_backend_update_connection**](WebBackendApi.md#web_backend_update_connection) | **POST** /v1/web_backend/connections/update | Update a connection



## web_backend_create_connection

> crate::models::WebBackendConnectionRead web_backend_create_connection(web_backend_connection_create)
Create a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_connection_create** | [**WebBackendConnectionCreate**](WebBackendConnectionCreate.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionRead**](WebBackendConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_get_connection

> crate::models::WebBackendConnectionRead web_backend_get_connection(web_backend_connection_request_body)
Get a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_connection_request_body** | [**WebBackendConnectionRequestBody**](WebBackendConnectionRequestBody.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionRead**](WebBackendConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_get_workspace_state

> crate::models::WebBackendWorkspaceStateResult web_backend_get_workspace_state(web_backend_workspace_state)
Returns the current state of a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_workspace_state** | Option<[**WebBackendWorkspaceState**](WebBackendWorkspaceState.md)> |  |  |

### Return type

[**crate::models::WebBackendWorkspaceStateResult**](WebBackendWorkspaceStateResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_list_all_connections_for_workspace

> crate::models::WebBackendConnectionReadList web_backend_list_all_connections_for_workspace(workspace_id_request_body)
Returns all connections for a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionReadList**](WebBackendConnectionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_list_connections_for_workspace

> crate::models::WebBackendConnectionReadList web_backend_list_connections_for_workspace(workspace_id_request_body)
Returns all non-deleted connections for a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionReadList**](WebBackendConnectionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_search_connections

> crate::models::WebBackendConnectionReadList web_backend_search_connections(web_backend_connection_search)
Search connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_connection_search** | [**WebBackendConnectionSearch**](WebBackendConnectionSearch.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionReadList**](WebBackendConnectionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_update_connection

> crate::models::WebBackendConnectionRead web_backend_update_connection(web_backend_connection_update)
Update a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_connection_update** | [**WebBackendConnectionUpdate**](WebBackendConnectionUpdate.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionRead**](WebBackendConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

