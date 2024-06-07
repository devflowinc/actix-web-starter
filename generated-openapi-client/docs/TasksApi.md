# \TasksApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **POST** /api/tasks | 
[**delete_task**](TasksApi.md#delete_task) | **DELETE** /api/tasks/{task_id} | 
[**get_task**](TasksApi.md#get_task) | **GET** /api/tasks/{task_id} | 
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

