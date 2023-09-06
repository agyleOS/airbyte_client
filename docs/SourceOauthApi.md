# \SourceOauthApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_source_o_auth**](SourceOauthApi.md#complete_source_o_auth) | **POST** /v1/source_oauths/complete_oauth | Given a source def ID generate an access/refresh token etc.
[**get_source_o_auth_consent**](SourceOauthApi.md#get_source_o_auth_consent) | **POST** /v1/source_oauths/get_consent_url | Given a source connector definition ID, return the URL to the consent screen where to redirect the user to.
[**revoke_source_o_auth_tokens**](SourceOauthApi.md#revoke_source_o_auth_tokens) | **POST** /v1/source_oauths/revoke | Given a source definition ID and workspace ID revoke access/refresh token etc.
[**set_instancewide_source_oauth_params**](SourceOauthApi.md#set_instancewide_source_oauth_params) | **POST** /v1/source_oauths/oauth_params/create | Sets instancewide variables to be used for the oauth flow when creating this source. When set, these variables will be injected into a connector's configuration before any interaction with the connector image itself. This enables running oauth flows with consistent variables e.g: the company's Google Ads developer_token, client_id, and client_secret without the user having to know about these variables. 



## complete_source_o_auth

> crate::models::CompleteOAuthResponse complete_source_o_auth(complete_source_oauth_request)
Given a source def ID generate an access/refresh token etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**complete_source_oauth_request** | [**CompleteSourceOauthRequest**](CompleteSourceOauthRequest.md) |  | [required] |

### Return type

[**crate::models::CompleteOAuthResponse**](CompleteOAuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_o_auth_consent

> crate::models::OAuthConsentRead get_source_o_auth_consent(source_oauth_consent_request)
Given a source connector definition ID, return the URL to the consent screen where to redirect the user to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_oauth_consent_request** | [**SourceOauthConsentRequest**](SourceOauthConsentRequest.md) |  | [required] |

### Return type

[**crate::models::OAuthConsentRead**](OAuthConsentRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_source_o_auth_tokens

> revoke_source_o_auth_tokens(revoke_source_oauth_tokens_request)
Given a source definition ID and workspace ID revoke access/refresh token etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_source_oauth_tokens_request** | [**RevokeSourceOauthTokensRequest**](RevokeSourceOauthTokensRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instancewide_source_oauth_params

> set_instancewide_source_oauth_params(set_instancewide_source_oauth_params_request_body)
Sets instancewide variables to be used for the oauth flow when creating this source. When set, these variables will be injected into a connector's configuration before any interaction with the connector image itself. This enables running oauth flows with consistent variables e.g: the company's Google Ads developer_token, client_id, and client_secret without the user having to know about these variables. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_instancewide_source_oauth_params_request_body** | [**SetInstancewideSourceOauthParamsRequestBody**](SetInstancewideSourceOauthParamsRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

