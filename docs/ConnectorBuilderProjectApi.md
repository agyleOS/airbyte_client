# \ConnectorBuilderProjectApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_connector_builder_project**](ConnectorBuilderProjectApi.md#create_connector_builder_project) | **POST** /v1/connector_builder_projects/create | Create new connector builder project
[**delete_connector_builder_project**](ConnectorBuilderProjectApi.md#delete_connector_builder_project) | **POST** /v1/connector_builder_projects/delete | Deletes connector builder project
[**get_connector_builder_project**](ConnectorBuilderProjectApi.md#get_connector_builder_project) | **POST** /v1/connector_builder_projects/get_with_manifest | Get a connector builder project with draft manifest
[**list_connector_builder_projects**](ConnectorBuilderProjectApi.md#list_connector_builder_projects) | **POST** /v1/connector_builder_projects/list | List connector builder projects for workspace
[**publish_connector_builder_project**](ConnectorBuilderProjectApi.md#publish_connector_builder_project) | **POST** /v1/connector_builder_projects/publish | Publish a connector to the workspace
[**update_connector_builder_project**](ConnectorBuilderProjectApi.md#update_connector_builder_project) | **POST** /v1/connector_builder_projects/update | Update connector builder project



## create_connector_builder_project

> crate::models::ConnectorBuilderProjectIdWithWorkspaceId create_connector_builder_project(connector_builder_project_with_workspace_id)
Create new connector builder project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_builder_project_with_workspace_id** | [**ConnectorBuilderProjectWithWorkspaceId**](ConnectorBuilderProjectWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::ConnectorBuilderProjectIdWithWorkspaceId**](ConnectorBuilderProjectIdWithWorkspaceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connector_builder_project

> delete_connector_builder_project(connector_builder_project_id_with_workspace_id)
Deletes connector builder project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_builder_project_id_with_workspace_id** | [**ConnectorBuilderProjectIdWithWorkspaceId**](ConnectorBuilderProjectIdWithWorkspaceId.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connector_builder_project

> crate::models::ConnectorBuilderProjectRead get_connector_builder_project(connector_builder_project_id_with_workspace_id)
Get a connector builder project with draft manifest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_builder_project_id_with_workspace_id** | [**ConnectorBuilderProjectIdWithWorkspaceId**](ConnectorBuilderProjectIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::ConnectorBuilderProjectRead**](ConnectorBuilderProjectRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connector_builder_projects

> crate::models::ConnectorBuilderProjectReadList list_connector_builder_projects(workspace_id_request_body)
List connector builder projects for workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::ConnectorBuilderProjectReadList**](ConnectorBuilderProjectReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_connector_builder_project

> crate::models::SourceDefinitionIdBody publish_connector_builder_project(connector_builder_publish_request_body)
Publish a connector to the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_builder_publish_request_body** | [**ConnectorBuilderPublishRequestBody**](ConnectorBuilderPublishRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceDefinitionIdBody**](SourceDefinitionIdBody.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connector_builder_project

> update_connector_builder_project(existing_connector_builder_project_with_workspace_id)
Update connector builder project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**existing_connector_builder_project_with_workspace_id** | [**ExistingConnectorBuilderProjectWithWorkspaceId**](ExistingConnectorBuilderProjectWithWorkspaceId.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

