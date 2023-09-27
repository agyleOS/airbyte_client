# FailureReason

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failure_origin** | Option<[**crate::models::FailureOrigin**](FailureOrigin.md)> |  | [optional]
**failure_type** | Option<[**crate::models::FailureType**](FailureType.md)> |  | [optional]
**external_message** | Option<**String**> |  | [optional]
**internal_message** | Option<**String**> |  | [optional]
**stacktrace** | Option<**String**> |  | [optional]
**retryable** | Option<**bool**> | True if it is known that retrying may succeed, e.g. for a transient failure. False if it is known that a retry will not succeed, e.g. for a configuration issue. If not set, retryable status is not well known. | [optional]
**timestamp** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


