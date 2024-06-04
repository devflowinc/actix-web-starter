# \LinksApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_link**](LinksApi.md#create_link) | **POST** /api/links | 
[**delete_link**](LinksApi.md#delete_link) | **DELETE** /api/links/{link_id} | 
[**get_link**](LinksApi.md#get_link) | **GET** /api/links/{link_id} | 
[**update_link**](LinksApi.md#update_link) | **PUT** /api/links/{link_id} | 



## create_link

> models::Org create_link(create_link_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_link_req_payload** | [**CreateLinkReqPayload**](CreateLinkReqPayload.md) | JSON request payload to create a new link | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_link

> delete_link(link)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link** | **String** | The link id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_link

> models::Org get_link()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_link

> models::Org update_link(update_link_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_link_req_payload** | [**UpdateLinkReqPayload**](UpdateLinkReqPayload.md) | JSON request payload to update the link | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

