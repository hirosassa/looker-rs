# \LookmlModelApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_lookml_models**](LookmlModelApi.md#all_lookml_models) | **GET** /lookml_models | Get All LookML Models
[**create_lookml_model**](LookmlModelApi.md#create_lookml_model) | **POST** /lookml_models | Create LookML Model
[**delete_lookml_model**](LookmlModelApi.md#delete_lookml_model) | **DELETE** /lookml_models/{lookml_model_name} | Delete LookML Model
[**graph_derived_tables_for_model**](LookmlModelApi.md#graph_derived_tables_for_model) | **GET** /derived_table/graph/model/{model} | Get Derived Table
[**lookml_model**](LookmlModelApi.md#lookml_model) | **GET** /lookml_models/{lookml_model_name} | Get LookML Model
[**lookml_model_explore**](LookmlModelApi.md#lookml_model_explore) | **GET** /lookml_models/{lookml_model_name}/explores/{explore_name} | Get LookML Model Explore
[**update_lookml_model**](LookmlModelApi.md#update_lookml_model) | **PATCH** /lookml_models/{lookml_model_name} | Update LookML Model



## all_lookml_models

> Vec<crate::models::LookmlModel> all_lookml_models(fields)
Get All LookML Models

### Get information about all lookml models. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::LookmlModel>**](LookmlModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lookml_model

> crate::models::LookmlModel create_lookml_model(body)
Create LookML Model

### Create a lookml model using the specified configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LookmlModel**](LookmlModel.md) | LookML Model | [required] |

### Return type

[**crate::models::LookmlModel**](LookmlModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lookml_model

> String delete_lookml_model(lookml_model_name)
Delete LookML Model

### Delete a lookml model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_model_name** | **String** | Name of lookml model. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graph_derived_tables_for_model

> crate::models::DependencyGraph graph_derived_tables_for_model(model, format, color)
Get Derived Table

### Discover information about derived tables 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | **String** | The name of the Lookml model. | [required] |
**format** | Option<**String**> | The format of the graph. Valid values are [dot]. Default is `dot` |  |
**color** | Option<**String**> | Color denoting the build status of the graph. Grey = not built, green = built, yellow = building, red = error. |  |

### Return type

[**crate::models::DependencyGraph**](DependencyGraph.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookml_model

> crate::models::LookmlModel lookml_model(lookml_model_name, fields)
Get LookML Model

### Get information about a lookml model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_model_name** | **String** | Name of lookml model. | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::LookmlModel**](LookmlModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookml_model_explore

> crate::models::LookmlModelExplore lookml_model_explore(lookml_model_name, explore_name, fields)
Get LookML Model Explore

### Get information about a lookml model explore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_model_name** | **String** | Name of lookml model. | [required] |
**explore_name** | **String** | Name of explore. | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::LookmlModelExplore**](LookmlModelExplore.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lookml_model

> crate::models::LookmlModel update_lookml_model(lookml_model_name, body)
Update LookML Model

### Update a lookml model using the specified configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_model_name** | **String** | Name of lookml model. | [required] |
**body** | [**LookmlModel**](LookmlModel.md) | LookML Model | [required] |

### Return type

[**crate::models::LookmlModel**](LookmlModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

