# \HomepageApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_homepage_items**](HomepageApi.md#all_homepage_items) | **GET** /homepage_items | Get All Homepage Items
[**all_homepage_sections**](HomepageApi.md#all_homepage_sections) | **GET** /homepage_sections | Get All Homepage sections
[**all_homepages**](HomepageApi.md#all_homepages) | **GET** /homepages | Get All Homepages
[**all_primary_homepage_sections**](HomepageApi.md#all_primary_homepage_sections) | **GET** /primary_homepage_sections | Get All Primary homepage sections
[**create_homepage**](HomepageApi.md#create_homepage) | **POST** /homepages | Create Homepage
[**create_homepage_item**](HomepageApi.md#create_homepage_item) | **POST** /homepage_items | Create Homepage Item
[**create_homepage_section**](HomepageApi.md#create_homepage_section) | **POST** /homepage_sections | Create Homepage section
[**delete_homepage**](HomepageApi.md#delete_homepage) | **DELETE** /homepages/{homepage_id} | Delete Homepage
[**delete_homepage_item**](HomepageApi.md#delete_homepage_item) | **DELETE** /homepage_items/{homepage_item_id} | Delete Homepage Item
[**delete_homepage_section**](HomepageApi.md#delete_homepage_section) | **DELETE** /homepage_sections/{homepage_section_id} | Delete Homepage section
[**homepage**](HomepageApi.md#homepage) | **GET** /homepages/{homepage_id} | Get Homepage
[**homepage_item**](HomepageApi.md#homepage_item) | **GET** /homepage_items/{homepage_item_id} | Get Homepage Item
[**homepage_section**](HomepageApi.md#homepage_section) | **GET** /homepage_sections/{homepage_section_id} | Get Homepage section
[**search_homepages**](HomepageApi.md#search_homepages) | **GET** /homepages/search | Search Homepages
[**update_homepage**](HomepageApi.md#update_homepage) | **PATCH** /homepages/{homepage_id} | Update Homepage
[**update_homepage_item**](HomepageApi.md#update_homepage_item) | **PATCH** /homepage_items/{homepage_item_id} | Update Homepage Item
[**update_homepage_section**](HomepageApi.md#update_homepage_section) | **PATCH** /homepage_sections/{homepage_section_id} | Update Homepage section



## all_homepage_items

> Vec<crate::models::HomepageItem> all_homepage_items(fields, sorts, homepage_section_id)
Get All Homepage Items

### Get information about all homepage items. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**homepage_section_id** | Option<**String**> | Filter to a specific homepage section |  |

### Return type

[**Vec<crate::models::HomepageItem>**](HomepageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_homepage_sections

> Vec<crate::models::HomepageSection> all_homepage_sections(fields, sorts)
Get All Homepage sections

### Get information about all homepage sections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |

### Return type

[**Vec<crate::models::HomepageSection>**](HomepageSection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_homepages

> Vec<crate::models::Homepage> all_homepages(fields)
Get All Homepages

### Get information about all homepages. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Homepage>**](Homepage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_primary_homepage_sections

> Vec<crate::models::HomepageSection> all_primary_homepage_sections(fields)
Get All Primary homepage sections

### Get information about the primary homepage's sections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::HomepageSection>**](HomepageSection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_homepage

> crate::models::Homepage create_homepage(body, fields)
Create Homepage

### Create a new homepage. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Homepage**](Homepage.md) | Homepage | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Homepage**](Homepage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_homepage_item

> crate::models::HomepageItem create_homepage_item(body, fields)
Create Homepage Item

### Create a new homepage item. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**HomepageItem**](HomepageItem.md) | Homepage Item | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageItem**](HomepageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_homepage_section

> crate::models::HomepageSection create_homepage_section(body, fields)
Create Homepage section

### Create a new homepage section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**HomepageSection**](HomepageSection.md) | Homepage section | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageSection**](HomepageSection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_homepage

> String delete_homepage(homepage_id)
Delete Homepage

### Delete a homepage. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_id** | **i64** | Id of homepage | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_homepage_item

> String delete_homepage_item(homepage_item_id)
Delete Homepage Item

### Delete a homepage item. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_item_id** | **i64** | Id of homepage_item | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_homepage_section

> String delete_homepage_section(homepage_section_id)
Delete Homepage section

### Delete a homepage section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_section_id** | **i64** | Id of homepage_section | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## homepage

> crate::models::Homepage homepage(homepage_id, fields)
Get Homepage

### Get information about a homepage. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_id** | **i64** | Id of homepage | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Homepage**](Homepage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## homepage_item

> crate::models::HomepageItem homepage_item(homepage_item_id, fields)
Get Homepage Item

### Get information about a homepage item. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_item_id** | **i64** | Id of homepage item | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageItem**](HomepageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## homepage_section

> crate::models::HomepageSection homepage_section(homepage_section_id, fields)
Get Homepage section

### Get information about a homepage section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_section_id** | **i64** | Id of homepage section | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageSection**](HomepageSection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_homepages

> Vec<crate::models::Homepage> search_homepages(title, created_at, first_name, last_name, fields, favorited, creator_id, page, per_page, offset, limit, sorts, filter_or)
Search Homepages

### Search Homepages  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**title** | Option<**String**> | Matches homepage title. |  |
**created_at** | Option<**String**> | Matches the timestamp for when the homepage was created. |  |
**first_name** | Option<**String**> | The first name of the user who created this homepage. |  |
**last_name** | Option<**String**> | The last name of the user who created this homepage. |  |
**fields** | Option<**String**> | Requested fields. |  |
**favorited** | Option<**bool**> | Return favorited homepages when true. |  |
**creator_id** | Option<**String**> | Filter on homepages created by a particular user. |  |
**page** | Option<**i64**> | The page to return. |  |
**per_page** | Option<**i64**> | The number of items in the returned page. |  |
**offset** | Option<**i64**> | The number of items to skip before returning any. (used with limit and takes priority over page and per_page) |  |
**limit** | Option<**i64**> | The maximum number of items to return. (used with offset and takes priority over page and per_page) |  |
**sorts** | Option<**String**> | The fields to sort the results by. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::Homepage>**](Homepage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_homepage

> crate::models::Homepage update_homepage(homepage_id, body, fields)
Update Homepage

### Update a homepage definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_id** | **i64** | Id of homepage | [required] |
**body** | [**Homepage**](Homepage.md) | Homepage | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Homepage**](Homepage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_homepage_item

> crate::models::HomepageItem update_homepage_item(homepage_item_id, body, fields)
Update Homepage Item

### Update a homepage item definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_item_id** | **i64** | Id of homepage item | [required] |
**body** | [**HomepageItem**](HomepageItem.md) | Homepage Item | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageItem**](HomepageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_homepage_section

> crate::models::HomepageSection update_homepage_section(homepage_section_id, body, fields)
Update Homepage section

### Update a homepage section definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homepage_section_id** | **i64** | Id of homepage section | [required] |
**body** | [**HomepageSection**](HomepageSection.md) | Homepage section | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::HomepageSection**](HomepageSection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

