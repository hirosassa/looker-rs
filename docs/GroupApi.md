# \GroupApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_group**](GroupApi.md#add_group_group) | **POST** /groups/{group_id}/groups | Add a Group to Group
[**add_group_user**](GroupApi.md#add_group_user) | **POST** /groups/{group_id}/users | Add a User to Group
[**all_group_groups**](GroupApi.md#all_group_groups) | **GET** /groups/{group_id}/groups | Get All Groups in Group
[**all_group_users**](GroupApi.md#all_group_users) | **GET** /groups/{group_id}/users | Get All Users in Group
[**all_groups**](GroupApi.md#all_groups) | **GET** /groups | Get All Groups
[**create_group**](GroupApi.md#create_group) | **POST** /groups | Create Group
[**delete_group**](GroupApi.md#delete_group) | **DELETE** /groups/{group_id} | Delete Group
[**delete_group_from_group**](GroupApi.md#delete_group_from_group) | **DELETE** /groups/{group_id}/groups/{deleting_group_id} | Deletes a Group from Group
[**delete_group_user**](GroupApi.md#delete_group_user) | **DELETE** /groups/{group_id}/users/{user_id} | Remove a User from Group
[**delete_user_attribute_group_value**](GroupApi.md#delete_user_attribute_group_value) | **DELETE** /groups/{group_id}/attribute_values/{user_attribute_id} | Delete User Attribute Group Value
[**group**](GroupApi.md#group) | **GET** /groups/{group_id} | Get Group
[**search_groups**](GroupApi.md#search_groups) | **GET** /groups/search | Search Groups
[**update_group**](GroupApi.md#update_group) | **PATCH** /groups/{group_id} | Update Group
[**update_user_attribute_group_value**](GroupApi.md#update_user_attribute_group_value) | **PATCH** /groups/{group_id}/attribute_values/{user_attribute_id} | Set User Attribute Group Value



## add_group_group

> crate::models::Group add_group_group(group_id, body)
Add a Group to Group

### Adds a new group to a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**body** | [**GroupIdForGroupInclusion**](GroupIdForGroupInclusion.md) | Group id to add | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_group_user

> crate::models::User add_group_user(group_id, body)
Add a User to Group

### Adds a new user to a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**body** | [**GroupIdForGroupUserInclusion**](GroupIdForGroupUserInclusion.md) | User id to add | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_group_groups

> Vec<crate::models::Group> all_group_groups(group_id, fields)
Get All Groups in Group

### Get information about all the groups in a group 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_group_users

> Vec<crate::models::User> all_group_users(group_id, fields, page, per_page, sorts)
Get All Users in Group

### Get information about all the users directly included in a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_groups

> Vec<crate::models::Group> all_groups(fields, page, per_page, sorts, ids, content_metadata_id, can_add_to_content_metadata)
Get All Groups

### Get information about all groups. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**ids** | Option<[**Vec<i64>**](i64.md)> | Optional of ids to get specific groups. |  |
**content_metadata_id** | Option<**i64**> | Id of content metadata to which groups must have access. |  |
**can_add_to_content_metadata** | Option<**bool**> | Select only groups that either can/cannot be given access to content. |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::Group create_group(body, fields)
Create Group

### Creates a new group (admin only). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Group**](Group.md) | Group | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> String delete_group(group_id)
Delete Group

### Deletes a group (admin only). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_from_group

> delete_group_from_group(group_id, deleting_group_id)
Deletes a Group from Group

### Removes a group from a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**deleting_group_id** | **i64** | Id of group to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_user

> delete_group_user(group_id, user_id)
Remove a User from Group

### Removes a user from a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**user_id** | **i64** | Id of user to remove from group | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute_group_value

> delete_user_attribute_group_value(group_id, user_attribute_id)
Delete User Attribute Group Value

### Remove a user attribute value from a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**user_attribute_id** | **i64** | Id of user attribute | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group

> crate::models::Group group(group_id, fields)
Get Group

### Get information about a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_groups

> Vec<crate::models::Group> search_groups(fields, limit, offset, sorts, filter_or, id, name, external_group_id, externally_managed, externally_orphaned)
Search Groups

### Search groups  Returns all group records that match the given search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**limit** | Option<**i64**> | Number of results to return (used with `offset`). |  |
**offset** | Option<**i64**> | Number of results to skip before returning any (used with `limit`). |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |
**id** | Option<**i64**> | Match group id. |  |
**name** | Option<**String**> | Match group name. |  |
**external_group_id** | Option<**String**> | Match group external_group_id. |  |
**externally_managed** | Option<**bool**> | Match group externally_managed. |  |
**externally_orphaned** | Option<**bool**> | Match group externally_orphaned. |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> crate::models::Group update_group(group_id, body, fields)
Update Group

### Updates the a group (admin only).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**body** | [**Group**](Group.md) | Group | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_attribute_group_value

> crate::models::UserAttributeGroupValue update_user_attribute_group_value(group_id, user_attribute_id, body)
Set User Attribute Group Value

### Set the value of a user attribute for a group.  For information about how user attribute values are calculated, see [Set User Attribute Group Values](#!/UserAttribute/set_user_attribute_group_values). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Id of group | [required] |
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**body** | [**UserAttributeGroupValue**](UserAttributeGroupValue.md) | New value for group. | [required] |

### Return type

[**crate::models::UserAttributeGroupValue**](UserAttributeGroupValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

