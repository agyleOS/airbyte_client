# DestinationOauthConsentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_definition_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**workspace_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**redirect_url** | **String** | The url to redirect to after getting the user consent | 
**o_auth_input_configuration** | Option<[**serde_json::Value**](.md)> | The values required to configure OAuth flows. The schema for this must match the `OAuthConfigSpecification.oauthUserInputFromConnectorConfigSpecification` schema. | [optional]
**destination_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


