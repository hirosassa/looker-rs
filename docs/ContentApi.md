# \ContentApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_content_metadata_accesses**](ContentApi.md#all_content_metadata_accesses) | **GET** /content_metadata_access | Get All Content Metadata Accesses
[**all_content_metadatas**](ContentApi.md#all_content_metadatas) | **GET** /content_metadata | Get All Content Metadatas
[**content_favorite**](ContentApi.md#content_favorite) | **GET** /content_favorite/{content_favorite_id} | Get Favorite Content
[**content_metadata**](ContentApi.md#content_metadata) | **GET** /content_metadata/{content_metadata_id} | Get Content Metadata
[**content_thumbnail**](ContentApi.md#content_thumbnail) | **GET** /content_thumbnail/{type}/{resource_id} | Get Content Thumbnail
[**content_validation**](ContentApi.md#content_validation) | **GET** /content_validation | Validate Content
[**create_content_favorite**](ContentApi.md#create_content_favorite) | **POST** /content_favorite | Create Favorite Content
[**create_content_metadata_access**](ContentApi.md#create_content_metadata_access) | **POST** /content_metadata_access | Create Content Metadata Access
[**delete_content_favorite**](ContentApi.md#delete_content_favorite) | **DELETE** /content_favorite/{content_favorite_id} | Delete Favorite Content
[**delete_content_metadata_access**](ContentApi.md#delete_content_metadata_access) | **DELETE** /content_metadata_access/{content_metadata_access_id} | Delete Content Metadata Access
[**search_content_favorites**](ContentApi.md#search_content_favorites) | **GET** /content_favorite/search | Search Favorite Contents
[**search_content_views**](ContentApi.md#search_content_views) | **GET** /content_view/search | Search Content Views
[**update_content_metadata**](ContentApi.md#update_content_metadata) | **PATCH** /content_metadata/{content_metadata_id} | Update Content Metadata
[**update_content_metadata_access**](ContentApi.md#update_content_metadata_access) | **PUT** /content_metadata_access/{content_metadata_access_id} | Update Content Metadata Access
[**vector_thumbnail**](ContentApi.md#vector_thumbnail) | **GET** /vector_thumbnail/{type}/{resource_id} | Get Vector Thumbnail



## all_content_metadata_accesses

> Vec<crate::models::ContentMetaGroupUser> all_content_metadata_accesses(content_metadata_id, fields)
Get All Content Metadata Accesses

### All content metadata access records for a content metadata item. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_metadata_id** | **i64** | Id of content metadata | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ContentMetaGroupUser>**](ContentMetaGroupUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_content_metadatas

> Vec<crate::models::ContentMeta> all_content_metadatas(parent_id, fields)
Get All Content Metadatas

### Get information about all content metadata in a space. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_id** | **i64** | Parent space of content. | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ContentMeta>**](ContentMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_favorite

> crate::models::ContentFavorite content_favorite(content_favorite_id, fields)
Get Favorite Content

### Get favorite content by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_favorite_id** | **i64** | Id of favorite content | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ContentFavorite**](ContentFavorite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_metadata

> crate::models::ContentMeta content_metadata(content_metadata_id, fields)
Get Content Metadata

### Get information about an individual content metadata record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_metadata_id** | **i64** | Id of content metadata | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ContentMeta**](ContentMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_thumbnail

> String content_thumbnail(_type, resource_id, reload, format, width, height)
Get Content Thumbnail

### Get an image representing the contents of a dashboard or look.  The returned thumbnail is an abstract representation of the contents of a dashbord or look and does not reflect the actual data displayed in the respective visualizations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Either dashboard or look | [required] |
**resource_id** | **String** | ID of the dashboard or look to render | [required] |
**reload** | Option<**String**> | Whether or not to refresh the rendered image with the latest content |  |
**format** | Option<**String**> | A value of png produces a thumbnail in PNG format instead of SVG (default) |  |
**width** | Option<**i64**> | The width of the image if format is supplied |  |
**height** | Option<**i64**> | The height of the image if format is supplied |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_validation

> crate::models::ContentValidation content_validation(fields)
Validate Content

### Validate All Content  Performs validation of all looks and dashboards Returns a list of errors found as well as metadata about the content validation run. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ContentValidation**](ContentValidation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_content_favorite

> crate::models::ContentFavorite create_content_favorite(body)
Create Favorite Content

### Create favorite content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContentFavorite**](ContentFavorite.md) | Favorite Content | [required] |

### Return type

[**crate::models::ContentFavorite**](ContentFavorite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_content_metadata_access

> crate::models::ContentMetaGroupUser create_content_metadata_access(body, send_boards_notification_email)
Create Content Metadata Access

### Create content metadata access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContentMetaGroupUser**](ContentMetaGroupUser.md) | Content Metadata Access | [required] |
**send_boards_notification_email** | Option<**bool**> | Optionally sends notification email when granting access to a board. |  |

### Return type

[**crate::models::ContentMetaGroupUser**](ContentMetaGroupUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_content_favorite

> String delete_content_favorite(content_favorite_id)
Delete Favorite Content

### Delete favorite content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_favorite_id** | **i64** | Id of favorite content | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_content_metadata_access

> String delete_content_metadata_access(content_metadata_access_id)
Delete Content Metadata Access

### Remove content metadata access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_metadata_access_id** | **i64** | Id of content metadata access | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_content_favorites

> Vec<crate::models::ContentFavorite> search_content_favorites(id, user_id, content_metadata_id, dashboard_id, look_id, limit, offset, sorts, fields, filter_or)
Search Favorite Contents

### Search Favorite Content  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Match content favorite id(s) |  |
**user_id** | Option<**i64**> | Match user id(s) |  |
**content_metadata_id** | Option<**i64**> | Match content metadata id(s) |  |
**dashboard_id** | Option<**i64**> | Match dashboard id(s) |  |
**look_id** | Option<**i64**> | Match look id(s) |  |
**limit** | Option<**i64**> | Number of results to return. (used with offset) |  |
**offset** | Option<**i64**> | Number of results to skip before returning any. (used with limit) |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**fields** | Option<**String**> | Requested fields. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::ContentFavorite>**](ContentFavorite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_content_views

> Vec<crate::models::ContentView> search_content_views(view_count, group_id, look_id, dashboard_id, content_metadata_id, start_of_week_date, all_time, user_id, fields, limit, offset, sorts, filter_or)
Search Content Views

### Search Content Views  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_count** | Option<**i64**> | Match view count |  |
**group_id** | Option<**i64**> | Match Group Id |  |
**look_id** | Option<**String**> | Match look_id |  |
**dashboard_id** | Option<**String**> | Match dashboard_id |  |
**content_metadata_id** | Option<**i64**> | Match content metadata id |  |
**start_of_week_date** | Option<**String**> | Match start of week date (format is \"YYYY-MM-DD\") |  |
**all_time** | Option<**bool**> | True if only all time view records should be returned |  |
**user_id** | Option<**i64**> | Match user id |  |
**fields** | Option<**String**> | Requested fields |  |
**limit** | Option<**i64**> | Number of results to return. Use with `offset` to manage pagination of results |  |
**offset** | Option<**i64**> | Number of results to skip before returning data |  |
**sorts** | Option<**String**> | Fields to sort by |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::ContentView>**](ContentView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_content_metadata

> crate::models::ContentMeta update_content_metadata(content_metadata_id, body)
Update Content Metadata

### Move a piece of content. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_metadata_id** | **i64** | Id of content metadata | [required] |
**body** | [**ContentMeta**](ContentMeta.md) | Content Metadata | [required] |

### Return type

[**crate::models::ContentMeta**](ContentMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_content_metadata_access

> crate::models::ContentMetaGroupUser update_content_metadata_access(content_metadata_access_id, body)
Update Content Metadata Access

### Update type of access for content metadata. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_metadata_access_id** | **i64** | Id of content metadata access | [required] |
**body** | [**ContentMetaGroupUser**](ContentMetaGroupUser.md) | Content Metadata Access | [required] |

### Return type

[**crate::models::ContentMetaGroupUser**](ContentMetaGroupUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vector_thumbnail

> String vector_thumbnail(_type, resource_id, reload)
Get Vector Thumbnail

### Get a vector image representing the contents of a dashboard or look.  # DEPRECATED:  Use [content_thumbnail()](#!/Content/content_thumbnail)  The returned thumbnail is an abstract representation of the contents of a dashbord or look and does not reflect the actual data displayed in the respective visualizations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Either dashboard or look | [required] |
**resource_id** | **String** | ID of the dashboard or look to render | [required] |
**reload** | Option<**String**> | Whether or not to refresh the rendered image with the latest content |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

