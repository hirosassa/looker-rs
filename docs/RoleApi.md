# \RoleApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_model_sets**](RoleApi.md#all_model_sets) | **GET** /model_sets | Get All Model Sets
[**all_permission_sets**](RoleApi.md#all_permission_sets) | **GET** /permission_sets | Get All Permission Sets
[**all_permissions**](RoleApi.md#all_permissions) | **GET** /permissions | Get All Permissions
[**all_roles**](RoleApi.md#all_roles) | **GET** /roles | Get All Roles
[**create_model_set**](RoleApi.md#create_model_set) | **POST** /model_sets | Create Model Set
[**create_permission_set**](RoleApi.md#create_permission_set) | **POST** /permission_sets | Create Permission Set
[**create_role**](RoleApi.md#create_role) | **POST** /roles | Create Role
[**delete_model_set**](RoleApi.md#delete_model_set) | **DELETE** /model_sets/{model_set_id} | Delete Model Set
[**delete_permission_set**](RoleApi.md#delete_permission_set) | **DELETE** /permission_sets/{permission_set_id} | Delete Permission Set
[**delete_role**](RoleApi.md#delete_role) | **DELETE** /roles/{role_id} | Delete Role
[**model_set**](RoleApi.md#model_set) | **GET** /model_sets/{model_set_id} | Get Model Set
[**permission_set**](RoleApi.md#permission_set) | **GET** /permission_sets/{permission_set_id} | Get Permission Set
[**role**](RoleApi.md#role) | **GET** /roles/{role_id} | Get Role
[**role_groups**](RoleApi.md#role_groups) | **GET** /roles/{role_id}/groups | Get Role Groups
[**role_users**](RoleApi.md#role_users) | **GET** /roles/{role_id}/users | Get Role Users
[**search_model_sets**](RoleApi.md#search_model_sets) | **GET** /model_sets/search | Search Model Sets
[**search_permission_sets**](RoleApi.md#search_permission_sets) | **GET** /permission_sets/search | Search Permission Sets
[**search_roles**](RoleApi.md#search_roles) | **GET** /roles/search | Search Roles
[**set_role_groups**](RoleApi.md#set_role_groups) | **PUT** /roles/{role_id}/groups | Update Role Groups
[**set_role_users**](RoleApi.md#set_role_users) | **PUT** /roles/{role_id}/users | Update Role Users
[**update_model_set**](RoleApi.md#update_model_set) | **PATCH** /model_sets/{model_set_id} | Update Model Set
[**update_permission_set**](RoleApi.md#update_permission_set) | **PATCH** /permission_sets/{permission_set_id} | Update Permission Set
[**update_role**](RoleApi.md#update_role) | **PATCH** /roles/{role_id} | Update Role



## all_model_sets

> Vec<crate::models::ModelSet> all_model_sets(fields)
Get All Model Sets

### Get information about all model sets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ModelSet>**](ModelSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_permission_sets

> Vec<crate::models::PermissionSet> all_permission_sets(fields)
Get All Permission Sets

### Get information about all permission sets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::PermissionSet>**](PermissionSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_permissions

> Vec<crate::models::Permission> all_permissions()
Get All Permissions

### Get all supported permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Permission>**](Permission.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_roles

> Vec<crate::models::Role> all_roles(fields, ids)
Get All Roles

### Get information about all roles. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**ids** | Option<[**Vec<i64>**](i64.md)> | Optional list of ids to get specific roles. |  |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_model_set

> crate::models::ModelSet create_model_set(body)
Create Model Set

### Create a model set with the specified information. Model sets are used by Roles. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelSet**](ModelSet.md) | ModelSet | [required] |

### Return type

[**crate::models::ModelSet**](ModelSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_permission_set

> crate::models::PermissionSet create_permission_set(body)
Create Permission Set

### Create a permission set with the specified information. Permission sets are used by Roles. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PermissionSet**](PermissionSet.md) | Permission Set | [required] |

### Return type

[**crate::models::PermissionSet**](PermissionSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> crate::models::Role create_role(body)
Create Role

### Create a role with the specified information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Role**](Role.md) | Role | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_model_set

> String delete_model_set(model_set_id)
Delete Model Set

### Delete the model set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_set_id** | **i64** | id of model set | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_permission_set

> String delete_permission_set(permission_set_id)
Delete Permission Set

### Delete the permission set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_set_id** | **i64** | Id of permission set | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> String delete_role(role_id)
Delete Role

### Delete the role with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of role | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_set

> crate::models::ModelSet model_set(model_set_id, fields)
Get Model Set

### Get information about the model set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_set_id** | **i64** | Id of model set | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ModelSet**](ModelSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permission_set

> crate::models::PermissionSet permission_set(permission_set_id, fields)
Get Permission Set

### Get information about the permission set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_set_id** | **i64** | Id of permission set | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::PermissionSet**](PermissionSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role

> crate::models::Role role(role_id)
Get Role

### Get information about the role with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of role | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_groups

> Vec<crate::models::Group> role_groups(role_id, fields)
Get Role Groups

### Get information about all the groups with the role that has a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of role | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_users

> Vec<crate::models::User> role_users(role_id, fields, direct_association_only)
Get Role Users

### Get information about all the users with the role that has a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**direct_association_only** | Option<**bool**> | Get only users associated directly with the role: exclude those only associated through groups. |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_model_sets

> Vec<crate::models::ModelSet> search_model_sets(fields, limit, offset, sorts, id, name, all_access, built_in, filter_or)
Search Model Sets

### Search model sets Returns all model set records that match the given search criteria. If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**limit** | Option<**i64**> | Number of results to return (used with `offset`). |  |
**offset** | Option<**i64**> | Number of results to skip before returning any (used with `limit`). |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**id** | Option<**i64**> | Match model set id. |  |
**name** | Option<**String**> | Match model set name. |  |
**all_access** | Option<**bool**> | Match model sets by all_access status. |  |
**built_in** | Option<**bool**> | Match model sets by built_in status. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression. |  |

### Return type

[**Vec<crate::models::ModelSet>**](ModelSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_permission_sets

> Vec<crate::models::PermissionSet> search_permission_sets(fields, limit, offset, sorts, id, name, all_access, built_in, filter_or)
Search Permission Sets

### Search permission sets Returns all permission set records that match the given search criteria. If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**limit** | Option<**i64**> | Number of results to return (used with `offset`). |  |
**offset** | Option<**i64**> | Number of results to skip before returning any (used with `limit`). |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**id** | Option<**i64**> | Match permission set id. |  |
**name** | Option<**String**> | Match permission set name. |  |
**all_access** | Option<**bool**> | Match permission sets by all_access status. |  |
**built_in** | Option<**bool**> | Match permission sets by built_in status. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression. |  |

### Return type

[**Vec<crate::models::PermissionSet>**](PermissionSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_roles

> Vec<crate::models::Role> search_roles(fields, limit, offset, sorts, id, name, built_in, filter_or)
Search Roles

### Search roles  Returns all role records that match the given search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**limit** | Option<**i64**> | Number of results to return (used with `offset`). |  |
**offset** | Option<**i64**> | Number of results to skip before returning any (used with `limit`). |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**id** | Option<**i64**> | Match role id. |  |
**name** | Option<**String**> | Match role name. |  |
**built_in** | Option<**bool**> | Match roles by built_in status. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression. |  |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_role_groups

> Vec<crate::models::Group> set_role_groups(role_id, body)
Update Role Groups

### Set all groups for a role, removing all existing group associations from that role. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | Id of Role | [required] |
**body** | [**Vec<i64>**](i64.md) | Array of Group Ids | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_role_users

> Vec<crate::models::User> set_role_users(role_id, body)
Update Role Users

### Set all the users of the role with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of role | [required] |
**body** | [**Vec<i64>**](i64.md) | array of user ids for role | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_model_set

> crate::models::ModelSet update_model_set(model_set_id, body)
Update Model Set

### Update information about the model set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_set_id** | **i64** | id of model set | [required] |
**body** | [**ModelSet**](ModelSet.md) | ModelSet | [required] |

### Return type

[**crate::models::ModelSet**](ModelSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permission_set

> crate::models::PermissionSet update_permission_set(permission_set_id, body)
Update Permission Set

### Update information about the permission set with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_set_id** | **i64** | id of permission set | [required] |
**body** | [**PermissionSet**](PermissionSet.md) | Permission Set | [required] |

### Return type

[**crate::models::PermissionSet**](PermissionSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> crate::models::Role update_role(role_id, body)
Update Role

### Update information about the role with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i64** | id of role | [required] |
**body** | [**Role**](Role.md) | Role | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

