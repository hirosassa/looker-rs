# \DashboardApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_dashboards**](DashboardApi.md#all_dashboards) | **GET** /dashboards | Get All Dashboards
[**create_dashboard**](DashboardApi.md#create_dashboard) | **POST** /dashboards | Create Dashboard
[**create_dashboard_element**](DashboardApi.md#create_dashboard_element) | **POST** /dashboard_elements | Create DashboardElement
[**create_dashboard_filter**](DashboardApi.md#create_dashboard_filter) | **POST** /dashboard_filters | Create Dashboard Filter
[**create_dashboard_layout**](DashboardApi.md#create_dashboard_layout) | **POST** /dashboard_layouts | Create DashboardLayout
[**dashboard**](DashboardApi.md#dashboard) | **GET** /dashboards/{dashboard_id} | Get Dashboard
[**dashboard_aggregate_table_lookml**](DashboardApi.md#dashboard_aggregate_table_lookml) | **GET** /dashboards/aggregate_table_lookml/{dashboard_id} | Get Aggregate Table LookML for a dashboard
[**dashboard_dashboard_elements**](DashboardApi.md#dashboard_dashboard_elements) | **GET** /dashboards/{dashboard_id}/dashboard_elements | Get All DashboardElements
[**dashboard_dashboard_filters**](DashboardApi.md#dashboard_dashboard_filters) | **GET** /dashboards/{dashboard_id}/dashboard_filters | Get All Dashboard Filters
[**dashboard_dashboard_layouts**](DashboardApi.md#dashboard_dashboard_layouts) | **GET** /dashboards/{dashboard_id}/dashboard_layouts | Get All DashboardLayouts
[**dashboard_element**](DashboardApi.md#dashboard_element) | **GET** /dashboard_elements/{dashboard_element_id} | Get DashboardElement
[**dashboard_filter**](DashboardApi.md#dashboard_filter) | **GET** /dashboard_filters/{dashboard_filter_id} | Get Dashboard Filter
[**dashboard_layout**](DashboardApi.md#dashboard_layout) | **GET** /dashboard_layouts/{dashboard_layout_id} | Get DashboardLayout
[**dashboard_layout_component**](DashboardApi.md#dashboard_layout_component) | **GET** /dashboard_layout_components/{dashboard_layout_component_id} | Get DashboardLayoutComponent
[**dashboard_layout_dashboard_layout_components**](DashboardApi.md#dashboard_layout_dashboard_layout_components) | **GET** /dashboard_layouts/{dashboard_layout_id}/dashboard_layout_components | Get All DashboardLayoutComponents
[**dashboard_lookml**](DashboardApi.md#dashboard_lookml) | **GET** /dashboards/lookml/{dashboard_id} | Get lookml of a UDD
[**delete_dashboard**](DashboardApi.md#delete_dashboard) | **DELETE** /dashboards/{dashboard_id} | Delete Dashboard
[**delete_dashboard_element**](DashboardApi.md#delete_dashboard_element) | **DELETE** /dashboard_elements/{dashboard_element_id} | Delete DashboardElement
[**delete_dashboard_filter**](DashboardApi.md#delete_dashboard_filter) | **DELETE** /dashboard_filters/{dashboard_filter_id} | Delete Dashboard Filter
[**delete_dashboard_layout**](DashboardApi.md#delete_dashboard_layout) | **DELETE** /dashboard_layouts/{dashboard_layout_id} | Delete DashboardLayout
[**import_lookml_dashboard**](DashboardApi.md#import_lookml_dashboard) | **POST** /dashboards/{lookml_dashboard_id}/import/{space_id} | Import LookML Dashboard
[**search_dashboard_elements**](DashboardApi.md#search_dashboard_elements) | **GET** /dashboard_elements/search | Search Dashboard Elements
[**search_dashboards**](DashboardApi.md#search_dashboards) | **GET** /dashboards/search | Search Dashboards
[**sync_lookml_dashboard**](DashboardApi.md#sync_lookml_dashboard) | **PATCH** /dashboards/{lookml_dashboard_id}/sync | Sync LookML Dashboard
[**update_dashboard**](DashboardApi.md#update_dashboard) | **PATCH** /dashboards/{dashboard_id} | Update Dashboard
[**update_dashboard_element**](DashboardApi.md#update_dashboard_element) | **PATCH** /dashboard_elements/{dashboard_element_id} | Update DashboardElement
[**update_dashboard_filter**](DashboardApi.md#update_dashboard_filter) | **PATCH** /dashboard_filters/{dashboard_filter_id} | Update Dashboard Filter
[**update_dashboard_layout**](DashboardApi.md#update_dashboard_layout) | **PATCH** /dashboard_layouts/{dashboard_layout_id} | Update DashboardLayout
[**update_dashboard_layout_component**](DashboardApi.md#update_dashboard_layout_component) | **PATCH** /dashboard_layout_components/{dashboard_layout_component_id} | Update DashboardLayoutComponent



## all_dashboards

> Vec<crate::models::DashboardBase> all_dashboards(fields)
Get All Dashboards

### Get information about all active dashboards.  Returns an array of **abbreviated dashboard objects**. Dashboards marked as deleted are excluded from this list.  Get the **full details** of a specific dashboard by id with [dashboard()](#!/Dashboard/dashboard)  Find **deleted dashboards** with [search_dashboards()](#!/Dashboard/search_dashboards) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DashboardBase>**](DashboardBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard

> crate::models::Dashboard create_dashboard(body)
Create Dashboard

### Create a new dashboard  Creates a new dashboard object and returns the details of the newly created dashboard.  `Title`, `user_id`, and `space_id` are all required fields. `Space_id` and `user_id` must contain the id of an existing space or user, respectively. A dashboard's `title` must be unique within the space in which it resides.  If you receive a 422 error response when creating a dashboard, be sure to look at the response body for information about exactly which fields are missing or contain invalid data.  You can **update** an existing dashboard with [update_dashboard()](#!/Dashboard/update_dashboard)  You can **permanently delete** an existing dashboard with [delete_dashboard()](#!/Dashboard/delete_dashboard) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Dashboard**](Dashboard.md) | Dashboard | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard_element

> crate::models::DashboardElement create_dashboard_element(body, fields)
Create DashboardElement

### Create a dashboard element on the dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DashboardElement**](DashboardElement.md) | DashboardElement | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardElement**](DashboardElement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard_filter

> crate::models::DashboardFilter create_dashboard_filter(body, fields)
Create Dashboard Filter

### Create a dashboard filter on the dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateDashboardFilter**](CreateDashboardFilter.md) | Dashboard Filter | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::DashboardFilter**](DashboardFilter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard_layout

> crate::models::DashboardLayout create_dashboard_layout(body, fields)
Create DashboardLayout

### Create a dashboard layout on the dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DashboardLayout**](DashboardLayout.md) | DashboardLayout | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardLayout**](DashboardLayout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard

> crate::models::Dashboard dashboard(dashboard_id, fields)
Get Dashboard

### Get information about a dashboard  Returns the full details of the identified dashboard object  Get a **summary list** of all active dashboards with [all_dashboards()](#!/Dashboard/all_dashboards)  You can **Search** for dashboards with [search_dashboards()](#!/Dashboard/search_dashboards) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_aggregate_table_lookml

> crate::models::DashboardAggregateTableLookml dashboard_aggregate_table_lookml(dashboard_id)
Get Aggregate Table LookML for a dashboard

### Get Aggregate Table LookML for Each Query on a Dahboard  Returns a JSON object that contains the dashboard id and Aggregate Table lookml  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |

### Return type

[**crate::models::DashboardAggregateTableLookml**](DashboardAggregateTableLookml.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_dashboard_elements

> Vec<crate::models::DashboardElement> dashboard_dashboard_elements(dashboard_id, fields)
Get All DashboardElements

### Get information about all the dashboard elements on a dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DashboardElement>**](DashboardElement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_dashboard_filters

> Vec<crate::models::DashboardFilter> dashboard_dashboard_filters(dashboard_id, fields)
Get All Dashboard Filters

### Get information about all the dashboard filters on a dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DashboardFilter>**](DashboardFilter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_dashboard_layouts

> Vec<crate::models::DashboardLayout> dashboard_dashboard_layouts(dashboard_id, fields)
Get All DashboardLayouts

### Get information about all the dashboard elements on a dashboard with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DashboardLayout>**](DashboardLayout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_element

> crate::models::DashboardElement dashboard_element(dashboard_element_id, fields)
Get DashboardElement

### Get information about the dashboard element with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_element_id** | **String** | Id of dashboard element | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardElement**](DashboardElement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_filter

> crate::models::DashboardFilter dashboard_filter(dashboard_filter_id, fields)
Get Dashboard Filter

### Get information about the dashboard filters with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_filter_id** | **String** | Id of dashboard filters | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardFilter**](DashboardFilter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_layout

> crate::models::DashboardLayout dashboard_layout(dashboard_layout_id, fields)
Get DashboardLayout

### Get information about the dashboard layouts with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_id** | **String** | Id of dashboard layouts | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardLayout**](DashboardLayout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_layout_component

> crate::models::DashboardLayoutComponent dashboard_layout_component(dashboard_layout_component_id, fields)
Get DashboardLayoutComponent

### Get information about the dashboard elements with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_component_id** | **String** | Id of dashboard layout component | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardLayoutComponent**](DashboardLayoutComponent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_layout_dashboard_layout_components

> Vec<crate::models::DashboardLayoutComponent> dashboard_layout_dashboard_layout_components(dashboard_layout_id, fields)
Get All DashboardLayoutComponents

### Get information about all the dashboard layout components for a dashboard layout with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_id** | **String** | Id of dashboard layout component | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DashboardLayoutComponent>**](DashboardLayoutComponent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboard_lookml

> crate::models::DashboardLookml dashboard_lookml(dashboard_id)
Get lookml of a UDD

### Get lookml of a UDD  Returns a JSON object that contains the dashboard id and the full lookml  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |

### Return type

[**crate::models::DashboardLookml**](DashboardLookml.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard

> String delete_dashboard(dashboard_id)
Delete Dashboard

### Delete the dashboard with the specified id  Permanently **deletes** a dashboard. (The dashboard cannot be recovered after this operation.)  \"Soft\" delete or hide a dashboard by setting its `deleted` status to `True` with [update_dashboard()](#!/Dashboard/update_dashboard).  Note: When a dashboard is deleted in the UI, it is soft deleted. Use this API call to permanently remove it, if desired. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_element

> String delete_dashboard_element(dashboard_element_id)
Delete DashboardElement

### Delete a dashboard element with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_element_id** | **String** | Id of dashboard element | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_filter

> String delete_dashboard_filter(dashboard_filter_id)
Delete Dashboard Filter

### Delete a dashboard filter with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_filter_id** | **String** | Id of dashboard filter | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_layout

> String delete_dashboard_layout(dashboard_layout_id)
Delete DashboardLayout

### Delete a dashboard layout with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_id** | **String** | Id of dashboard layout | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_lookml_dashboard

> crate::models::Dashboard import_lookml_dashboard(lookml_dashboard_id, space_id, raw_locale, body)
Import LookML Dashboard

### Import a LookML dashboard to a space as a UDD Creates a UDD (a dashboard which exists in the Looker database rather than as a LookML file) from the LookML dashboard and places it in the space specified. The created UDD will have a lookml_link_id which links to the original LookML dashboard.  To give the imported dashboard specify a (e.g. title: \"my title\") in the body of your request, otherwise the imported dashboard will have the same title as the original LookML dashboard.  For this operation to succeed the user must have permission to see the LookML dashboard in question, and have permission to create content in the space the dashboard is being imported to.  **Sync** a linked UDD with [sync_lookml_dashboard()](#!/Dashboard/sync_lookml_dashboard) **Unlink** a linked UDD by setting lookml_link_id to null with [update_dashboard()](#!/Dashboard/update_dashboard) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_dashboard_id** | **String** | Id of LookML dashboard | [required] |
**space_id** | **String** | Id of space to import the dashboard to | [required] |
**raw_locale** | Option<**bool**> | If true, and this dashboard is localized, export it with the raw keys, not localized. |  |
**body** | Option<[**Dashboard**](Dashboard.md)> | Dashboard |  |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_dashboard_elements

> Vec<crate::models::DashboardElement> search_dashboard_elements(dashboard_id, look_id, title, deleted, fields, filter_or, sorts)
Search Dashboard Elements

### Search Dashboard Elements  Returns an **array of DashboardElement objects** that match the specified search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | Option<**i64**> | Select elements that refer to a given dashboard id |  |
**look_id** | Option<**i64**> | Select elements that refer to a given look id |  |
**title** | Option<**String**> | Match the title of element |  |
**deleted** | Option<**bool**> | Select soft-deleted dashboard elements |  |
**fields** | Option<**String**> | Requested fields. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |
**sorts** | Option<**String**> | Fields to sort by. Sortable fields: [:look_id, :dashboard_id, :deleted, :title] |  |

### Return type

[**Vec<crate::models::DashboardElement>**](DashboardElement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_dashboards

> Vec<crate::models::Dashboard> search_dashboards(id, slug, title, description, content_favorite_id, space_id, folder_id, deleted, user_id, view_count, content_metadata_id, curate, fields, page, per_page, limit, offset, sorts, filter_or)
Search Dashboards

### Search Dashboards  Returns an **array of dashboard objects** that match the specified search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.   The parameters `limit`, and `offset` are recommended for fetching results in page-size chunks.  Get a **single dashboard** by id with [dashboard()](#!/Dashboard/dashboard) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Match dashboard id. |  |
**slug** | Option<**String**> | Match dashboard slug. |  |
**title** | Option<**String**> | Match Dashboard title. |  |
**description** | Option<**String**> | Match Dashboard description. |  |
**content_favorite_id** | Option<**i64**> | Filter on a content favorite id. |  |
**space_id** | Option<**String**> | Filter on a particular space. |  |
**folder_id** | Option<**String**> | Filter on a particular space. |  |
**deleted** | Option<**String**> | Filter on dashboards deleted status. |  |
**user_id** | Option<**String**> | Filter on dashboards created by a particular user. |  |
**view_count** | Option<**String**> | Filter on a particular value of view_count |  |
**content_metadata_id** | Option<**i64**> | Filter on a content favorite id. |  |
**curate** | Option<**bool**> | Exclude items that exist only in personal spaces other than the users |  |
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**limit** | Option<**i64**> | Number of results to return. (used with offset and takes priority over page and per_page) |  |
**offset** | Option<**i64**> | Number of results to skip before returning any. (used with limit and takes priority over page and per_page) |  |
**sorts** | Option<**String**> | One or more fields to sort by. Sortable fields: [:title, :user_id, :id, :created_at, :space_id, :folder_id, :description, :view_count, :favorite_count, :slug, :content_favorite_id, :content_metadata_id, :deleted, :deleted_at, :last_viewed_at, :last_accessed_at] |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::Dashboard>**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_lookml_dashboard

> Vec<i64> sync_lookml_dashboard(lookml_dashboard_id, body, raw_locale)
Sync LookML Dashboard

### Update all linked dashboards to match the specified LookML dashboard.  Any UDD (a dashboard which exists in the Looker database rather than as a LookML file) which has a `lookml_link_id` property value referring to a LookML dashboard's id (model::dashboardname) will be updated so that it matches the current state of the LookML dashboard.  For this operation to succeed the user must have permission to view the LookML dashboard, and only linked dashboards that the user has permission to update will be synced.  To **link** or **unlink** a UDD set the `lookml_link_id` property with [update_dashboard()](#!/Dashboard/update_dashboard) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_dashboard_id** | **String** | Id of LookML dashboard, in the form 'model::dashboardname' | [required] |
**body** | [**Dashboard**](Dashboard.md) | Dashboard | [required] |
**raw_locale** | Option<**bool**> | If true, and this dashboard is localized, export it with the raw keys, not localized. |  |

### Return type

**Vec<i64>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard

> crate::models::Dashboard update_dashboard(dashboard_id, body)
Update Dashboard

### Update a dashboard  You can use this function to change the string and integer properties of a dashboard. Nested objects such as filters, dashboard elements, or dashboard layout components cannot be modified by this function - use the update functions for the respective nested object types (like [update_dashboard_filter()](#!/3.1/Dashboard/update_dashboard_filter) to change a filter) to modify nested objects referenced by a dashboard.  If you receive a 422 error response when updating a dashboard, be sure to look at the response body for information about exactly which fields are missing or contain invalid data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of dashboard | [required] |
**body** | [**Dashboard**](Dashboard.md) | Dashboard | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_element

> crate::models::DashboardElement update_dashboard_element(dashboard_element_id, body, fields)
Update DashboardElement

### Update the dashboard element with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_element_id** | **String** | Id of dashboard element | [required] |
**body** | [**DashboardElement**](DashboardElement.md) | DashboardElement | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardElement**](DashboardElement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_filter

> crate::models::DashboardFilter update_dashboard_filter(dashboard_filter_id, body, fields)
Update Dashboard Filter

### Update the dashboard filter with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_filter_id** | **String** | Id of dashboard filter | [required] |
**body** | [**DashboardFilter**](DashboardFilter.md) | Dashboard Filter | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardFilter**](DashboardFilter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_layout

> crate::models::DashboardLayout update_dashboard_layout(dashboard_layout_id, body, fields)
Update DashboardLayout

### Update the dashboard layout with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_id** | **String** | Id of dashboard layout | [required] |
**body** | [**DashboardLayout**](DashboardLayout.md) | DashboardLayout | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardLayout**](DashboardLayout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_layout_component

> crate::models::DashboardLayoutComponent update_dashboard_layout_component(dashboard_layout_component_id, body, fields)
Update DashboardLayoutComponent

### Update the dashboard element with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_layout_component_id** | **String** | Id of dashboard layout component | [required] |
**body** | [**DashboardLayoutComponent**](DashboardLayoutComponent.md) | DashboardLayoutComponent | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DashboardLayoutComponent**](DashboardLayoutComponent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

