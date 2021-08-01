# \DatagroupApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_datagroups**](DatagroupApi.md#all_datagroups) | **GET** /datagroups | Get All Datagroups
[**datagroup**](DatagroupApi.md#datagroup) | **GET** /datagroups/{datagroup_id} | Get Datagroup
[**update_datagroup**](DatagroupApi.md#update_datagroup) | **PATCH** /datagroups/{datagroup_id} | Update Datagroup



## all_datagroups

> Vec<crate::models::Datagroup> all_datagroups()
Get All Datagroups

### Get information about all datagroups. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Datagroup>**](Datagroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datagroup

> crate::models::Datagroup datagroup(datagroup_id)
Get Datagroup

### Get information about a datagroup. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datagroup_id** | **String** | ID of datagroup. | [required] |

### Return type

[**crate::models::Datagroup**](Datagroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_datagroup

> crate::models::Datagroup update_datagroup(datagroup_id, body)
Update Datagroup

### Update a datagroup using the specified params. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datagroup_id** | **String** | ID of datagroup. | [required] |
**body** | [**Datagroup**](Datagroup.md) | Datagroup | [required] |

### Return type

[**crate::models::Datagroup**](Datagroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

