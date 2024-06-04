# \PhonesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_phone**](PhonesApi.md#create_phone) | **POST** /api/phones | 
[**delete_phone**](PhonesApi.md#delete_phone) | **DELETE** /api/phones/{phone_id} | 
[**get_phone**](PhonesApi.md#get_phone) | **GET** /api/phones/{phone_id} | 
[**update_phone**](PhonesApi.md#update_phone) | **PUT** /api/phones/{phone_id} | 



## create_phone

> models::Phone create_phone(organization, create_phone_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**create_phone_req_payload** | [**CreatePhoneReqPayload**](CreatePhoneReqPayload.md) | JSON request payload to create a new phone | [required] |

### Return type

[**models::Phone**](Phone.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_phone

> delete_phone(phone_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | The phone id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phone

> models::Phone get_phone(phone_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | The phone id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

[**models::Phone**](Phone.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_phone

> models::Phone update_phone(phone_id, organization, update_phone_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | The phone id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |
**update_phone_req_payload** | [**UpdatePhoneReqPayload**](UpdatePhoneReqPayload.md) | JSON request payload to update the phone | [required] |

### Return type

[**models::Phone**](Phone.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

