# \WebBackendApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_state_type**](WebBackendApi.md#get_state_type) | **POST** /v1/web_backend/state/get_type | Fetch the current state type for a connection.
[**web_backend_check_updates**](WebBackendApi.md#web_backend_check_updates) | **POST** /v1/web_backend/check_updates | Returns a summary of source and destination definitions that could be updated.
[**web_backend_create_connection**](WebBackendApi.md#web_backend_create_connection) | **POST** /v1/web_backend/connections/create | Create a connection
[**web_backend_get_connection**](WebBackendApi.md#web_backend_get_connection) | **POST** /v1/web_backend/connections/get | Get a connection
[**web_backend_get_workspace_state**](WebBackendApi.md#web_backend_get_workspace_state) | **POST** /v1/web_backend/workspace/state | Returns the current state of a workspace
[**web_backend_list_connections_for_workspace**](WebBackendApi.md#web_backend_list_connections_for_workspace) | **POST** /v1/web_backend/connections/list | Returns all non-deleted connections for a workspace.
[**web_backend_list_geographies**](WebBackendApi.md#web_backend_list_geographies) | **POST** /v1/web_backend/geographies/list | Returns available geographies can be selected to run data syncs in a particular geography. The 'auto' entry indicates that the sync will be automatically assigned to a geography according to the platform default behavior. Entries other than 'auto' are two-letter country codes that follow the ISO 3166-1 alpha-2 standard. 
[**web_backend_update_connection**](WebBackendApi.md#web_backend_update_connection) | **POST** /v1/web_backend/connections/update | Update a connection



## get_state_type

> crate::models::ConnectionStateType get_state_type(connection_id_request_body)
Fetch the current state type for a connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id_request_body** | [**ConnectionIdRequestBody**](ConnectionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::ConnectionStateType**](ConnectionStateType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_check_updates

> crate::models::WebBackendCheckUpdatesRead web_backend_check_updates()
Returns a summary of source and destination definitions that could be updated.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebBackendCheckUpdatesRead**](WebBackendCheckUpdatesRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## web_backend_list_connections_for_workspace

> crate::models::WebBackendConnectionReadList web_backend_list_connections_for_workspace(web_backend_connection_list_request_body)
Returns all non-deleted connections for a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_backend_connection_list_request_body** | [**WebBackendConnectionListRequestBody**](WebBackendConnectionListRequestBody.md) |  | [required] |

### Return type

[**crate::models::WebBackendConnectionReadList**](WebBackendConnectionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_list_geographies

> crate::models::WebBackendGeographiesListResult web_backend_list_geographies()
Returns available geographies can be selected to run data syncs in a particular geography. The 'auto' entry indicates that the sync will be automatically assigned to a geography according to the platform default behavior. Entries other than 'auto' are two-letter country codes that follow the ISO 3166-1 alpha-2 standard. 

Returns all available geographies in which a data sync can run.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebBackendGeographiesListResult**](WebBackendGeographiesListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_backend_update_connection

> crate::models::WebBackendConnectionRead web_backend_update_connection(web_backend_connection_update)
Update a connection

Apply a patch-style update to a connection. Only fields present on the update request body will be updated. Any operations that lack an ID will be created. Then, the newly created operationId will be applied to the connection along with the rest of the operationIds in the request body. Apply a patch-style update to a connection. Only fields present on the update request body will be updated. Note that if a catalog is present in the request body, the connection's entire catalog will be replaced with the catalog from the request. This means that to modify a single stream, the entire new catalog containing the updated stream needs to be sent. 

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

