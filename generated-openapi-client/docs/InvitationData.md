# InvitationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_url** | **String** | The url of the app that the user will be directed to in order to set their password. Usually admin.trieve.ai, but may differ for local dev or self-hosted setups. | 
**email** | **String** | The email of the user to invite. Must be a valid email as they will be sent an email to register. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the organization to invite the user to. | 
**redirect_uri** | **String** | The url that the user will be redirected to after setting their password. | 
**user_role** | **i32** | The role the user will have in the organization. 0 = User, 1 = Admin, 2 = Owner. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


