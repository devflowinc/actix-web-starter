# \TasksApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **POST** /api/tasks | 
[**create_task_resource**](TasksApi.md#create_task_resource) | **POST** /api/tasks/{task_id}/{resource_type}/{resource_id} | 
[**delete_task**](TasksApi.md#delete_task) | **DELETE** /api/tasks/{task_id} | 
[**delete_task_resource**](TasksApi.md#delete_task_resource) | **DELETE** /api/tasks/{task_id}/{resource_type}/{resource_id} | 
[**get_task**](TasksApi.md#get_task) | **GET** /api/tasks/{task_id} | 
[**list_task_resource**](TasksApi.md#list_task_resource) | **GET** /api/tasks/{task_id}/{resource_type} | 
[**update_task**](TasksApi.md#update_task) | **PUT** /api/tasks/{task_id} | 



## create_task

> models::Task create_task(organization, create_task_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**create_task_req_payload** | [**CreateTaskReqPayload**](CreateTaskReqPayload.md) | JSON request payload to create a new task | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_task_resource

> models::TaskResource create_task_resource(task_id, resource_type, resource_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**resource_type** | [**TaskResType**](.md) | The resource type to use for the request | [required] |
**resource_id** | **String** | The resource id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

[**models::TaskResource**](TaskResource.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_task

> delete_task(task_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_task_resource

> delete_task_resource(task_id, resource_type, resource_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**resource_type** | [**TaskResType**](.md) | The resource type to use for the request | [required] |
**resource_id** | **String** | The resource id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::Task get_task(task_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_task_resource

> models::TaskResourceListWithPagination list_task_resource(task_id, resource_type, organization, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**resource_type** | [**TaskResType**](.md) | The resource type to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |
**limit** | **i64** | The number of records to return | [required] |
**offset** | **i64** | The number of records to skip | [required] |

### Return type

[**models::TaskResourceListWithPagination**](TaskResourceListWithPagination.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> models::Task update_task(task_id, organization, update_task_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |
**update_task_req_payload** | [**UpdateTaskReqPayload**](UpdateTaskReqPayload.md) | JSON request payload to update the task | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

