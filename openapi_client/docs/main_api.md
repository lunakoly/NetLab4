# main_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**getMyself**](main_api.md#getMyself) | **GET** /user/me | Returns your status
**getNewUser**](main_api.md#getNewUser) | **GET** /user/new | Get the initial user context
**postQuery**](main_api.md#postQuery) | **POST** /query | Run a command


# **getMyself**
> models::InlineResponse2001 getMyself()
Returns your status

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getNewUser**
> models::InlineResponse200 getNewUser()
Get the initial user context

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postQuery**
> models::KillResult postQuery(ctx, request_body)
Run a command

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request_body** | [**Query**](Query.md)| The command line to run | 

### Return type

[**models::KillResult**](KillResult.md)

### Authorization

[sus](../README.md#sus)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

