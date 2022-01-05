# \DestinationDefinitionSpecificationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_destination_definition_specification**](DestinationDefinitionSpecificationApi.md#get_destination_definition_specification) | **POST** /v1/destination_definition_specifications/get | Get specification for a destinationDefinition



## get_destination_definition_specification

> crate::models::DestinationDefinitionSpecificationRead get_destination_definition_specification(destination_definition_id_request_body)
Get specification for a destinationDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_id_request_body** | [**DestinationDefinitionIdRequestBody**](DestinationDefinitionIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::DestinationDefinitionSpecificationRead**](DestinationDefinitionSpecificationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

