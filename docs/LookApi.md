# \LookApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_looks**](LookApi.md#all_looks) | **GET** /looks | Get All Looks
[**create_look**](LookApi.md#create_look) | **POST** /looks | Create Look
[**delete_look**](LookApi.md#delete_look) | **DELETE** /looks/{look_id} | Delete Look
[**look**](LookApi.md#look) | **GET** /looks/{look_id} | Get Look
[**run_look**](LookApi.md#run_look) | **GET** /looks/{look_id}/run/{result_format} | Run Look
[**search_looks**](LookApi.md#search_looks) | **GET** /looks/search | Search Looks
[**update_look**](LookApi.md#update_look) | **PATCH** /looks/{look_id} | Update Look



## all_looks

> Vec<crate::models::Look> all_looks(fields)
Get All Looks

### Get information about all active Looks  Returns an array of **abbreviated Look objects** describing all the looks that the caller has access to. Soft-deleted Looks are **not** included.  Get the **full details** of a specific look by id with [look(id)](#!/Look/look)  Find **soft-deleted looks** with [search_looks()](#!/Look/search_looks) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Look>**](Look.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_look

> crate::models::LookWithQuery create_look(body, fields)
Create Look

### Create a Look  To create a look to display query data, first create the query with [create_query()](#!/Query/create_query) then assign the query's id to the `query_id` property in the call to `create_look()`.  To place the look into a particular space, assign the space's id to the `space_id` property in the call to `create_look()`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LookWithQuery**](LookWithQuery.md) | Look | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::LookWithQuery**](LookWithQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_look

> String delete_look(look_id)
Delete Look

### Permanently Delete a Look  This operation **permanently** removes a look from the Looker database.  NOTE: There is no \"undo\" for this kind of delete.  For information about soft-delete (which can be undone) see [update_look()](#!/Look/update_look). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Id of look | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## look

> crate::models::LookWithQuery look(look_id, fields)
Get Look

### Get a Look.  Returns detailed information about a Look and its associated Query.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Id of look | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::LookWithQuery**](LookWithQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_look

> String run_look(look_id, result_format, limit, apply_formatting, apply_vis, cache, image_width, image_height, generate_drill_links, force_production, cache_only, path_prefix, rebuild_pdts, server_table_calcs)
Run Look

### Run a Look  Runs a given look's query and returns the results in the requested format.  Supported formats:  | result_format | Description | :-----------: | :--- | | json | Plain json | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | md | Simple markdown | xlsx | MS Excel spreadsheet | sql | Returns the generated SQL rather than running the query | png | A PNG image of the visualization of the query | jpg | A JPG image of the visualization of the query   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Id of look | [required] |
**result_format** | **String** | Format of result | [required] |
**limit** | Option<**i64**> | Row limit (may override the limit in the saved query). |  |
**apply_formatting** | Option<**bool**> | Apply model-specified formatting to each result. |  |
**apply_vis** | Option<**bool**> | Apply visualization options to results. |  |
**cache** | Option<**bool**> | Get results from cache if available. |  |
**image_width** | Option<**i64**> | Render width for image formats. |  |
**image_height** | Option<**i64**> | Render height for image formats. |  |
**generate_drill_links** | Option<**bool**> | Generate drill links (only applicable to 'json_detail' format. |  |
**force_production** | Option<**bool**> | Force use of production models even if the user is in development mode. |  |
**cache_only** | Option<**bool**> | Retrieve any results from cache even if the results have expired. |  |
**path_prefix** | Option<**String**> | Prefix to use for drill links (url encoded). |  |
**rebuild_pdts** | Option<**bool**> | Rebuild PDTS used in query. |  |
**server_table_calcs** | Option<**bool**> | Perform table calculations on query results |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text, application/json, image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_looks

> Vec<crate::models::Look> search_looks(id, title, description, content_favorite_id, space_id, user_id, view_count, deleted, query_id, curate, fields, page, per_page, limit, offset, sorts, filter_or)
Search Looks

### Search Looks  Returns an **array of Look objects** that match the specified search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.   Get a **single look** by id with [look(id)](#!/Look/look) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Match look id. |  |
**title** | Option<**String**> | Match Look title. |  |
**description** | Option<**String**> | Match Look description. |  |
**content_favorite_id** | Option<**i64**> | Select looks with a particular content favorite id |  |
**space_id** | Option<**String**> | Select looks in a particular space. |  |
**user_id** | Option<**String**> | Select looks created by a particular user. |  |
**view_count** | Option<**String**> | Select looks with particular view_count value |  |
**deleted** | Option<**bool**> | Select soft-deleted looks |  |
**query_id** | Option<**i64**> | Select looks that reference a particular query by query_id |  |
**curate** | Option<**bool**> | Exclude items that exist only in personal spaces other than the users |  |
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**limit** | Option<**i64**> | Number of results to return. (used with offset and takes priority over page and per_page) |  |
**offset** | Option<**i64**> | Number of results to skip before returning any. (used with limit and takes priority over page and per_page) |  |
**sorts** | Option<**String**> | One or more fields to sort results by. Sortable fields: [:title, :user_id, :id, :created_at, :space_id, :folder_id, :description, :updated_at, :last_updater_id, :view_count, :favorite_count, :content_favorite_id, :deleted, :deleted_at, :last_viewed_at, :last_accessed_at, :query_id] |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::Look>**](Look.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_look

> crate::models::LookWithQuery update_look(look_id, body, fields)
Update Look

### Modify a Look  Use this function to modify parts of a look. Property values given in a call to `update_look` are applied to the existing look, so there's no need to include properties whose values are not changing. It's best to specify only the properties you want to change and leave everything else out of your `update_look` call. **Look properties marked 'read-only' will be ignored.**  When a user deletes a look in the Looker UI, the look data remains in the database but is marked with a deleted flag (\"soft-deleted\"). Soft-deleted looks can be undeleted (by an admin) if the delete was in error.  To soft-delete a look via the API, use [update_look()](#!/Look/update_look) to change the look's `deleted` property to `true`. You can undelete a look by calling `update_look` to change the look's `deleted` property to `false`.  Soft-deleted looks are excluded from the results of [all_looks()](#!/Look/all_looks) and [search_looks()](#!/Look/search_looks), so they essentially disappear from view even though they still reside in the db. In API 3.1 and later, you can pass `deleted: true` as a parameter to [search_looks()](#!/3.1/Look/search_looks) to list soft-deleted looks.  NOTE: [delete_look()](#!/Look/delete_look) performs a \"hard delete\" - the look data is removed from the Looker database and destroyed. There is no \"undo\" for `delete_look()`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Id of look | [required] |
**body** | [**LookWithQuery**](LookWithQuery.md) | Look | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::LookWithQuery**](LookWithQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

