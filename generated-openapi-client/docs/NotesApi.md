# \NotesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_note**](NotesApi.md#create_note) | **POST** /api/notes | 
[**delete_note**](NotesApi.md#delete_note) | **DELETE** /api/notes/{note_id} | 
[**get_note_by_id**](NotesApi.md#get_note_by_id) | **GET** /api/notes/{note_id} | 
[**get_notes_for_org**](NotesApi.md#get_notes_for_org) | **GET** /api/notes | 
[**update_note**](NotesApi.md#update_note) | **PUT** /api/notes/{note_id} | 



## create_note

> models::Note create_note(organization, create_note_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**create_note_req_payload** | [**CreateNoteReqPayload**](CreateNoteReqPayload.md) | JSON request payload to create a new note | [required] |

### Return type

[**models::Note**](Note.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_note

> delete_note(organization, note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**note_id** | **String** | The id of the note you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_note_by_id

> models::Note get_note_by_id(organization, note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**note_id** | **String** | The id of the organization you want to fetch. | [required] |

### Return type

[**models::Note**](Note.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notes_for_org

> Vec<models::Org> get_notes_for_org(limit, offset)


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


## update_note

> models::Note update_note(organization, note_id, update_note_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**note_id** | **String** | The id of the note you want to update. | [required] |
**update_note_req_payload** | [**UpdateNoteReqPayload**](UpdateNoteReqPayload.md) | JSON request payload to rename the organization | [required] |

### Return type

[**models::Note**](Note.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

