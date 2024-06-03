# \NotesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_note**](NotesApi.md#create_note) | **POST** /api/notes | 



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

