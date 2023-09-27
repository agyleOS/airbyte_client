# \SourceDefinitionSpecificationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_source_definition_specification**](SourceDefinitionSpecificationApi.md#get_source_definition_specification) | **POST** /v1/source_definition_specifications/get | Get specification for a SourceDefinition.
[**get_specification_for_source_id**](SourceDefinitionSpecificationApi.md#get_specification_for_source_id) | **POST** /v1/source_definition_specifications/get_for_source | Get specification for a source.



## get_source_definition_specification

> crate::models::SourceDefinitionSpecificationRead get_source_definition_specification(source_definition_id_with_workspace_id)
Get specification for a SourceDefinition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_definition_id_with_workspace_id** | [**SourceDefinitionIdWithWorkspaceId**](SourceDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::SourceDefinitionSpecificationRead**](SourceDefinitionSpecificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specification_for_source_id

> crate::models::SourceDefinitionSpecificationRead get_specification_for_source_id(source_id_request_body)
Get specification for a source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_id_request_body** | [**SourceIdRequestBody**](SourceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceDefinitionSpecificationRead**](SourceDefinitionSpecificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

