# \IntegrationApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_integration_hub_legal_agreement**](IntegrationApi.md#accept_integration_hub_legal_agreement) | **POST** /integration_hubs/{integration_hub_id}/accept_legal_agreement | Accept Integration Hub Legal Agreement
[**all_integration_hubs**](IntegrationApi.md#all_integration_hubs) | **GET** /integration_hubs | Get All Integration Hubs
[**all_integrations**](IntegrationApi.md#all_integrations) | **GET** /integrations | Get All Integrations
[**create_integration_hub**](IntegrationApi.md#create_integration_hub) | **POST** /integration_hubs | Create Integration Hub
[**delete_integration_hub**](IntegrationApi.md#delete_integration_hub) | **DELETE** /integration_hubs/{integration_hub_id} | Delete Integration Hub
[**fetch_integration_form**](IntegrationApi.md#fetch_integration_form) | **POST** /integrations/{integration_id}/form | Fetch Remote Integration Form
[**integration**](IntegrationApi.md#integration) | **GET** /integrations/{integration_id} | Get Integration
[**integration_hub**](IntegrationApi.md#integration_hub) | **GET** /integration_hubs/{integration_hub_id} | Get Integration Hub
[**test_integration**](IntegrationApi.md#test_integration) | **POST** /integrations/{integration_id}/test | Test integration
[**update_integration**](IntegrationApi.md#update_integration) | **PATCH** /integrations/{integration_id} | Update Integration
[**update_integration_hub**](IntegrationApi.md#update_integration_hub) | **PATCH** /integration_hubs/{integration_hub_id} | Update Integration Hub



## accept_integration_hub_legal_agreement

> crate::models::IntegrationHub accept_integration_hub_legal_agreement(integration_hub_id)
Accept Integration Hub Legal Agreement

Accepts the legal agreement for a given integration hub. This only works for integration hubs that have legal_agreement_required set to true and legal_agreement_signed set to false.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_hub_id** | **i64** | Id of integration_hub | [required] |

### Return type

[**crate::models::IntegrationHub**](IntegrationHub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_integration_hubs

> Vec<crate::models::IntegrationHub> all_integration_hubs(fields)
Get All Integration Hubs

### Get information about all Integration Hubs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::IntegrationHub>**](IntegrationHub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_integrations

> Vec<crate::models::Integration> all_integrations(fields, integration_hub_id)
Get All Integrations

### Get information about all Integrations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**integration_hub_id** | Option<**String**> | Filter to a specific provider |  |

### Return type

[**Vec<crate::models::Integration>**](Integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_integration_hub

> crate::models::IntegrationHub create_integration_hub(body, fields)
Create Integration Hub

### Create a new Integration Hub.  This API is rate limited to prevent it from being used for SSRF attacks 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IntegrationHub**](IntegrationHub.md) | Integration Hub | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::IntegrationHub**](IntegrationHub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integration_hub

> String delete_integration_hub(integration_hub_id)
Delete Integration Hub

### Delete a Integration Hub. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_hub_id** | **i64** | Id of integration_hub | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_integration_form

> crate::models::DataActionForm fetch_integration_form(integration_id, body)
Fetch Remote Integration Form

Returns the Integration form for presentation to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Id of integration | [required] |
**body** | Option<[**::std::collections::HashMap<String, String>**](String.md)> | Integration Form Request |  |

### Return type

[**crate::models::DataActionForm**](DataActionForm.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integration

> crate::models::Integration integration(integration_id, fields)
Get Integration

### Get information about a Integration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Id of integration | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integration_hub

> crate::models::IntegrationHub integration_hub(integration_hub_id, fields)
Get Integration Hub

### Get information about a Integration Hub. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_hub_id** | **i64** | Id of Integration Hub | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::IntegrationHub**](IntegrationHub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_integration

> crate::models::IntegrationTestResult test_integration(integration_id)
Test integration

Tests the integration to make sure all the settings are working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Id of integration | [required] |

### Return type

[**crate::models::IntegrationTestResult**](IntegrationTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_integration

> crate::models::Integration update_integration(integration_id, body, fields)
Update Integration

### Update parameters on a Integration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Id of integration | [required] |
**body** | [**Integration**](Integration.md) | Integration | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_integration_hub

> crate::models::IntegrationHub update_integration_hub(integration_hub_id, body, fields)
Update Integration Hub

### Update a Integration Hub definition.  This API is rate limited to prevent it from being used for SSRF attacks 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_hub_id** | **i64** | Id of Integration Hub | [required] |
**body** | [**IntegrationHub**](IntegrationHub.md) | Integration Hub | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::IntegrationHub**](IntegrationHub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

