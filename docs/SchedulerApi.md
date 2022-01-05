# \SchedulerApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_destination_check_connection**](SchedulerApi.md#execute_destination_check_connection) | **POST** /v1/scheduler/destinations/check_connection | Run check connection for a given destination configuration
[**execute_source_check_connection**](SchedulerApi.md#execute_source_check_connection) | **POST** /v1/scheduler/sources/check_connection | Run check connection for a given source configuration
[**execute_source_discover_schema**](SchedulerApi.md#execute_source_discover_schema) | **POST** /v1/scheduler/sources/discover_schema | Run discover schema for a given source a source configuration



## execute_destination_check_connection

> crate::models::CheckConnectionRead execute_destination_check_connection(destination_core_config)
Run check connection for a given destination configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_core_config** | [**DestinationCoreConfig**](DestinationCoreConfig.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_source_check_connection

> crate::models::CheckConnectionRead execute_source_check_connection(source_core_config)
Run check connection for a given source configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_core_config** | [**SourceCoreConfig**](SourceCoreConfig.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_source_discover_schema

> crate::models::SourceDiscoverSchemaRead execute_source_discover_schema(source_core_config)
Run discover schema for a given source a source configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_core_config** | [**SourceCoreConfig**](SourceCoreConfig.md) |  | [required] |

### Return type

[**crate::models::SourceDiscoverSchemaRead**](SourceDiscoverSchemaRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

