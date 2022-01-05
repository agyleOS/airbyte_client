# \DeploymentApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_archive**](DeploymentApi.md#export_archive) | **POST** /v1/deployment/export | Export Airbyte Configuration and Data Archive
[**export_workspace**](DeploymentApi.md#export_workspace) | **POST** /v1/deployment/export_workspace | Export Airbyte Workspace Configuration
[**import_archive**](DeploymentApi.md#import_archive) | **POST** /v1/deployment/import | Import Airbyte Configuration and Data Archive
[**import_into_workspace**](DeploymentApi.md#import_into_workspace) | **POST** /v1/deployment/import_into_workspace | Import Airbyte Configuration into Workspace (this operation might change ids of imported configurations). Note, in order to use this api endpoint, you might need to upload a temporary archive resource with 'deployment/upload_archive_resource' first 
[**upload_archive_resource**](DeploymentApi.md#upload_archive_resource) | **POST** /v1/deployment/upload_archive_resource | Upload a GZIP archive tarball and stage it in the server's cache as a temporary resource



## export_archive

> std::path::PathBuf export_archive()
Export Airbyte Configuration and Data Archive

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-gzip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_workspace

> std::path::PathBuf export_workspace(workspace_id_request_body)
Export Airbyte Workspace Configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id_request_body** | [**WorkspaceIdRequestBody**](WorkspaceIdRequestBody.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/x-gzip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_archive

> crate::models::ImportRead import_archive(body)
Import Airbyte Configuration and Data Archive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::ImportRead**](ImportRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-gzip
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_into_workspace

> crate::models::ImportRead import_into_workspace(import_request_body)
Import Airbyte Configuration into Workspace (this operation might change ids of imported configurations). Note, in order to use this api endpoint, you might need to upload a temporary archive resource with 'deployment/upload_archive_resource' first 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_request_body** | [**ImportRequestBody**](ImportRequestBody.md) |  | [required] |

### Return type

[**crate::models::ImportRead**](ImportRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_archive_resource

> crate::models::UploadRead upload_archive_resource(body)
Upload a GZIP archive tarball and stage it in the server's cache as a temporary resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::UploadRead**](UploadRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-gzip
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

