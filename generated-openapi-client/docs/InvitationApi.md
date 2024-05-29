# \InvitationApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_invitation**](InvitationApi.md#delete_invitation) | **DELETE** /api/invitation/{invitation_id} | Delete Invitation
[**get_invitations**](InvitationApi.md#get_invitations) | **GET** /api/invitation/{organization_id} | Get Invitations
[**post_invitation**](InvitationApi.md#post_invitation) | **POST** /api/invitation | Send Invitation



## delete_invitation

> delete_invitation(invitation_id)
Delete Invitation

Delete Invitation  Delete an invitation by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invitations

> Vec<models::Invitation> get_invitations(organization_id)
Get Invitations

Get Invitations  Get all invitations for the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::Invitation>**](Invitation.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_invitation

> post_invitation(organization, invitation_data)
Send Invitation

Send Invitation  Invitations act as a way to invite users to join an organization. After a user is invited, they will automatically be added to the organization with the role specified in the invitation once they set their.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**invitation_data** | [**InvitationData**](InvitationData.md) | JSON request payload to send an invitation | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

