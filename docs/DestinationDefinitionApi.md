# \DestinationDefinitionApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_destination_definition**](DestinationDefinitionApi.md#create_custom_destination_definition) | **POST** /v1/destination_definitions/create_custom | Creates a custom destinationDefinition for the given workspace
[**delete_destination_definition**](DestinationDefinitionApi.md#delete_destination_definition) | **POST** /v1/destination_definitions/delete | Delete a destination definition
[**get_destination_definition**](DestinationDefinitionApi.md#get_destination_definition) | **POST** /v1/destination_definitions/get | Get destinationDefinition
[**get_destination_definition_for_workspace**](DestinationDefinitionApi.md#get_destination_definition_for_workspace) | **POST** /v1/destination_definitions/get_for_workspace | Get a destinationDefinition that is configured for the given workspace
[**grant_destination_definition_to_workspace**](DestinationDefinitionApi.md#grant_destination_definition_to_workspace) | **POST** /v1/destination_definitions/grant_definition | grant a private, non-custom destinationDefinition to a given workspace
[**list_destination_definitions**](DestinationDefinitionApi.md#list_destination_definitions) | **POST** /v1/destination_definitions/list | List all the destinationDefinitions the current Airbyte deployment is configured to use
[**list_destination_definitions_for_workspace**](DestinationDefinitionApi.md#list_destination_definitions_for_workspace) | **POST** /v1/destination_definitions/list_for_workspace | List all the destinationDefinitions the given workspace is configured to use
[**list_latest_destination_definitions**](DestinationDefinitionApi.md#list_latest_destination_definitions) | **POST** /v1/destination_definitions/list_latest | List the latest destinationDefinitions Airbyte supports
[**list_private_destination_definitions**](DestinationDefinitionApi.md#list_private_destination_definitions) | **POST** /v1/destination_definitions/list_private | List all private, non-custom destinationDefinitions, and for each indicate whether the given workspace has a grant for using the definition. Used by admins to view and modify a given workspace's grants.
[**revoke_destination_definition_from_workspace**](DestinationDefinitionApi.md#revoke_destination_definition_from_workspace) | **POST** /v1/destination_definitions/revoke_definition | revoke a grant to a private, non-custom destinationDefinition from a given workspace
[**update_destination_definition**](DestinationDefinitionApi.md#update_destination_definition) | **POST** /v1/destination_definitions/update | Update destinationDefinition



## create_custom_destination_definition

> crate::models::DestinationDefinitionRead create_custom_destination_definition(custom_destination_definition_create)
Creates a custom destinationDefinition for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_destination_definition_create** | Option<[**CustomDestinationDefinitionCreate**](CustomDestinationDefinitionCreate.md)> |  |  |

### Return type

[**crate::models::DestinationDefinitionRead**](DestinationDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_destination_definition

> delete_destination_definition(destination_definition_id_request_body)
Delete a destination definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_request_body** | [**DestinationDefinitionIdRequestBody**](DestinationDefinitionIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destination_definition

> crate::models::DestinationDefinitionRead get_destination_definition(destination_definition_id_request_body)
Get destinationDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_request_body** | [**DestinationDefinitionIdRequestBody**](DestinationDefinitionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionRead**](DestinationDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destination_definition_for_workspace

> crate::models::DestinationDefinitionRead get_destination_definition_for_workspace(destination_definition_id_with_workspace_id)
Get a destinationDefinition that is configured for the given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_with_workspace_id** | [**DestinationDefinitionIdWithWorkspaceId**](DestinationDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionRead**](DestinationDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_destination_definition_to_workspace

> crate::models::PrivateDestinationDefinitionRead grant_destination_definition_to_workspace(destination_definition_id_with_workspace_id)
grant a private, non-custom destinationDefinition to a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_with_workspace_id** | [**DestinationDefinitionIdWithWorkspaceId**](DestinationDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::PrivateDestinationDefinitionRead**](PrivateDestinationDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_destination_definitions

> crate::models::DestinationDefinitionReadList list_destination_definitions()
List all the destinationDefinitions the current Airbyte deployment is configured to use

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DestinationDefinitionReadList**](DestinationDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_destination_definitions_for_workspace

> crate::models::DestinationDefinitionReadList list_destination_definitions_for_workspace(workspace_id_request_body)
List all the destinationDefinitions the given workspace is configured to use

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | Option<[**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md)> |  |  |

### Return type

[**crate::models::DestinationDefinitionReadList**](DestinationDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_latest_destination_definitions

> crate::models::DestinationDefinitionReadList list_latest_destination_definitions()
List the latest destinationDefinitions Airbyte supports

Guaranteed to retrieve the latest information on supported destinations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DestinationDefinitionReadList**](DestinationDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_destination_definitions

> crate::models::PrivateDestinationDefinitionReadList list_private_destination_definitions(workspace_id_request_body)
List all private, non-custom destinationDefinitions, and for each indicate whether the given workspace has a grant for using the definition. Used by admins to view and modify a given workspace's grants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | Option<[**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md)> |  |  |

### Return type

[**crate::models::PrivateDestinationDefinitionReadList**](PrivateDestinationDefinitionReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_destination_definition_from_workspace

> revoke_destination_definition_from_workspace(destination_definition_id_with_workspace_id)
revoke a grant to a private, non-custom destinationDefinition from a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_with_workspace_id** | [**DestinationDefinitionIdWithWorkspaceId**](DestinationDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_destination_definition

> crate::models::DestinationDefinitionRead update_destination_definition(destination_definition_update)
Update destinationDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_update** | [**DestinationDefinitionUpdate**](DestinationDefinitionUpdate.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionRead**](DestinationDefinitionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

