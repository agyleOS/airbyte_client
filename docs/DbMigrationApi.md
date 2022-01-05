# \DbMigrationApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_migrations**](DbMigrationApi.md#execute_migrations) | **POST** /v1/db_migrations/migrate | Migrate the database to the latest version
[**list_migrations**](DbMigrationApi.md#list_migrations) | **POST** /v1/db_migrations/list | List all database migrations



## execute_migrations

> crate::models::DbMigrationExecutionRead execute_migrations(db_migration_request_body)
Migrate the database to the latest version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db_migration_request_body** | [**DbMigrationRequestBody**](DbMigrationRequestBody.md) |  | [required] |

### Return type

[**crate::models::DbMigrationExecutionRead**](DbMigrationExecutionRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_migrations

> crate::models::DbMigrationReadList list_migrations(db_migration_request_body)
List all database migrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db_migration_request_body** | [**DbMigrationRequestBody**](DbMigrationRequestBody.md) |  | [required] |

### Return type

[**crate::models::DbMigrationReadList**](DbMigrationReadList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

