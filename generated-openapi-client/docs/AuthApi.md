# \AuthApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**callback**](AuthApi.md#callback) | **GET** /api/auth/callback | OpenID Connect callback
[**login**](AuthApi.md#login) | **GET** /api/auth | Login
[**logout**](AuthApi.md#logout) | **DELETE** /api/auth | Logout
[**whoami**](AuthApi.md#whoami) | **GET** /api/auth/whoami | Get Currently Auth'ed User



## callback

> callback()
OpenID Connect callback

OpenID Connect callback  This is the callback route for the OAuth provider, it should not be called directly. Redirects to browser with set-cookie header.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> login(redirect_uri, inv_code)
Login

Login  This will redirect you to the OAuth provider for authentication with email/pass, SSO, Google, Github, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redirect_uri** | Option<**String**> | URL to redirect to after successful login |  |
**inv_code** | Option<**uuid::Uuid**> | Code sent via email as a result of successful call to send_invitation |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout()
Logout

Logout  Invalidate your current auth credential stored typically stored in a cookie. This does not invalidate your API key.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## whoami

> models::CreateApiKeyRespPayload whoami()
Get Currently Auth'ed User

Get Currently Auth'ed User  Get the currently auth'ed user. This will return the user object for the currently auth'ed user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateApiKeyRespPayload**](CreateApiKeyRespPayload.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

