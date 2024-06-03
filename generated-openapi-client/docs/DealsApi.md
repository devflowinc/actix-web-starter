# \DealsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deal**](DealsApi.md#create_deal) | **POST** /api/deals | 
[**delete_deal**](DealsApi.md#delete_deal) | **DELETE** /api/deals/{deal_id} | 
[**get_deal**](DealsApi.md#get_deal) | **GET** /api/deals/{deal_id} | 
[**update_deal**](DealsApi.md#update_deal) | **PUT** /api/deals/{deal_id} | 



## create_deal

> models::Org create_deal(create_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_deal_req_payload** | [**CreateDealReqPayload**](CreateDealReqPayload.md) | JSON request payload to create a new deal | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal

> delete_deal(deal, deal_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal** | **String** | The deal id to use for the request | [required] |
**deal_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal

> models::Org get_deal(deal_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal

> models::Org update_deal(deal_id, update_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **uuid::Uuid** |  | [required] |
**update_deal_req_payload** | [**UpdateDealReqPayload**](UpdateDealReqPayload.md) | JSON request payload to update the deal | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

