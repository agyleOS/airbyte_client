# \WorkspaceApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workspace**](WorkspaceApi.md#create_workspace) | **POST** /v1/workspaces/create | Creates a workspace
[**delete_workspace**](WorkspaceApi.md#delete_workspace) | **POST** /v1/workspaces/delete | Deletes a workspace
[**get_workspace**](WorkspaceApi.md#get_workspace) | **POST** /v1/workspaces/get | Find workspace by ID
[**get_workspace_by_slug**](WorkspaceApi.md#get_workspace_by_slug) | **POST** /v1/workspaces/get_by_slug | Find workspace by slug
[**list_workspaces**](WorkspaceApi.md#list_workspaces) | **POST** /v1/workspaces/list | List all workspaces registered in the current Airbyte deployment
[**update_workspace**](WorkspaceApi.md#update_workspace) | **POST** /v1/workspaces/update | Update workspace state
[**update_workspace_feedback**](WorkspaceApi.md#update_workspace_feedback) | **POST** /v1/workspaces/tag_feedback_status_as_done | Update workspace feedback state



## create_workspace

> crate::models::WorkspaceRead create_workspace(workspace_create)
Creates a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_create** | [**WorkspaceCreate**](WorkspaceCreate.md) |  | [required] |

### Return type

[**crate::models::WorkspaceRead**](WorkspaceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workspace

> delete_workspace(workspace_id_request_body)
Deletes a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace

> crate::models::WorkspaceRead get_workspace(workspace_id_request_body)
Find workspace by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::WorkspaceRead**](WorkspaceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_by_slug

> crate::models::WorkspaceRead get_workspace_by_slug(slug_request_body)
Find workspace by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug_request_body** | [**SlugRequestBody**](SlugRequestBody.md) |  | [required] |

### Return type

[**crate::models::WorkspaceRead**](WorkspaceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workspaces

> crate::models::WorkspaceReadList list_workspaces()
List all workspaces registered in the current Airbyte deployment

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkspaceReadList**](WorkspaceReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace

> crate::models::WorkspaceRead update_workspace(workspace_update)
Update workspace state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_update** | [**WorkspaceUpdate**](WorkspaceUpdate.md) |  | [required] |

### Return type

[**crate::models::WorkspaceRead**](WorkspaceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace_feedback

> update_workspace_feedback(workspace_give_feedback)
Update workspace feedback state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_give_feedback** | [**WorkspaceGiveFeedback**](WorkspaceGiveFeedback.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

