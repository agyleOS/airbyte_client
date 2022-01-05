# \SourceDefinitionApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_source_definition**](SourceDefinitionApi.md#create_source_definition) | **POST** /v1/source_definitions/create | Creates a sourceDefinition
[**delete_source_definition**](SourceDefinitionApi.md#delete_source_definition) | **POST** /v1/source_definitions/delete | Delete a source definition
[**get_source_definition**](SourceDefinitionApi.md#get_source_definition) | **POST** /v1/source_definitions/get | Get source
[**list_latest_source_definitions**](SourceDefinitionApi.md#list_latest_source_definitions) | **POST** /v1/source_definitions/list_latest | List the latest sourceDefinitions Airbyte supports
[**list_source_definitions**](SourceDefinitionApi.md#list_source_definitions) | **POST** /v1/source_definitions/list | List all the sourceDefinitions the current Airbyte deployment is configured to use
[**update_source_definition**](SourceDefinitionApi.md#update_source_definition) | **POST** /v1/source_definitions/update | Update a sourceDefinition



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

