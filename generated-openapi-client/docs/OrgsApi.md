# \OrgsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_org**](OrgsApi.md#create_org) | **POST** /api/orgs | 
[**delete_org**](OrgsApi.md#delete_org) | **DELETE** /api/orgs/{org_id} | 
[**get_orgs_for_authed_user**](OrgsApi.md#get_orgs_for_authed_user) | **GET** /api/orgs | 
[**leave_org**](OrgsApi.md#leave_org) | **DELETE** /api/orgs/leave/{org_id} | 
[**update_org**](OrgsApi.md#update_org) | **PUT** /api/orgs/{org_id} | 



## create_org

> models::Org create_org(create_org_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_org_req_payload** | [**CreateOrgReqPayload**](CreateOrgReqPayload.md) | JSON request payload to create a new organization | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_org

> delete_org(organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgs_for_authed_user

> Vec<models::Org> get_orgs_for_authed_user(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Limit the number of results. Default is 10 |  |
**offset** | Option<**i64**> | Offset the results. Default is 0 |  |

### Return type

[**Vec<models::Org>**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_org

> leave_org(organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org

> models::Org update_org(update_org_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_org_req_payload** | [**UpdateOrgReqPayload**](UpdateOrgReqPayload.md) | JSON request payload to rename the organization | [required] |

### Return type

[**models::Org**](Org.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

