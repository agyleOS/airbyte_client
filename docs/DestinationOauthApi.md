# \DestinationOauthApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_destination_o_auth**](DestinationOauthApi.md#complete_destination_o_auth) | **POST** /v1/destination_oauths/complete_oauth | Given a destination def ID generate an access/refresh token etc.
[**get_destination_o_auth_consent**](DestinationOauthApi.md#get_destination_o_auth_consent) | **POST** /v1/destination_oauths/get_consent_url | Given a destination connector definition ID, return the URL to the consent screen where to redirect the user to.
[**set_instancewide_destination_oauth_params**](DestinationOauthApi.md#set_instancewide_destination_oauth_params) | **POST** /v1/destination_oauths/oauth_params/create | Sets instancewide variables to be used for the oauth flow when creating this destination. When set, these variables will be injected into a connector's configuration before any interaction with the connector image itself. This enables running oauth flows with consistent variables e.g: the company's Google Ads developer_token, client_id, and client_secret without the user having to know about these variables. 



## complete_destination_o_auth

> crate::models::CompleteOAuthResponse complete_destination_o_auth(complete_destination_o_auth_request)
Given a destination def ID generate an access/refresh token etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**complete_destination_o_auth_request** | [**CompleteDestinationOAuthRequest**](CompleteDestinationOAuthRequest.md) |  | [required] |

### Return type

[**crate::models::CompleteOAuthResponse**](CompleteOAuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destination_o_auth_consent

> crate::models::OAuthConsentRead get_destination_o_auth_consent(destination_oauth_consent_request)
Given a destination connector definition ID, return the URL to the consent screen where to redirect the user to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_oauth_consent_request** | [**DestinationOauthConsentRequest**](DestinationOauthConsentRequest.md) |  | [required] |

### Return type

[**crate::models::OAuthConsentRead**](OAuthConsentRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instancewide_destination_oauth_params

> set_instancewide_destination_oauth_params(set_instancewide_destination_oauth_params_request_body)
Sets instancewide variables to be used for the oauth flow when creating this destination. When set, these variables will be injected into a connector's configuration before any interaction with the connector image itself. This enables running oauth flows with consistent variables e.g: the company's Google Ads developer_token, client_id, and client_secret without the user having to know about these variables. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_instancewide_destination_oauth_params_request_body** | [**SetInstancewideDestinationOauthParamsRequestBody**](SetInstancewideDestinationOauthParamsRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

