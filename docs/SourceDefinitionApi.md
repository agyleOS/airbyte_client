# \SourceDefinitionApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_source_definition**](SourceDefinitionApi.md#create_custom_source_definition) | **POST** /v1/source_definitions/create_custom | Creates a custom sourceDefinition for the given workspace
[**create_source_definition**](SourceDefinitionApi.md#create_source_definition) | **POST** /v1/source_definitions/create | Creates a sourceDefinition
[**delete_custom_source_definition**](SourceDefinitionApi.md#delete_custom_source_definition) | **POST** /v1/source_definitions/delete_custom | Delete a custom source definition for the given workspace
[**delete_source_definition**](SourceDefinitionApi.md#delete_source_definition) | **POST** /v1/source_definitions/delete | Delete a source definition
[**get_source_definition**](SourceDefinitionApi.md#get_source_definition) | **POST** /v1/source_definitions/get | Get source
[**get_source_definition_for_workspace**](SourceDefinitionApi.md#get_source_definition_for_workspace) | **POST** /v1/source_definitions/get_for_workspace | Get a sourceDefinition that is configured for the given workspace
[**grant_source_definition_to_workspace**](SourceDefinitionApi.md#grant_source_definition_to_workspace) | **POST** /v1/source_definitions/grant_definition | grant a private, non-custom sourceDefinition to a given workspace
[**list_latest_source_definitions**](SourceDefinitionApi.md#list_latest_source_definitions) | **POST** /v1/source_definitions/list_latest | List the latest sourceDefinitions Airbyte supports
[**list_private_source_definitions**](SourceDefinitionApi.md#list_private_source_definitions) | **POST** /v1/source_definitions/list_private | List all private, non-custom sourceDefinitions, and for each indicate whether the given workspace has a grant for using the definition. Used by admins to view and modify a given workspace's grants.
[**list_source_definitions**](SourceDefinitionApi.md#list_source_definitions) | **POST** /v1/source_definitions/list | List all the sourceDefinitions the current Airbyte deployment is configured to use
[**list_source_definitions_for_workspace**](SourceDefinitionApi.md#list_source_definitions_for_workspace) | **POST** /v1/source_definitions/list_for_workspace | List all the sourceDefinitions the given workspace is configured to use
[**revoke_source_definition_from_workspace**](SourceDefinitionApi.md#revoke_source_definition_from_workspace) | **POST** /v1/source_definitions/revoke_definition | revoke a grant to a private, non-custom sourceDefinition from a given workspace
[**update_custom_source_definition**](SourceDefinitionApi.md#update_custom_source_definition) | **POST** /v1/source_definitions/update_custom | Update a custom sourceDefinition for the given workspace
[**update_source_definition**](SourceDefinitionApi.md#update_source_definition) | **POST** /v1/source_definitions/update | Update a sourceDefinition



## create_custom_source_definition

> crate::models::SourceDefinitionRead create_custom_source_definition(custom_source_definition_create)
Creates a custom sourceDefinition for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_source_definition_create** | Option<[**CustomSourceDefinitionCreate**](CustomSourceDefinitionCreate.md)> |  |  |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_source_definition

> crate::models::SourceDefinitionRead create_source_definition(source_definition_create)
Creates a sourceDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_create** | Option<[**SourceDefinitionCreate**](SourceDefinitionCreate.md)> |  |  |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_source_definition

> delete_custom_source_definition(source_definition_id_with_workspace_id)
Delete a custom source definition for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_with_workspace_id** | [**SourceDefinitionIdWithWorkspaceId**](SourceDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_source_definition

> delete_source_definition(source_definition_id_request_body)
Delete a source definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_request_body** | [**SourceDefinitionIdRequestBody**](SourceDefinitionIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_definition

> crate::models::SourceDefinitionRead get_source_definition(source_definition_id_request_body)
Get source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_request_body** | [**SourceDefinitionIdRequestBody**](SourceDefinitionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_definition_for_workspace

> crate::models::SourceDefinitionRead get_source_definition_for_workspace(source_definition_id_with_workspace_id)
Get a sourceDefinition that is configured for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_with_workspace_id** | [**SourceDefinitionIdWithWorkspaceId**](SourceDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_source_definition_to_workspace

> crate::models::PrivateSourceDefinitionRead grant_source_definition_to_workspace(source_definition_id_with_workspace_id)
grant a private, non-custom sourceDefinition to a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_with_workspace_id** | [**SourceDefinitionIdWithWorkspaceId**](SourceDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::PrivateSourceDefinitionRead**](PrivateSourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_latest_source_definitions

> crate::models::SourceDefinitionReadList list_latest_source_definitions()
List the latest sourceDefinitions Airbyte supports

Guaranteed to retrieve the latest information on supported sources.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SourceDefinitionReadList**](SourceDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_source_definitions

> crate::models::PrivateSourceDefinitionReadList list_private_source_definitions(workspace_id_request_body)
List all private, non-custom sourceDefinitions, and for each indicate whether the given workspace has a grant for using the definition. Used by admins to view and modify a given workspace's grants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | Option<[**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md)> |  |  |

### Return type

[**crate::models::PrivateSourceDefinitionReadList**](PrivateSourceDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_source_definitions

> crate::models::SourceDefinitionReadList list_source_definitions()
List all the sourceDefinitions the current Airbyte deployment is configured to use

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SourceDefinitionReadList**](SourceDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_source_definitions_for_workspace

> crate::models::SourceDefinitionReadList list_source_definitions_for_workspace(workspace_id_request_body)
List all the sourceDefinitions the given workspace is configured to use

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | Option<[**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md)> |  |  |

### Return type

[**crate::models::SourceDefinitionReadList**](SourceDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_source_definition_from_workspace

> revoke_source_definition_from_workspace(source_definition_id_with_workspace_id)
revoke a grant to a private, non-custom sourceDefinition from a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_with_workspace_id** | [**SourceDefinitionIdWithWorkspaceId**](SourceDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_source_definition

> crate::models::SourceDefinitionRead update_custom_source_definition(custom_source_definition_update)
Update a custom sourceDefinition for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_source_definition_update** | Option<[**CustomSourceDefinitionUpdate**](CustomSourceDefinitionUpdate.md)> |  |  |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_source_definition

> crate::models::SourceDefinitionRead update_source_definition(source_definition_update)
Update a sourceDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_update** | Option<[**SourceDefinitionUpdate**](SourceDefinitionUpdate.md)> |  |  |

### Return type

[**crate::models::SourceDefinitionRead**](SourceDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

