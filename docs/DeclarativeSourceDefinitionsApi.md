# \DeclarativeSourceDefinitionsApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_declarative_source_definition_manifest**](DeclarativeSourceDefinitionsApi.md#create_declarative_source_definition_manifest) | **POST** /v1/declarative_source_definitions/create_manifest | Create a declarative manifest to be used by the specified source definition
[**list_declarative_manifests**](DeclarativeSourceDefinitionsApi.md#list_declarative_manifests) | **POST** /v1/declarative_source_definitions/list_manifests | List all available declarative manifest versions of a declarative source definition
[**update_declarative_manifest_version**](DeclarativeSourceDefinitionsApi.md#update_declarative_manifest_version) | **POST** /v1/declarative_source_definitions/update_active_manifest | Update the declarative manifest version for a source



## create_declarative_source_definition_manifest

> create_declarative_source_definition_manifest(declarative_source_definition_create_manifest_request_body)
Create a declarative manifest to be used by the specified source definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**declarative_source_definition_create_manifest_request_body** | [**DeclarativeSourceDefinitionCreateManifestRequestBody**](DeclarativeSourceDefinitionCreateManifestRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_declarative_manifests

> crate::models::DeclarativeManifestsReadList list_declarative_manifests(list_declarative_manifests_request_body)
List all available declarative manifest versions of a declarative source definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_declarative_manifests_request_body** | [**ListDeclarativeManifestsRequestBody**](ListDeclarativeManifestsRequestBody.md) |  | [required] |

### Return type

[**crate::models::DeclarativeManifestsReadList**](DeclarativeManifestsReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_declarative_manifest_version

> update_declarative_manifest_version(update_active_manifest_request_body)
Update the declarative manifest version for a source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_active_manifest_request_body** | [**UpdateActiveManifestRequestBody**](UpdateActiveManifestRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

