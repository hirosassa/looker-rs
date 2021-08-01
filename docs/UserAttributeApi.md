# \UserAttributeApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_user_attribute_group_values**](UserAttributeApi.md#all_user_attribute_group_values) | **GET** /user_attributes/{user_attribute_id}/group_values | Get User Attribute Group Values
[**all_user_attributes**](UserAttributeApi.md#all_user_attributes) | **GET** /user_attributes | Get All User Attributes
[**create_user_attribute**](UserAttributeApi.md#create_user_attribute) | **POST** /user_attributes | Create User Attribute
[**delete_user_attribute**](UserAttributeApi.md#delete_user_attribute) | **DELETE** /user_attributes/{user_attribute_id} | Delete User Attribute
[**set_user_attribute_group_values**](UserAttributeApi.md#set_user_attribute_group_values) | **POST** /user_attributes/{user_attribute_id}/group_values | Set User Attribute Group Values
[**update_user_attribute**](UserAttributeApi.md#update_user_attribute) | **PATCH** /user_attributes/{user_attribute_id} | Update User Attribute
[**user_attribute**](UserAttributeApi.md#user_attribute) | **GET** /user_attributes/{user_attribute_id} | Get User Attribute



## all_user_attribute_group_values

> Vec<crate::models::UserAttributeGroupValue> all_user_attribute_group_values(user_attribute_id, fields)
Get User Attribute Group Values

### Returns all values of a user attribute defined by user groups, in precedence order.  A user may be a member of multiple groups which define different values for a given user attribute. The order of group-values in the response determines precedence for selecting which group-value applies to a given user.  For more information, see [Set User Attribute Group Values](#!/UserAttribute/set_user_attribute_group_values).  Results will only include groups that the caller's user account has permission to see. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::UserAttributeGroupValue>**](UserAttributeGroupValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_user_attributes

> Vec<crate::models::UserAttribute> all_user_attributes(fields, sorts)
Get All User Attributes

### Get information about all user attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**sorts** | Option<**String**> | Fields to order the results by. Sortable fields include: name, label |  |

### Return type

[**Vec<crate::models::UserAttribute>**](UserAttribute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_attribute

> crate::models::UserAttribute create_user_attribute(body, fields)
Create User Attribute

### Create a new user attribute  Permission information for a user attribute is conveyed through the `can` and `user_can_edit` fields. The `user_can_edit` field indicates whether an attribute is user-editable _anywhere_ in the application. The `can` field gives more granular access information, with the `set_value` child field indicating whether an attribute's value can be set by [Setting the User Attribute User Value](#!/User/set_user_attribute_user_value).  Note: `name` and `label` fields must be unique across all user attributes in the Looker instance. Attempting to create a new user attribute with a name or label that duplicates an existing user attribute will fail with a 422 error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserAttribute**](UserAttribute.md) | User Attribute | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::UserAttribute**](UserAttribute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute

> String delete_user_attribute(user_attribute_id)
Delete User Attribute

### Delete a user attribute (admin only). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_attribute_id** | **i64** | Id of user_attribute | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_attribute_group_values

> Vec<crate::models::UserAttributeGroupValue> set_user_attribute_group_values(user_attribute_id, body)
Set User Attribute Group Values

### Define values for a user attribute across a set of groups, in priority order.  This function defines all values for a user attribute defined by user groups. This is a global setting, potentially affecting all users in the system. This function replaces any existing group value definitions for the indicated user attribute.  The value of a user attribute for a given user is determined by searching the following locations, in this order:  1. the user's account settings 2. the groups that the user is a member of 3. the default value of the user attribute, if any  The user may be a member of multiple groups which define different values for that user attribute. The order of items in the group_values parameter determines which group takes priority for that user. Lowest array index wins.  An alternate method to indicate the selection precedence of group-values is to assign numbers to the 'rank' property of each group-value object in the array. Lowest 'rank' value wins. If you use this technique, you must assign a rank value to every group-value object in the array.    To set a user attribute value for a single user, see [Set User Attribute User Value](#!/User/set_user_attribute_user_value). To set a user attribute value for all members of a group, see [Set User Attribute Group Value](#!/Group/update_user_attribute_group_value). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**body** | [**Vec<crate::models::UserAttributeGroupValue>**](UserAttributeGroupValue.md) | Array of group values. | [required] |

### Return type

[**Vec<crate::models::UserAttributeGroupValue>**](UserAttributeGroupValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_attribute

> crate::models::UserAttribute update_user_attribute(user_attribute_id, body, fields)
Update User Attribute

### Update a user attribute definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**body** | [**UserAttribute**](UserAttribute.md) | User Attribute | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::UserAttribute**](UserAttribute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_attribute

> crate::models::UserAttribute user_attribute(user_attribute_id, fields)
Get User Attribute

### Get information about a user attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::UserAttribute**](UserAttribute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

