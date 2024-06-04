# \DealsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deal**](DealsApi.md#create_deal) | **POST** /api/deals | 
[**delete_deal**](DealsApi.md#delete_deal) | **DELETE** /api/deals/{deal_id} | 
[**get_deal**](DealsApi.md#get_deal) | **GET** /api/deals/{deal_id} | 
[**update_deal**](DealsApi.md#update_deal) | **PUT** /api/deals/{deal_id} | 



## create_deal

> models::Deal create_deal(organization, create_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**create_deal_req_payload** | [**CreateDealReqPayload**](CreateDealReqPayload.md) | JSON request payload to create a new deal | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal

> delete_deal(deal_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **uuid::Uuid** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal

> models::Deal get_deal(deal_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **uuid::Uuid** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal

> models::Deal update_deal(deal_id, organization, update_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **uuid::Uuid** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |
**update_deal_req_payload** | [**UpdateDealReqPayload**](UpdateDealReqPayload.md) | JSON request payload to update the deal | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

