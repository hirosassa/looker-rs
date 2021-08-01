# \ThemeApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**active_themes**](ThemeApi.md#active_themes) | **GET** /themes/active | Get Active Themes
[**all_themes**](ThemeApi.md#all_themes) | **GET** /themes | Get All Themes
[**create_theme**](ThemeApi.md#create_theme) | **POST** /themes | Create Theme
[**default_theme**](ThemeApi.md#default_theme) | **GET** /themes/default | Get Default Theme
[**delete_theme**](ThemeApi.md#delete_theme) | **DELETE** /themes/{theme_id} | Delete Theme
[**search_themes**](ThemeApi.md#search_themes) | **GET** /themes/search | Search Themes
[**set_default_theme**](ThemeApi.md#set_default_theme) | **PUT** /themes/default | Set Default Theme
[**theme**](ThemeApi.md#theme) | **GET** /themes/{theme_id} | Get Theme
[**theme_or_default**](ThemeApi.md#theme_or_default) | **GET** /themes/theme_or_default | Get Theme or Default
[**update_theme**](ThemeApi.md#update_theme) | **PATCH** /themes/{theme_id} | Update Theme
[**validate_theme**](ThemeApi.md#validate_theme) | **POST** /themes/validate | Validate Theme



## active_themes

> Vec<crate::models::Theme> active_themes(name, ts, fields)
Get Active Themes

### Get active themes  Returns an array of active themes.  If the `name` parameter is specified, it will return an array with one theme if it's active and found.  The optional `ts` parameter can specify a different timestamp than \"now.\"  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of theme |  |
**ts** | Option<**String**> | Timestamp representing the target datetime for the active period. Defaults to 'now' |  |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Theme>**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_themes

> Vec<crate::models::Theme> all_themes(fields)
Get All Themes

### Get an array of all existing themes  Get a **single theme** by id with [Theme](#!/Theme/theme)  This method returns an array of all existing themes. The active time for the theme is not considered.  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Theme>**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_theme

> crate::models::Theme create_theme(body)
Create Theme

### Create a theme  Creates a new theme object, returning the theme details, including the created id.  If `settings` are not specified, the default theme settings will be copied into the new theme.  The theme `name` can only contain alphanumeric characters or underscores. Theme names should not contain any confidential information, such as customer names.  **Update** an existing theme with [Update Theme](#!/Theme/update_theme)  **Permanently delete** an existing theme with [Delete Theme](#!/Theme/delete_theme)  For more information, see [Creating and Applying Themes](https://looker.com/docs/r/admin/themes).  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Theme**](Theme.md) | Theme | [required] |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## default_theme

> crate::models::Theme default_theme(ts)
Get Default Theme

### Get the default theme  Returns the active theme object set as the default.  The **default** theme name can be set in the UI on the Admin|Theme UI page  The optional `ts` parameter can specify a different timestamp than \"now.\" If specified, it returns the default theme at the time indicated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ts** | Option<**String**> | Timestamp representing the target datetime for the active period. Defaults to 'now' |  |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_theme

> String delete_theme(theme_id)
Delete Theme

### Delete a specific theme by id  This operation permanently deletes the identified theme from the database.  Because multiple themes can have the same name (with different activation time spans) themes can only be deleted by ID.  All IDs associated with a theme name can be retrieved by searching for the theme name with [Theme Search](#!/Theme/search).  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | Id of theme | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_themes

> Vec<crate::models::Theme> search_themes(id, name, begin_at, end_at, limit, offset, sorts, fields, filter_or)
Search Themes

### Search all themes for matching criteria.  Returns an **array of theme objects** that match the specified search criteria.  | Search Parameters | Description | :-------------------: | :------ | | `begin_at` only | Find themes active at or after `begin_at` | `end_at` only | Find themes active at or before `end_at` | both set | Find themes with an active inclusive period between `begin_at` and `end_at`  Note: Range matching requires boolean AND logic. When using `begin_at` and `end_at` together, do not use `filter_or`=TRUE  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.   Get a **single theme** by id with [Theme](#!/Theme/theme)  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Match theme id. |  |
**name** | Option<**String**> | Match theme name. |  |
**begin_at** | Option<**String**> | Timestamp for activation. |  |
**end_at** | Option<**String**> | Timestamp for expiration. |  |
**limit** | Option<**i64**> | Number of results to return (used with `offset`). |  |
**offset** | Option<**i64**> | Number of results to skip before returning any (used with `limit`). |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**fields** | Option<**String**> | Requested fields. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::Theme>**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_theme

> crate::models::Theme set_default_theme(name)
Set Default Theme

### Set the global default theme by theme name  Only Admin users can call this function.  Only an active theme with no expiration (`end_at` not set) can be assigned as the default theme. As long as a theme has an active record with no expiration, it can be set as the default.  [Create Theme](#!/Theme/create) has detailed information on rules for default and active themes  Returns the new specified default theme object.  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of theme to set as default | [required] |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## theme

> crate::models::Theme theme(theme_id, fields)
Get Theme

### Get a theme by ID  Use this to retrieve a specific theme, whether or not it's currently active.  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | Id of theme | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## theme_or_default

> crate::models::Theme theme_or_default(name, ts)
Get Theme or Default

### Get the named theme if it's active. Otherwise, return the default theme  The optional `ts` parameter can specify a different timestamp than \"now.\" Note: API users with `show` ability can call this function  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of theme | [required] |
**ts** | Option<**String**> | Timestamp representing the target datetime for the active period. Defaults to 'now' |  |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_theme

> crate::models::Theme update_theme(theme_id, body)
Update Theme

### Update the theme by id.  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | Id of theme | [required] |
**body** | [**Theme**](Theme.md) | Theme | [required] |

### Return type

[**crate::models::Theme**](Theme.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_theme

> crate::models::ValidationError validate_theme(body)
Validate Theme

### Validate a theme with the specified information  Validates all values set for the theme, returning any errors encountered, or 200 OK if valid  See [Create Theme](#!/Theme/create_theme) for constraints  **Note**: Custom themes needs to be enabled by Looker. Unless custom themes are enabled, only the automatically generated default theme can be used. Please contact your Account Manager or help.looker.com to update your license for this feature.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Theme**](Theme.md) | Theme | [required] |

### Return type

[**crate::models::ValidationError**](ValidationError.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

