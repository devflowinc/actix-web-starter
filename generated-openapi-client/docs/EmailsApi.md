# \EmailsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email**](EmailsApi.md#create_email) | **POST** /api/emails | 
[**delete_email**](EmailsApi.md#delete_email) | **DELETE** /api/emails/{email_id} | 
[**get_email**](EmailsApi.md#get_email) | **GET** /api/emails/{email_id} | 
[**update_email**](EmailsApi.md#update_email) | **PUT** /api/emails/{email_id} | 



## create_email

> models::Email create_email(organization, create_email_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**create_email_req_payload** | [**CreateEmailReqPayload**](CreateEmailReqPayload.md) | JSON request payload to create a new email | [required] |

### Return type

[**models::Email**](Email.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email

> delete_email(email_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | **String** | The email id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email

> models::Email get_email(email_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | **String** | The email id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

[**models::Email**](Email.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email

> models::Email update_email(email_id, organization, update_email_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | **String** | The email id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |
**update_email_req_payload** | [**UpdateEmailReqPayload**](UpdateEmailReqPayload.md) | JSON request payload to update the email | [required] |

### Return type

[**models::Email**](Email.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

