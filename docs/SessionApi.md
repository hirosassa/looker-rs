# \SessionApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**session**](SessionApi.md#session) | **GET** /session | Get Session
[**update_session**](SessionApi.md#update_session) | **PATCH** /session | Update Session



## session

> crate::models::ApiSession session()
Get Session

### Get API Session  Returns information about the current API session, such as which workspace is selected for the session. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiSession**](ApiSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_session

> crate::models::ApiSession update_session(body)
Update Session

### Update API Session  #### API Session Workspace  You can use this endpoint to change the active workspace for the current API session.  Only one workspace can be active in a session. The active workspace can be changed any number of times in a session.  The default workspace for API sessions is the \"production\" workspace.  All Looker APIs that use projects or lookml models (such as running queries) will use the version of project and model files defined by this workspace for the lifetime of the current API session or until the session workspace is changed again.  An API session has the same lifetime as the access_token used to authenticate API requests. Each successful API login generates a new access_token and a new API session.  If your Looker API client application needs to work in a dev workspace across multiple API sessions, be sure to select the dev workspace after each login. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiSession**](ApiSession.md) | Session | [required] |

### Return type

[**crate::models::ApiSession**](ApiSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

