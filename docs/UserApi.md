# \UserApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_user_credentials_api3s**](UserApi.md#all_user_credentials_api3s) | **GET** /users/{user_id}/credentials_api3 | Get All API 3 Credentials
[**all_user_credentials_embeds**](UserApi.md#all_user_credentials_embeds) | **GET** /users/{user_id}/credentials_embed | Get All Embedding Credentials
[**all_user_sessions**](UserApi.md#all_user_sessions) | **GET** /users/{user_id}/sessions | Get All Web Login Sessions
[**all_users**](UserApi.md#all_users) | **GET** /users | Get All Users
[**create_user**](UserApi.md#create_user) | **POST** /users | Create User
[**create_user_credentials_api3**](UserApi.md#create_user_credentials_api3) | **POST** /users/{user_id}/credentials_api3 | Create API 3 Credential
[**create_user_credentials_email**](UserApi.md#create_user_credentials_email) | **POST** /users/{user_id}/credentials_email | Create Email/Password Credential
[**create_user_credentials_email_password_reset**](UserApi.md#create_user_credentials_email_password_reset) | **POST** /users/{user_id}/credentials_email/password_reset | Create Password Reset Token
[**create_user_credentials_totp**](UserApi.md#create_user_credentials_totp) | **POST** /users/{user_id}/credentials_totp | Create Two-Factor Credential
[**delete_user**](UserApi.md#delete_user) | **DELETE** /users/{user_id} | Delete User
[**delete_user_attribute_user_value**](UserApi.md#delete_user_attribute_user_value) | **DELETE** /users/{user_id}/attribute_values/{user_attribute_id} | Delete User Attribute User Value
[**delete_user_credentials_api3**](UserApi.md#delete_user_credentials_api3) | **DELETE** /users/{user_id}/credentials_api3/{credentials_api3_id} | Delete API 3 Credential
[**delete_user_credentials_email**](UserApi.md#delete_user_credentials_email) | **DELETE** /users/{user_id}/credentials_email | Delete Email/Password Credential
[**delete_user_credentials_embed**](UserApi.md#delete_user_credentials_embed) | **DELETE** /users/{user_id}/credentials_embed/{credentials_embed_id} | Delete Embedding Credential
[**delete_user_credentials_google**](UserApi.md#delete_user_credentials_google) | **DELETE** /users/{user_id}/credentials_google | Delete Google Auth Credential
[**delete_user_credentials_ldap**](UserApi.md#delete_user_credentials_ldap) | **DELETE** /users/{user_id}/credentials_ldap | Delete LDAP Credential
[**delete_user_credentials_looker_openid**](UserApi.md#delete_user_credentials_looker_openid) | **DELETE** /users/{user_id}/credentials_looker_openid | Delete Looker OpenId Credential
[**delete_user_credentials_oidc**](UserApi.md#delete_user_credentials_oidc) | **DELETE** /users/{user_id}/credentials_oidc | Delete OIDC Auth Credential
[**delete_user_credentials_saml**](UserApi.md#delete_user_credentials_saml) | **DELETE** /users/{user_id}/credentials_saml | Delete Saml Auth Credential
[**delete_user_credentials_totp**](UserApi.md#delete_user_credentials_totp) | **DELETE** /users/{user_id}/credentials_totp | Delete Two-Factor Credential
[**delete_user_session**](UserApi.md#delete_user_session) | **DELETE** /users/{user_id}/sessions/{session_id} | Delete Web Login Session
[**me**](UserApi.md#me) | **GET** /user | Get Current User
[**search_users**](UserApi.md#search_users) | **GET** /users/search | Search Users
[**search_users_names**](UserApi.md#search_users_names) | **GET** /users/search/names/{pattern} | Search User Names
[**set_user_attribute_user_value**](UserApi.md#set_user_attribute_user_value) | **PATCH** /users/{user_id}/attribute_values/{user_attribute_id} | Set User Attribute User Value
[**set_user_roles**](UserApi.md#set_user_roles) | **PUT** /users/{user_id}/roles | Set User Roles
[**update_user**](UserApi.md#update_user) | **PATCH** /users/{user_id} | Update User
[**update_user_credentials_email**](UserApi.md#update_user_credentials_email) | **PATCH** /users/{user_id}/credentials_email | Update Email/Password Credential
[**user**](UserApi.md#user) | **GET** /users/{user_id} | Get User by Id
[**user_attribute_user_values**](UserApi.md#user_attribute_user_values) | **GET** /users/{user_id}/attribute_values | Get User Attribute Values
[**user_credentials_api3**](UserApi.md#user_credentials_api3) | **GET** /users/{user_id}/credentials_api3/{credentials_api3_id} | Get API 3 Credential
[**user_credentials_email**](UserApi.md#user_credentials_email) | **GET** /users/{user_id}/credentials_email | Get Email/Password Credential
[**user_credentials_embed**](UserApi.md#user_credentials_embed) | **GET** /users/{user_id}/credentials_embed/{credentials_embed_id} | Get Embedding Credential
[**user_credentials_google**](UserApi.md#user_credentials_google) | **GET** /users/{user_id}/credentials_google | Get Google Auth Credential
[**user_credentials_ldap**](UserApi.md#user_credentials_ldap) | **GET** /users/{user_id}/credentials_ldap | Get LDAP Credential
[**user_credentials_looker_openid**](UserApi.md#user_credentials_looker_openid) | **GET** /users/{user_id}/credentials_looker_openid | Get Looker OpenId Credential
[**user_credentials_oidc**](UserApi.md#user_credentials_oidc) | **GET** /users/{user_id}/credentials_oidc | Get OIDC Auth Credential
[**user_credentials_saml**](UserApi.md#user_credentials_saml) | **GET** /users/{user_id}/credentials_saml | Get Saml Auth Credential
[**user_credentials_totp**](UserApi.md#user_credentials_totp) | **GET** /users/{user_id}/credentials_totp | Get Two-Factor Credential
[**user_for_credential**](UserApi.md#user_for_credential) | **GET** /users/credential/{credential_type}/{credential_id} | Get User by Credential Id
[**user_roles**](UserApi.md#user_roles) | **GET** /users/{user_id}/roles | Get User Roles
[**user_session**](UserApi.md#user_session) | **GET** /users/{user_id}/sessions/{session_id} | Get Web Login Session



## all_user_credentials_api3s

> Vec<crate::models::CredentialsApi3> all_user_credentials_api3s(user_id, fields)
Get All API 3 Credentials

### API 3 login information for the specified user. This is for the newer API keys that can be added for any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::CredentialsApi3>**](CredentialsApi3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_user_credentials_embeds

> Vec<crate::models::CredentialsEmbed> all_user_credentials_embeds(user_id, fields)
Get All Embedding Credentials

### Embed login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::CredentialsEmbed>**](CredentialsEmbed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_user_sessions

> Vec<crate::models::Session> all_user_sessions(user_id, fields)
Get All Web Login Sessions

### Web login session for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Session>**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_users

> Vec<crate::models::User> all_users(fields, page, per_page, sorts, ids)
Get All Users

### Get information about all users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**ids** | Option<[**Vec<i64>**](i64.md)> | Optional list of ids to get specific users. |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::User create_user(fields, body)
Create User

### Create a user with the specified information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**body** | Option<[**User**](User.md)> | User |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_credentials_api3

> crate::models::CredentialsApi3 create_user_credentials_api3(user_id, fields, body)
Create API 3 Credential

### API 3 login information for the specified user. This is for the newer API keys that can be added for any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**body** | Option<[**CredentialsApi3**](CredentialsApi3.md)> | API 3 Credential |  |

### Return type

[**crate::models::CredentialsApi3**](CredentialsApi3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_credentials_email

> crate::models::CredentialsEmail create_user_credentials_email(user_id, body, fields)
Create Email/Password Credential

### Email/password login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**body** | [**CredentialsEmail**](CredentialsEmail.md) | Email/Password Credential | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsEmail**](CredentialsEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_credentials_email_password_reset

> crate::models::CredentialsEmail create_user_credentials_email_password_reset(user_id, expires, fields)
Create Password Reset Token

### Create a password reset token. This will create a cryptographically secure random password reset token for the user. If the user already has a password reset token then this invalidates the old token and creates a new one. The token is expressed as the 'password_reset_url' of the user's email/password credential object. This takes an optional 'expires' param to indicate if the new token should be an expiring token. Tokens that expire are typically used for self-service password resets for existing users. Invitation emails for new users typically are not set to expire. The expire period is always 60 minutes when expires is enabled. This method can be called with an empty body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**expires** | Option<**bool**> | Expiring token. |  |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsEmail**](CredentialsEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_credentials_totp

> crate::models::CredentialsTotp create_user_credentials_totp(user_id, fields, body)
Create Two-Factor Credential

### Two-factor login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**body** | Option<[**CredentialsTotp**](CredentialsTotp.md)> | Two-Factor Credential |  |

### Return type

[**crate::models::CredentialsTotp**](CredentialsTotp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> String delete_user(user_id)
Delete User

### Delete the user with a specific id.  **DANGER** this will delete the user and all looks and other information owned by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute_user_value

> delete_user_attribute_user_value(user_id, user_attribute_id)
Delete User Attribute User Value

### Delete a user attribute value from a user's account settings.  After the user attribute value is deleted from the user's account settings, subsequent requests for the user attribute value for this user will draw from the user's groups or the default value of the user attribute. See [Get User Attribute Values](#!/User/user_attribute_user_values) for more information about how user attribute values are resolved. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**user_attribute_id** | **i64** | Id of user attribute | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_api3

> String delete_user_credentials_api3(user_id, credentials_api3_id)
Delete API 3 Credential

### API 3 login information for the specified user. This is for the newer API keys that can be added for any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**credentials_api3_id** | **i64** | id of API 3 Credential | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_email

> String delete_user_credentials_email(user_id)
Delete Email/Password Credential

### Email/password login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_embed

> String delete_user_credentials_embed(user_id, credentials_embed_id)
Delete Embedding Credential

### Embed login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**credentials_embed_id** | **i64** | id of Embedding Credential | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_google

> String delete_user_credentials_google(user_id)
Delete Google Auth Credential

### Google authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_ldap

> String delete_user_credentials_ldap(user_id)
Delete LDAP Credential

### LDAP login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_looker_openid

> String delete_user_credentials_looker_openid(user_id)
Delete Looker OpenId Credential

### Looker Openid login information for the specified user. Used by Looker Analysts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_oidc

> String delete_user_credentials_oidc(user_id)
Delete OIDC Auth Credential

### OpenID Connect (OIDC) authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_saml

> String delete_user_credentials_saml(user_id)
Delete Saml Auth Credential

### Saml authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_credentials_totp

> String delete_user_credentials_totp(user_id)
Delete Two-Factor Credential

### Two-factor login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_session

> String delete_user_session(user_id, session_id)
Delete Web Login Session

### Web login session for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**session_id** | **i64** | id of Web Login Session | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## me

> crate::models::User me(fields)
Get Current User

### Get information about the current user; i.e. the user account currently calling the API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<crate::models::User> search_users(fields, page, per_page, sorts, id, first_name, last_name, verified_looker_employee, embed_user, email, is_disabled, filter_or, content_metadata_id, group_id)
Search Users

### Search users  Returns all<sup>*</sup> user records that match the given search criteria.  If multiple search params are given and `filter_or` is FALSE or not specified, search params are combined in a logical AND operation. Only rows that match *all* search param criteria will be returned.  If `filter_or` is TRUE, multiple search params are combined in a logical OR operation. Results will include rows that match **any** of the search criteria.  String search params use case-insensitive matching. String search params can contain `%` and '_' as SQL LIKE pattern match wildcard expressions. example=\"dan%\" will match \"danger\" and \"Danzig\" but not \"David\" example=\"D_m%\" will match \"Damage\" and \"dump\"  Integer search params can accept a single value or a comma separated list of values. The multiple values will be combined under a logical OR operation - results will match at least one of the given values.  Most search params can accept \"IS NULL\" and \"NOT NULL\" as special expressions to match or exclude (respectively) rows where the column is null.  Boolean search params accept only \"true\" and \"false\" as values.   (<sup>*</sup>) Results are always filtered to the level of information the caller is permitted to view. Looker admins can see all user details; normal users in an open system can see names of other users but no details; normal users in a closed system can only see names of other users who are members of the same group as the user.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Include only these fields in the response |  |
**page** | Option<**i64**> | Return only page N of paginated results |  |
**per_page** | Option<**i64**> | Return N rows of data per page |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**id** | Option<**i64**> | Match User Id. |  |
**first_name** | Option<**String**> | Match First name. |  |
**last_name** | Option<**String**> | Match Last name. |  |
**verified_looker_employee** | Option<**bool**> | Search for user accounts associated with Looker employees |  |
**embed_user** | Option<**bool**> | Search for only embed users |  |
**email** | Option<**String**> | Search for the user with this email address |  |
**is_disabled** | Option<**bool**> | Search for disabled user accounts |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |
**content_metadata_id** | Option<**i64**> | Search for users who have access to this content_metadata item |  |
**group_id** | Option<**i64**> | Search for users who are direct members of this group |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_names

> Vec<crate::models::User> search_users_names(pattern, fields, page, per_page, sorts, id, first_name, last_name, verified_looker_employee, email, is_disabled)
Search User Names

### Search for user accounts by name  Returns all user accounts where `first_name` OR `last_name` OR `email` field values match a pattern. The pattern can contain `%` and `_` wildcards as in SQL LIKE expressions.  Any additional search params will be combined into a logical AND expression. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pattern** | **String** | Pattern to match | [required] |
**fields** | Option<**String**> | Include only these fields in the response |  |
**page** | Option<**i64**> | Return only page N of paginated results |  |
**per_page** | Option<**i64**> | Return N rows of data per page |  |
**sorts** | Option<**String**> | Fields to sort by |  |
**id** | Option<**i64**> | Match User Id |  |
**first_name** | Option<**String**> | Match First name |  |
**last_name** | Option<**String**> | Match Last name |  |
**verified_looker_employee** | Option<**bool**> | Match Verified Looker employee |  |
**email** | Option<**String**> | Match Email Address |  |
**is_disabled** | Option<**bool**> | Include or exclude disabled accounts in the results |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_attribute_user_value

> crate::models::UserAttributeWithValue set_user_attribute_user_value(user_id, user_attribute_id, body)
Set User Attribute User Value

### Store a custom value for a user attribute in a user's account settings.  Per-user user attribute values take precedence over group or default values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**user_attribute_id** | **i64** | Id of user attribute | [required] |
**body** | [**UserAttributeWithValue**](UserAttributeWithValue.md) | New attribute value for user. | [required] |

### Return type

[**crate::models::UserAttributeWithValue**](UserAttributeWithValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_roles

> Vec<crate::models::Role> set_user_roles(user_id, body, fields)
Set User Roles

### Set roles of the user with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**body** | [**Vec<i64>**](i64.md) | array of roles ids for user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::User update_user(user_id, body, fields)
Update User

### Update information about the user with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**body** | [**User**](User.md) | User | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_credentials_email

> crate::models::CredentialsEmail update_user_credentials_email(user_id, body, fields)
Update Email/Password Credential

### Email/password login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**body** | [**CredentialsEmail**](CredentialsEmail.md) | Email/Password Credential | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsEmail**](CredentialsEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user

> crate::models::User user(user_id, fields)
Get User by Id

### Get information about the user with a specific id.  If the caller is an admin or the caller is the user being specified, then full user information will be returned. Otherwise, a minimal 'public' variant of the user information will be returned. This contains The user name and avatar url, but no sensitive information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_attribute_user_values

> Vec<crate::models::UserAttributeWithValue> user_attribute_user_values(user_id, fields, user_attribute_ids, all_values, include_unset)
Get User Attribute Values

### Get user attribute values for a given user.  Returns the values of specified user attributes (or all user attributes) for a certain user.  A value for each user attribute is searched for in the following locations, in this order:  1. in the user's account information 1. in groups that the user is a member of 1. the default value of the user attribute  If more than one group has a value defined for a user attribute, the group with the lowest rank wins.  The response will only include user attributes for which values were found. Use `include_unset=true` to include empty records for user attributes with no value.  The value of all hidden user attributes will be blank. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**user_attribute_ids** | Option<[**Vec<i64>**](i64.md)> | Specific user attributes to request. Omit or leave blank to request all user attributes. |  |
**all_values** | Option<**bool**> | If true, returns all values in the search path instead of just the first value found. Useful for debugging group precedence. |  |
**include_unset** | Option<**bool**> | If true, returns an empty record for each requested attribute that has no user, group, or default value. |  |

### Return type

[**Vec<crate::models::UserAttributeWithValue>**](UserAttributeWithValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_api3

> crate::models::CredentialsApi3 user_credentials_api3(user_id, credentials_api3_id, fields)
Get API 3 Credential

### API 3 login information for the specified user. This is for the newer API keys that can be added for any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**credentials_api3_id** | **i64** | Id of API 3 Credential | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsApi3**](CredentialsApi3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_email

> crate::models::CredentialsEmail user_credentials_email(user_id, fields)
Get Email/Password Credential

### Email/password login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsEmail**](CredentialsEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_embed

> crate::models::CredentialsEmbed user_credentials_embed(user_id, credentials_embed_id, fields)
Get Embedding Credential

### Embed login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**credentials_embed_id** | **i64** | Id of Embedding Credential | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsEmbed**](CredentialsEmbed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_google

> crate::models::CredentialsGoogle user_credentials_google(user_id, fields)
Get Google Auth Credential

### Google authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsGoogle**](CredentialsGoogle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_ldap

> crate::models::CredentialsLdap user_credentials_ldap(user_id, fields)
Get LDAP Credential

### LDAP login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsLdap**](CredentialsLDAP.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_looker_openid

> crate::models::CredentialsLookerOpenid user_credentials_looker_openid(user_id, fields)
Get Looker OpenId Credential

### Looker Openid login information for the specified user. Used by Looker Analysts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsLookerOpenid**](CredentialsLookerOpenid.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_oidc

> crate::models::CredentialsOidc user_credentials_oidc(user_id, fields)
Get OIDC Auth Credential

### OpenID Connect (OIDC) authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsOidc**](CredentialsOIDC.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_saml

> crate::models::CredentialsSaml user_credentials_saml(user_id, fields)
Get Saml Auth Credential

### Saml authentication login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsSaml**](CredentialsSaml.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_credentials_totp

> crate::models::CredentialsTotp user_credentials_totp(user_id, fields)
Get Two-Factor Credential

### Two-factor login information for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::CredentialsTotp**](CredentialsTotp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_for_credential

> crate::models::User user_for_credential(credential_type, credential_id, fields)
Get User by Credential Id

### Get information about the user with a credential of given type with specific id.  This is used to do things like find users by their embed external_user_id. Or, find the user with a given api3 client_id, etc. The 'credential_type' matches the 'type' name of the various credential types. It must be one of the values listed in the table below. The 'credential_id' is your unique Id for the user and is specific to each type of credential.  An example using the Ruby sdk might look like:  `sdk.user_for_credential('embed', 'customer-4959425')`  This table shows the supported 'Credential Type' strings. The right column is for reference; it shows which field in the given credential type is actually searched when finding a user with the supplied 'credential_id'.  | Credential Types | Id Field Matched | | ---------------- | ---------------- | | email            | email            | | google           | google_user_id   | | saml             | saml_user_id     | | oidc             | oidc_user_id     | | ldap             | ldap_id          | | api              | token            | | api3             | client_id        | | embed            | external_user_id | | looker_openid    | email            |  **NOTE**: The 'api' credential type was only used with the legacy Looker query API and is no longer supported. The credential type for API you are currently looking at is 'api3'.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_type** | **String** | Type name of credential | [required] |
**credential_id** | **String** | Id of credential | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_roles

> Vec<crate::models::Role> user_roles(user_id, fields, direct_association_only)
Get User Roles

### Get information about roles of a given user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | id of user | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**direct_association_only** | Option<**bool**> | Get only roles associated directly with the user: exclude those only associated through groups. |  |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_session

> crate::models::Session user_session(user_id, session_id, fields)
Get Web Login Session

### Web login session for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Id of user | [required] |
**session_id** | **i64** | Id of Web Login Session | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

