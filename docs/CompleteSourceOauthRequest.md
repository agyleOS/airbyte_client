# CompleteSourceOauthRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**redirect_url** | Option<**String**> | When completing OAuth flow to gain an access token, some API sometimes requires to verify that the app re-send the redirectUrl that was used when consent was given. | [optional]
**query_params** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The query parameters present in the redirect URL after a user granted consent e.g auth code | [optional]
**o_auth_input_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure OAuth flows. The schema for this must match the `OAuthConfigSpecification.oauthUserInputFromConnectorConfigSpecification` schema. | [optional]
**return_secret_coordinate** | Option<**bool**> | If set to true, returns a secret coordinate which references the stored tokens. By default, returns raw tokens. | [optional][default to false]
**source_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


