# \DestinationDefinitionApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_destination_definition**](DestinationDefinitionApi.md#create_destination_definition) | **POST** /v1/destination_definitions/create | Creates a destinationsDefinition
[**delete_destination_definition**](DestinationDefinitionApi.md#delete_destination_definition) | **POST** /v1/destination_definitions/delete | Delete a destination definition
[**get_destination_definition**](DestinationDefinitionApi.md#get_destination_definition) | **POST** /v1/destination_definitions/get | Get destinationDefinition
[**list_destination_definitions**](DestinationDefinitionApi.md#list_destination_definitions) | **POST** /v1/destination_definitions/list | List all the destinationDefinitions the current Airbyte deployment is configured to use
[**list_latest_destination_definitions**](DestinationDefinitionApi.md#list_latest_destination_definitions) | **POST** /v1/destination_definitions/list_latest | List the latest destinationDefinitions Airbyte supports
[**update_destination_definition**](DestinationDefinitionApi.md#update_destination_definition) | **POST** /v1/destination_definitions/update | Update destinationDefinition



## create_destination_definition

> crate::models::DestinationDefinitionRead create_destination_definition(destination_definition_create)
Creates a destinationsDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_definition_create** | Option<[**DestinationDefinitionCreate**](DestinationDefinitionCreate.md)> |  |  |

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

