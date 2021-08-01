# \SpaceApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_spaces**](SpaceApi.md#all_spaces) | **GET** /spaces | Get All Spaces
[**create_space**](SpaceApi.md#create_space) | **POST** /spaces | Create Space
[**delete_space**](SpaceApi.md#delete_space) | **DELETE** /spaces/{space_id} | Delete Space
[**search_spaces**](SpaceApi.md#search_spaces) | **GET** /spaces/search | Search Spaces
[**space**](SpaceApi.md#space) | **GET** /spaces/{space_id} | Get Space
[**space_ancestors**](SpaceApi.md#space_ancestors) | **GET** /spaces/{space_id}/ancestors | Get Space Ancestors
[**space_children**](SpaceApi.md#space_children) | **GET** /spaces/{space_id}/children | Get Space Children
[**space_children_search**](SpaceApi.md#space_children_search) | **GET** /spaces/{space_id}/children/search | Search Space Children
[**space_dashboards**](SpaceApi.md#space_dashboards) | **GET** /spaces/{space_id}/dashboards | Get Space Dashboards
[**space_looks**](SpaceApi.md#space_looks) | **GET** /spaces/{space_id}/looks | Get Space Looks
[**space_parent**](SpaceApi.md#space_parent) | **GET** /spaces/{space_id}/parent | Get Space Parent
[**update_space**](SpaceApi.md#update_space) | **PATCH** /spaces/{space_id} | Update Space



## all_spaces

> Vec<crate::models::SpaceBase> all_spaces(fields)
Get All Spaces

### Get information about all spaces.  In API 3.x, this will not return empty personal spaces, unless they belong to the calling user. In API 4.0+, all personal spaces will be returned.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::SpaceBase>**](SpaceBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_space

> crate::models::Space create_space(body)
Create Space

### Create a space with specified information.  Caller must have permission to edit the parent space and to create spaces, otherwise the request returns 404 Not Found. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateSpace**](CreateSpace.md) | Create a new space | [required] |

### Return type

[**crate::models::Space**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_space

> String delete_space(space_id)
Delete Space

### Delete the space with a specific id including any children spaces. **DANGER** this will delete all looks and dashboards in the space. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_spaces

> Vec<crate::models::Space> search_spaces(fields, page, per_page, limit, offset, sorts, name, id, parent_id, creator_id, filter_or, is_shared_root)
Search Spaces

### Search Spaces    Returns an **array of space objects** that match the given search criteria.    If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.     The parameters `limit`, and `offset` are recommended for fetching results in page-size chunks.    Get a **single space** by id with [Space](#!/Space/space) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**limit** | Option<**i64**> | Number of results to return. (used with offset and takes priority over page and per_page) |  |
**offset** | Option<**i64**> | Number of results to skip before returning any. (used with limit and takes priority over page and per_page) |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**name** | Option<**String**> | Match Space title. |  |
**id** | Option<**i64**> | Match Space id |  |
**parent_id** | Option<**String**> | Filter on a children of a particular space. |  |
**creator_id** | Option<**String**> | Filter on spaces created by a particular user. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |
**is_shared_root** | Option<**bool**> | Match is shared root |  |

### Return type

[**Vec<crate::models::Space>**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space

> crate::models::Space space(space_id, fields)
Get Space

### Get information about the space with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Space**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_ancestors

> Vec<crate::models::Space> space_ancestors(space_id, fields)
Get Space Ancestors

### Get the ancestors of a space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Space>**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_children

> Vec<crate::models::Space> space_children(space_id, fields, page, per_page, sorts)
Get Space Children

### Get the children of a space.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |

### Return type

[**Vec<crate::models::Space>**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_children_search

> Vec<crate::models::Space> space_children_search(space_id, fields, sorts, name)
Search Space Children

### Search the children of a space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**name** | Option<**String**> | Match Space name. |  |

### Return type

[**Vec<crate::models::Space>**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_dashboards

> Vec<crate::models::Dashboard> space_dashboards(space_id, fields)
Get Space Dashboards

### Get the dashboards in a space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Dashboard>**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_looks

> Vec<crate::models::LookWithQuery> space_looks(space_id, fields)
Get Space Looks

### Get all looks in a space. In API 3.x, this will return all looks in a space, including looks in the trash. In API 4.0+, all looks in a space will be returned, excluding looks in the trash. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::LookWithQuery>**](LookWithQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## space_parent

> crate::models::Space space_parent(space_id, fields)
Get Space Parent

### Get the parent of a space

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Space**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_space

> crate::models::Space update_space(space_id, body)
Update Space

### Update the space with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **String** | Id of space | [required] |
**body** | [**UpdateSpace**](UpdateSpace.md) | Space parameters | [required] |

### Return type

[**crate::models::Space**](Space.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

