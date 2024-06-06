# \CompaniesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_company**](CompaniesApi.md#create_company) | **POST** /api/companies | 
[**delete_company**](CompaniesApi.md#delete_company) | **DELETE** /api/companies/{company_id} | 
[**get_companies_for_org**](CompaniesApi.md#get_companies_for_org) | **GET** /api/companies | 
[**get_company_by_id**](CompaniesApi.md#get_company_by_id) | **GET** /api/companies/{company_id} | 
[**update_company**](CompaniesApi.md#update_company) | **PUT** /api/companies/{company_id} | 



## create_company

> models::Company create_company(organization, create_company_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**create_company_req_payload** | [**CreateCompanyReqPayload**](CreateCompanyReqPayload.md) | JSON request payload to create a new company | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_company

> delete_company(organization, company_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**company_id** | **String** | The id of the company you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_companies_for_org

> Vec<models::Company> get_companies_for_org(organization, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**limit** | Option<**i64**> | Limit the number of results. Default is 10 |  |
**offset** | Option<**i64**> | Offset the results. Default is 0 |  |

### Return type

[**Vec<models::Company>**](Company.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_by_id

> models::Company get_company_by_id(organization, company_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**company_id** | **String** | The id of the company you want to fetch. | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_company

> models::Company update_company(organization, company_id, update_company_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**company_id** | **String** | The id of the note you want to update. | [required] |
**update_company_req_payload** | [**UpdateCompanyReqPayload**](UpdateCompanyReqPayload.md) | JSON request payload to rename the company | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

