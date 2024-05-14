# \ApiKeyApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeyApi.md#create_api_key) | **POST** /api/api_key | Set User Api Key



## create_api_key

> models::CreateApiKeyRespPayload create_api_key(body)
Set User Api Key

Set User Api Key  Create a new api key for the auth'ed user. Successful response will contain the newly created api key. The api key will have permission level of the auth'ed user who calls this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **models::SetUserApiKeyReq** | JSON request payload to create a new user api key | [required] |

### Return type

[**models::CreateApiKeyRespPayload**](CreateApiKeyRespPayload.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

