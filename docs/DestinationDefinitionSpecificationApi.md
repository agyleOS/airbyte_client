# \DestinationDefinitionSpecificationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_destination_definition_specification**](DestinationDefinitionSpecificationApi.md#get_destination_definition_specification) | **POST** /v1/destination_definition_specifications/get | Get specification for a destinationDefinition
[**get_specification_for_destination_id**](DestinationDefinitionSpecificationApi.md#get_specification_for_destination_id) | **POST** /v1/destination_definition_specifications/get_for_destination | Get specification for a destination



## get_destination_definition_specification

> crate::models::DestinationDefinitionSpecificationRead get_destination_definition_specification(destination_definition_id_with_workspace_id)
Get specification for a destinationDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_with_workspace_id** | [**DestinationDefinitionIdWithWorkspaceId**](DestinationDefinitionIdWithWorkspaceId.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionSpecificationRead**](DestinationDefinitionSpecificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specification_for_destination_id

> crate::models::DestinationDefinitionSpecificationRead get_specification_for_destination_id(destination_id_request_body)
Get specification for a destination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id_request_body** | [**DestinationIdRequestBody**](DestinationIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionSpecificationRead**](DestinationDefinitionSpecificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

