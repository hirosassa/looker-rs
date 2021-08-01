# \DataActionApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_remote_data_action_form**](DataActionApi.md#fetch_remote_data_action_form) | **POST** /data_actions/form | Fetch Remote Data Action Form
[**perform_data_action**](DataActionApi.md#perform_data_action) | **POST** /data_actions | Send a Data Action



## fetch_remote_data_action_form

> crate::models::DataActionForm fetch_remote_data_action_form(body)
Fetch Remote Data Action Form

For some data actions, the remote server may supply a form requesting further user input. This endpoint takes a data action, asks the remote server to generate a form for it, and returns that form to you for presentation to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**::std::collections::HashMap<String, String>**](String.md) | Data Action Request | [required] |

### Return type

[**crate::models::DataActionForm**](DataActionForm.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## perform_data_action

> crate::models::DataActionResponse perform_data_action(body)
Send a Data Action

Perform a data action. The data action object can be obtained from query results, and used to perform an arbitrary action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DataActionRequest**](DataActionRequest.md) | Data Action Request | [required] |

### Return type

[**crate::models::DataActionResponse**](DataActionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

