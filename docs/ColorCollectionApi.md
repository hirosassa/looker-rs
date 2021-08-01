# \ColorCollectionApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_color_collections**](ColorCollectionApi.md#all_color_collections) | **GET** /color_collections | Get all Color Collections
[**color_collection**](ColorCollectionApi.md#color_collection) | **GET** /color_collections/{collection_id} | Get Color Collection by ID
[**color_collections_custom**](ColorCollectionApi.md#color_collections_custom) | **GET** /color_collections/custom | Get all Custom Color Collections
[**color_collections_standard**](ColorCollectionApi.md#color_collections_standard) | **GET** /color_collections/standard | Get all Standard Color Collections
[**create_color_collection**](ColorCollectionApi.md#create_color_collection) | **POST** /color_collections | Create ColorCollection
[**default_color_collection**](ColorCollectionApi.md#default_color_collection) | **GET** /color_collections/default | Get Default Color Collection
[**delete_color_collection**](ColorCollectionApi.md#delete_color_collection) | **DELETE** /color_collections/{collection_id} | Delete ColorCollection
[**set_default_color_collection**](ColorCollectionApi.md#set_default_color_collection) | **PUT** /color_collections/default | Set Default Color Collection
[**update_color_collection**](ColorCollectionApi.md#update_color_collection) | **PATCH** /color_collections/{collection_id} | Update Custom Color collection



## all_color_collections

> Vec<crate::models::ColorCollection> all_color_collections(fields)
Get all Color Collections

### Get an array of all existing Color Collections Get a **single** color collection by id with [ColorCollection](#!/ColorCollection/color_collection)  Get all **standard** color collections with [ColorCollection](#!/ColorCollection/color_collections_standard)  Get all **custom** color collections with [ColorCollection](#!/ColorCollection/color_collections_custom)  **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ColorCollection>**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## color_collection

> crate::models::ColorCollection color_collection(collection_id, fields)
Get Color Collection by ID

### Get a Color Collection by ID  Use this to retrieve a specific Color Collection. Get a **single** color collection by id with [ColorCollection](#!/ColorCollection/color_collection)  Get all **standard** color collections with [ColorCollection](#!/ColorCollection/color_collections_standard)  Get all **custom** color collections with [ColorCollection](#!/ColorCollection/color_collections_custom)  **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Id of Color Collection | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ColorCollection**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## color_collections_custom

> Vec<crate::models::ColorCollection> color_collections_custom(fields)
Get all Custom Color Collections

### Get an array of all existing **Custom** Color Collections Get a **single** color collection by id with [ColorCollection](#!/ColorCollection/color_collection)  Get all **standard** color collections with [ColorCollection](#!/ColorCollection/color_collections_standard)  **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ColorCollection>**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## color_collections_standard

> Vec<crate::models::ColorCollection> color_collections_standard(fields)
Get all Standard Color Collections

### Get an array of all existing **Standard** Color Collections Get a **single** color collection by id with [ColorCollection](#!/ColorCollection/color_collection)  Get all **custom** color collections with [ColorCollection](#!/ColorCollection/color_collections_custom)  **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ColorCollection>**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_color_collection

> crate::models::ColorCollection create_color_collection(body)
Create ColorCollection

### Create a custom color collection with the specified information  Creates a new custom color collection object, returning the details, including the created id.  **Update** an existing color collection with [Update Color Collection](#!/ColorCollection/update_color_collection)  **Permanently delete** an existing custom color collection with [Delete Color Collection](#!/ColorCollection/delete_color_collection)  **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ColorCollection**](ColorCollection.md) | ColorCollection | [required] |

### Return type

[**crate::models::ColorCollection**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## default_color_collection

> crate::models::ColorCollection default_color_collection()
Get Default Color Collection

### Get the default color collection  Use this to retrieve the default Color Collection.  Set the default color collection with [ColorCollection](#!/ColorCollection/set_default_color_collection) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ColorCollection**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_color_collection

> delete_color_collection(collection_id)
Delete ColorCollection

### Delete a custom color collection by id  This operation permanently deletes the identified **Custom** color collection.  **Standard** color collections cannot be deleted  Because multiple color collections can have the same label, they must be deleted by ID, not name. **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Id of Color Collection | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_color_collection

> crate::models::ColorCollection set_default_color_collection(collection_id)
Set Default Color Collection

### Set the global default Color Collection by ID  Returns the new specified default Color Collection object. **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | ID of color collection to set as default | [required] |

### Return type

[**crate::models::ColorCollection**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_color_collection

> crate::models::ColorCollection update_color_collection(collection_id, body)
Update Custom Color collection

### Update a custom color collection by id. **Note**: Only an API user with the Admin role can call this endpoint. Unauthorized requests will return `Not Found` (404) errors.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Id of Custom Color Collection | [required] |
**body** | [**ColorCollection**](ColorCollection.md) | ColorCollection | [required] |

### Return type

[**crate::models::ColorCollection**](ColorCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

