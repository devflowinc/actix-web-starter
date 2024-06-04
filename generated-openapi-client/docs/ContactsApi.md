# \ContactsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_contact**](ContactsApi.md#create_contact) | **POST** /api/contacts | 
[**delete_contact**](ContactsApi.md#delete_contact) | **DELETE** /api/contacts/{contact_id} | 
[**get_contact**](ContactsApi.md#get_contact) | **GET** /api/contacts/{contact_id} | 
[**update_contact**](ContactsApi.md#update_contact) | **PUT** /api/contacts/{contact_id} | 



## create_contact

> models::Org create_contact(create_contact_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_contact_req_payload** | [**CreateContactReqPayload**](CreateContactReqPayload.md) | JSON request payload to create a new contact | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contact

> delete_contact(contact_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contacts id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact

> models::Org get_contact(contact_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contacts id to use for the request | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_contact

> models::Org update_contact(contact_id, update_contact_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contacts id to use for the request | [required] |
**update_contact_req_payload** | [**UpdateContactReqPayload**](UpdateContactReqPayload.md) | JSON request payload to update the contact | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

