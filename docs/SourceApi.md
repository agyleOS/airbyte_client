# \SourceApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_connection_to_source**](SourceApi.md#check_connection_to_source) | **POST** /v1/sources/check_connection | Check connection to the source
[**check_connection_to_source_for_update**](SourceApi.md#check_connection_to_source_for_update) | **POST** /v1/sources/check_connection_for_update | Check connection for a proposed update to a source
[**clone_source**](SourceApi.md#clone_source) | **POST** /v1/sources/clone | Clone source
[**create_source**](SourceApi.md#create_source) | **POST** /v1/sources/create | Create a source
[**delete_source**](SourceApi.md#delete_source) | **POST** /v1/sources/delete | Delete a source
[**discover_schema_for_source**](SourceApi.md#discover_schema_for_source) | **POST** /v1/sources/discover_schema | Discover the schema catalog of the source
[**get_source**](SourceApi.md#get_source) | **POST** /v1/sources/get | Get source
[**list_sources_for_workspace**](SourceApi.md#list_sources_for_workspace) | **POST** /v1/sources/list | List sources for workspace
[**search_sources**](SourceApi.md#search_sources) | **POST** /v1/sources/search | Search sources
[**update_source**](SourceApi.md#update_source) | **POST** /v1/sources/update | Update a source



## check_connection_to_source

> crate::models::CheckConnectionRead check_connection_to_source(source_id_request_body)
Check connection to the source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_id_request_body** | [**SourceIdRequestBody**](SourceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_connection_to_source_for_update

> crate::models::CheckConnectionRead check_connection_to_source_for_update(source_update)
Check connection for a proposed update to a source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_update** | [**SourceUpdate**](SourceUpdate.md) |  | [required] |

### Return type

[**crate::models::CheckConnectionRead**](CheckConnectionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_source

> crate::models::SourceRead clone_source(source_clone_request_body)
Clone source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_clone_request_body** | [**SourceCloneRequestBody**](SourceCloneRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceRead**](SourceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_source

> crate::models::SourceRead create_source(source_create)
Create a source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_create** | [**SourceCreate**](SourceCreate.md) |  | [required] |

### Return type

[**crate::models::SourceRead**](SourceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_source

> delete_source(source_id_request_body)
Delete a source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_id_request_body** | [**SourceIdRequestBody**](SourceIdRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discover_schema_for_source

> crate::models::SourceDiscoverSchemaRead discover_schema_for_source(source_discover_schema_request_body)
Discover the schema catalog of the source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_discover_schema_request_body** | [**SourceDiscoverSchemaRequestBody**](SourceDiscoverSchemaRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceDiscoverSchemaRead**](SourceDiscoverSchemaRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source

> crate::models::SourceRead get_source(source_id_request_body)
Get source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_id_request_body** | [**SourceIdRequestBody**](SourceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceRead**](SourceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sources_for_workspace

> crate::models::SourceReadList list_sources_for_workspace(workspace_id_request_body)
List sources for workspace

List sources for workspace. Does not return deleted sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**crate::models::SourceReadList**](SourceReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_sources

> crate::models::SourceReadList search_sources(source_search)
Search sources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_search** | [**SourceSearch**](SourceSearch.md) |  | [required] |

### Return type

[**crate::models::SourceReadList**](SourceReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_source

> crate::models::SourceRead update_source(source_update)
Update a source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_update** | [**SourceUpdate**](SourceUpdate.md) |  | [required] |

### Return type

[**crate::models::SourceRead**](SourceRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

