# \AuthApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_user_login_lockouts**](AuthApi.md#all_user_login_lockouts) | **GET** /user_login_lockouts | Get All User Login Lockouts
[**create_oidc_test_config**](AuthApi.md#create_oidc_test_config) | **POST** /oidc_test_configs | Create OIDC Test Configuration
[**create_saml_test_config**](AuthApi.md#create_saml_test_config) | **POST** /saml_test_configs | Create SAML Test Configuration
[**create_sso_embed_url**](AuthApi.md#create_sso_embed_url) | **POST** /embed/sso_url | Create SSO Embed Url
[**delete_oidc_test_config**](AuthApi.md#delete_oidc_test_config) | **DELETE** /oidc_test_configs/{test_slug} | Delete OIDC Test Configuration
[**delete_saml_test_config**](AuthApi.md#delete_saml_test_config) | **DELETE** /saml_test_configs/{test_slug} | Delete SAML Test Configuration
[**delete_user_login_lockout**](AuthApi.md#delete_user_login_lockout) | **DELETE** /user_login_lockout/{key} | Delete User Login Lockout
[**fetch_and_parse_saml_idp_metadata**](AuthApi.md#fetch_and_parse_saml_idp_metadata) | **POST** /fetch_and_parse_saml_idp_metadata | Parse SAML IdP Url
[**force_password_reset_at_next_login_for_all_users**](AuthApi.md#force_password_reset_at_next_login_for_all_users) | **PUT** /password_config/force_password_reset_at_next_login_for_all_users | Force password reset
[**ldap_config**](AuthApi.md#ldap_config) | **GET** /ldap_config | Get LDAP Configuration
[**oidc_config**](AuthApi.md#oidc_config) | **GET** /oidc_config | Get OIDC Configuration
[**oidc_test_config**](AuthApi.md#oidc_test_config) | **GET** /oidc_test_configs/{test_slug} | Get OIDC Test Configuration
[**parse_saml_idp_metadata**](AuthApi.md#parse_saml_idp_metadata) | **POST** /parse_saml_idp_metadata | Parse SAML IdP XML
[**password_config**](AuthApi.md#password_config) | **GET** /password_config | Get Password Config
[**saml_config**](AuthApi.md#saml_config) | **GET** /saml_config | Get SAML Configuration
[**saml_test_config**](AuthApi.md#saml_test_config) | **GET** /saml_test_configs/{test_slug} | Get SAML Test Configuration
[**search_user_login_lockouts**](AuthApi.md#search_user_login_lockouts) | **GET** /user_login_lockouts/search | Search User Login Lockouts
[**session_config**](AuthApi.md#session_config) | **GET** /session_config | Get Session Config
[**test_ldap_config_auth**](AuthApi.md#test_ldap_config_auth) | **PUT** /ldap_config/test_auth | Test LDAP Auth
[**test_ldap_config_connection**](AuthApi.md#test_ldap_config_connection) | **PUT** /ldap_config/test_connection | Test LDAP Connection
[**test_ldap_config_user_auth**](AuthApi.md#test_ldap_config_user_auth) | **PUT** /ldap_config/test_user_auth | Test LDAP User Auth
[**test_ldap_config_user_info**](AuthApi.md#test_ldap_config_user_info) | **PUT** /ldap_config/test_user_info | Test LDAP User Info
[**update_ldap_config**](AuthApi.md#update_ldap_config) | **PATCH** /ldap_config | Update LDAP Configuration
[**update_oidc_config**](AuthApi.md#update_oidc_config) | **PATCH** /oidc_config | Update OIDC Configuration
[**update_password_config**](AuthApi.md#update_password_config) | **PATCH** /password_config | Update Password Config
[**update_saml_config**](AuthApi.md#update_saml_config) | **PATCH** /saml_config | Update SAML Configuration
[**update_session_config**](AuthApi.md#update_session_config) | **PATCH** /session_config | Update Session Config



## all_user_login_lockouts

> Vec<crate::models::UserLoginLockout> all_user_login_lockouts(fields)
Get All User Login Lockouts

### Get currently locked-out users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Include only these fields in the response |  |

### Return type

[**Vec<crate::models::UserLoginLockout>**](UserLoginLockout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_oidc_test_config

> crate::models::OidcConfig create_oidc_test_config(body)
Create OIDC Test Configuration

### Create a OIDC test configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OidcConfig**](OidcConfig.md) | OIDC test config | [required] |

### Return type

[**crate::models::OidcConfig**](OIDCConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_saml_test_config

> crate::models::SamlConfig create_saml_test_config(body)
Create SAML Test Configuration

### Create a SAML test configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SamlConfig**](SamlConfig.md) | SAML test config | [required] |

### Return type

[**crate::models::SamlConfig**](SamlConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sso_embed_url

> crate::models::EmbedUrlResponse create_sso_embed_url(body)
Create SSO Embed Url

### Create SSO Embed URL  Creates an SSO embed URL and cryptographically signs it with an embed secret. This signed URL can then be used to instantiate a Looker embed session in a PBL web application. Do not make any modifications to this URL - any change may invalidate the signature and cause the URL to fail to load a Looker embed session.  A signed SSO embed URL can only be used once. After it has been used to request a page from the Looker server, the URL is invalid. Future requests using the same URL will fail. This is to prevent 'replay attacks'.  The `target_url` property must be a complete URL of a Looker UI page - scheme, hostname, path and query params. To load a dashboard with id 56 and with a filter of `Date=1 years`, the looker URL would look like `https:/myname.looker.com/dashboards/56?Date=1%20years`. The best way to obtain this target_url is to navigate to the desired Looker page in your web browser, copy the URL shown in the browser address bar and paste it into the `target_url` property as a quoted string value in this API request.  Permissions for the embed user are defined by the groups in which the embed user is a member (group_ids property) and the lists of models and permissions assigned to the embed user. At a minimum, you must provide values for either the group_ids property, or both the models and permissions properties. These properties are additive; an embed user can be a member of certain groups AND be granted access to models and permissions.  The embed user's access is the union of permissions granted by the group_ids, models, and permissions properties.  This function does not strictly require all group_ids, user attribute names, or model names to exist at the moment the SSO embed url is created. Unknown group_id, user attribute names or model names will be passed through to the output URL. To diagnose potential problems with an SSO embed URL, you can copy the signed URL into the Embed URI Validator text box in `<your looker instance>/admin/embed`.  The `secret_id` parameter is optional. If specified, its value must be the id of an active secret defined in the Looker instance. if not specified, the URL will be signed using the newest active secret defined in the Looker instance.  #### Security Note Protect this signed URL as you would an access token or password credentials - do not write it to disk, do not pass it to a third party, and only pass it through a secure HTTPS encrypted transport. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EmbedSsoParams**](EmbedSsoParams.md) | SSO parameters | [required] |

### Return type

[**crate::models::EmbedUrlResponse**](EmbedUrlResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_oidc_test_config

> String delete_oidc_test_config(test_slug)
Delete OIDC Test Configuration

### Delete a OIDC test configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_slug** | **String** | Slug of test config | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saml_test_config

> String delete_saml_test_config(test_slug)
Delete SAML Test Configuration

### Delete a SAML test configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_slug** | **String** | Slug of test config | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_login_lockout

> String delete_user_login_lockout(key)
Delete User Login Lockout

### Removes login lockout for the associated user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key associated with the locked user | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_and_parse_saml_idp_metadata

> crate::models::SamlMetadataParseResult fetch_and_parse_saml_idp_metadata(body)
Parse SAML IdP Url

### Fetch the given url and parse it as a SAML IdP metadata document and return the result. Note that this requires that the url be public or at least at a location where the Looker instance can fetch it without requiring any special authentication. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | SAML IdP metadata public url | [required] |

### Return type

[**crate::models::SamlMetadataParseResult**](SamlMetadataParseResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## force_password_reset_at_next_login_for_all_users

> String force_password_reset_at_next_login_for_all_users()
Force password reset

### Force all credentials_email users to reset their login passwords upon their next login. 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ldap_config

> crate::models::LdapConfig ldap_config()
Get LDAP Configuration

### Get the LDAP configuration.  Looker can be optionally configured to authenticate users against an Active Directory or other LDAP directory server. LDAP setup requires coordination with an administrator of that directory server.  Only Looker administrators can read and update the LDAP configuration.  Configuring LDAP impacts authentication for all users. This configuration should be done carefully.  Looker maintains a single LDAP configuration. It can be read and updated.       Updates only succeed if the new state will be valid (in the sense that all required fields are populated);       it is up to you to ensure that the configuration is appropriate and correct).  LDAP is enabled or disabled for Looker using the **enabled** field.  Looker will never return an **auth_password** field. That value can be set, but never retrieved.  See the [Looker LDAP docs](https://www.looker.com/docs/r/api/ldap_setup) for additional information. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LdapConfig**](LDAPConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_config

> crate::models::OidcConfig oidc_config()
Get OIDC Configuration

### Get the OIDC configuration.  Looker can be optionally configured to authenticate users against an OpenID Connect (OIDC) authentication server. OIDC setup requires coordination with an administrator of that server.  Only Looker administrators can read and update the OIDC configuration.  Configuring OIDC impacts authentication for all users. This configuration should be done carefully.  Looker maintains a single OIDC configuation. It can be read and updated.       Updates only succeed if the new state will be valid (in the sense that all required fields are populated);       it is up to you to ensure that the configuration is appropriate and correct).  OIDC is enabled or disabled for Looker using the **enabled** field. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OidcConfig**](OIDCConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_test_config

> crate::models::OidcConfig oidc_test_config(test_slug)
Get OIDC Test Configuration

### Get a OIDC test configuration by test_slug. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_slug** | **String** | Slug of test config | [required] |

### Return type

[**crate::models::OidcConfig**](OIDCConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parse_saml_idp_metadata

> crate::models::SamlMetadataParseResult parse_saml_idp_metadata(body)
Parse SAML IdP XML

### Parse the given xml as a SAML IdP metadata document and return the result. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | SAML IdP metadata xml | [required] |

### Return type

[**crate::models::SamlMetadataParseResult**](SamlMetadataParseResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_config

> crate::models::PasswordConfig password_config()
Get Password Config

### Get password config. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PasswordConfig**](PasswordConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_config

> crate::models::SamlConfig saml_config()
Get SAML Configuration

### Get the SAML configuration.  Looker can be optionally configured to authenticate users against a SAML authentication server. SAML setup requires coordination with an administrator of that server.  Only Looker administrators can read and update the SAML configuration.  Configuring SAML impacts authentication for all users. This configuration should be done carefully.  Looker maintains a single SAML configuation. It can be read and updated.       Updates only succeed if the new state will be valid (in the sense that all required fields are populated);       it is up to you to ensure that the configuration is appropriate and correct).  SAML is enabled or disabled for Looker using the **enabled** field. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SamlConfig**](SamlConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_test_config

> crate::models::SamlConfig saml_test_config(test_slug)
Get SAML Test Configuration

### Get a SAML test configuration by test_slug. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_slug** | **String** | Slug of test config | [required] |

### Return type

[**crate::models::SamlConfig**](SamlConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_user_login_lockouts

> Vec<crate::models::UserLoginLockout> search_user_login_lockouts(fields, page, per_page, sorts, auth_type, full_name, email, remote_id, filter_or)
Search User Login Lockouts

### Search currently locked-out users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Include only these fields in the response |  |
**page** | Option<**i64**> | Return only page N of paginated results |  |
**per_page** | Option<**i64**> | Return N rows of data per page |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**auth_type** | Option<**String**> | Auth type user is locked out for (email, ldap, totp, api) |  |
**full_name** | Option<**String**> | Match name |  |
**email** | Option<**String**> | Match email |  |
**remote_id** | Option<**String**> | Match remote LDAP ID |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |

### Return type

[**Vec<crate::models::UserLoginLockout>**](UserLoginLockout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_config

> crate::models::SessionConfig session_config()
Get Session Config

### Get session config. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SessionConfig**](SessionConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_ldap_config_auth

> crate::models::LdapConfigTestResult test_ldap_config_auth(body)
Test LDAP Auth

### Test the connection authentication settings for an LDAP configuration.  This tests that the connection is possible and that a 'server' account to be used by Looker can       authenticate to the LDAP server given connection and authentication information.  **connection_host**, **connection_port**, and **auth_username**, are required.       **connection_tls** and **auth_password** are optional.  Example: ```json {   \"connection_host\": \"ldap.example.com\",   \"connection_port\": \"636\",   \"connection_tls\": true,   \"auth_username\": \"cn=looker,dc=example,dc=com\",   \"auth_password\": \"secret\" } ```  Looker will never return an **auth_password**. If this request omits the **auth_password** field, then       the **auth_password** value from the active config (if present) will be used for the test.  The active LDAP settings are not modified.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LdapConfig**](LdapConfig.md) | LDAP Config | [required] |

### Return type

[**crate::models::LdapConfigTestResult**](LDAPConfigTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_ldap_config_connection

> crate::models::LdapConfigTestResult test_ldap_config_connection(body)
Test LDAP Connection

### Test the connection settings for an LDAP configuration.  This tests that the connection is possible given a connection_host and connection_port.  **connection_host** and **connection_port** are required. **connection_tls** is optional.  Example: ```json {   \"connection_host\": \"ldap.example.com\",   \"connection_port\": \"636\",   \"connection_tls\": true } ```  No authentication to the LDAP server is attempted.  The active LDAP settings are not modified. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LdapConfig**](LdapConfig.md) | LDAP Config | [required] |

### Return type

[**crate::models::LdapConfigTestResult**](LDAPConfigTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_ldap_config_user_auth

> crate::models::LdapConfigTestResult test_ldap_config_user_auth(body)
Test LDAP User Auth

### Test the user authentication settings for an LDAP configuration.  This test accepts a full LDAP configuration along with a username/password pair and attempts to       authenticate the user with the LDAP server. The configuration is validated before attempting the       authentication.  Looker will never return an **auth_password**. If this request omits the **auth_password** field, then       the **auth_password** value from the active config (if present) will be used for the test.  **test_ldap_user** and **test_ldap_password** are required.  The active LDAP settings are not modified.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LdapConfig**](LdapConfig.md) | LDAP Config | [required] |

### Return type

[**crate::models::LdapConfigTestResult**](LDAPConfigTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_ldap_config_user_info

> crate::models::LdapConfigTestResult test_ldap_config_user_info(body)
Test LDAP User Info

### Test the user authentication settings for an LDAP configuration without authenticating the user.  This test will let you easily test the mapping for user properties and roles for any user without      needing to authenticate as that user.  This test accepts a full LDAP configuration along with a username and attempts to find the full info      for the user from the LDAP server without actually authenticating the user. So, user password is not      required.The configuration is validated before attempting to contact the server.  **test_ldap_user** is required.  The active LDAP settings are not modified.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LdapConfig**](LdapConfig.md) | LDAP Config | [required] |

### Return type

[**crate::models::LdapConfigTestResult**](LDAPConfigTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ldap_config

> crate::models::LdapConfig update_ldap_config(body)
Update LDAP Configuration

### Update the LDAP configuration.  Configuring LDAP impacts authentication for all users. This configuration should be done carefully.  Only Looker administrators can read and update the LDAP configuration.  LDAP is enabled or disabled for Looker using the **enabled** field.  It is **highly** recommended that any LDAP setting changes be tested using the APIs below before being set globally.  See the [Looker LDAP docs](https://www.looker.com/docs/r/api/ldap_setup) for additional information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LdapConfig**](LdapConfig.md) | LDAP Config | [required] |

### Return type

[**crate::models::LdapConfig**](LDAPConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_oidc_config

> crate::models::OidcConfig update_oidc_config(body)
Update OIDC Configuration

### Update the OIDC configuration.  Configuring OIDC impacts authentication for all users. This configuration should be done carefully.  Only Looker administrators can read and update the OIDC configuration.  OIDC is enabled or disabled for Looker using the **enabled** field.  It is **highly** recommended that any OIDC setting changes be tested using the APIs below before being set globally. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OidcConfig**](OidcConfig.md) | OIDC Config | [required] |

### Return type

[**crate::models::OidcConfig**](OIDCConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_password_config

> crate::models::PasswordConfig update_password_config(body)
Update Password Config

### Update password config. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PasswordConfig**](PasswordConfig.md) | Password Config | [required] |

### Return type

[**crate::models::PasswordConfig**](PasswordConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saml_config

> crate::models::SamlConfig update_saml_config(body)
Update SAML Configuration

### Update the SAML configuration.  Configuring SAML impacts authentication for all users. This configuration should be done carefully.  Only Looker administrators can read and update the SAML configuration.  SAML is enabled or disabled for Looker using the **enabled** field.  It is **highly** recommended that any SAML setting changes be tested using the APIs below before being set globally. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SamlConfig**](SamlConfig.md) | SAML Config | [required] |

### Return type

[**crate::models::SamlConfig**](SamlConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_session_config

> crate::models::SessionConfig update_session_config(body)
Update Session Config

### Update session config. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SessionConfig**](SessionConfig.md) | Session Config | [required] |

### Return type

[**crate::models::SessionConfig**](SessionConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

