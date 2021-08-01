# \ConfigApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_legacy_features**](ConfigApi.md#all_legacy_features) | **GET** /legacy_features | Get All Legacy Features
[**all_locales**](ConfigApi.md#all_locales) | **GET** /locales | Get All Locales
[**all_timezones**](ConfigApi.md#all_timezones) | **GET** /timezones | Get All Timezones
[**backup_configuration**](ConfigApi.md#backup_configuration) | **GET** /backup_configuration | Get Backup Configuration
[**cloud_storage_configuration**](ConfigApi.md#cloud_storage_configuration) | **GET** /cloud_storage | Get Cloud Storage
[**create_digest_email_send**](ConfigApi.md#create_digest_email_send) | **POST** /digest_email_send | Deliver digest email contents
[**custom_welcome_email**](ConfigApi.md#custom_welcome_email) | **GET** /custom_welcome_email | Get Custom Welcome Email
[**digest_emails_enabled**](ConfigApi.md#digest_emails_enabled) | **GET** /digest_emails_enabled | Get Digest_emails
[**internal_help_resources**](ConfigApi.md#internal_help_resources) | **GET** /internal_help_resources_enabled | Get Internal Help Resources
[**internal_help_resources_content**](ConfigApi.md#internal_help_resources_content) | **GET** /internal_help_resources_content | Get Internal Help Resources Content
[**legacy_feature**](ConfigApi.md#legacy_feature) | **GET** /legacy_features/{legacy_feature_id} | Get Legacy Feature
[**update_backup_configuration**](ConfigApi.md#update_backup_configuration) | **PATCH** /backup_configuration | Update Backup Configuration
[**update_cloud_storage_configuration**](ConfigApi.md#update_cloud_storage_configuration) | **PATCH** /cloud_storage | Update Cloud Storage
[**update_custom_welcome_email**](ConfigApi.md#update_custom_welcome_email) | **PATCH** /custom_welcome_email | Update Custom Welcome Email Content
[**update_custom_welcome_email_test**](ConfigApi.md#update_custom_welcome_email_test) | **PUT** /custom_welcome_email_test | Send a test welcome email to the currently logged in user with the supplied content 
[**update_digest_emails_enabled**](ConfigApi.md#update_digest_emails_enabled) | **PATCH** /digest_emails_enabled | Update Digest_emails
[**update_internal_help_resources**](ConfigApi.md#update_internal_help_resources) | **PATCH** /internal_help_resources | Update internal help resources configuration
[**update_internal_help_resources_content**](ConfigApi.md#update_internal_help_resources_content) | **PATCH** /internal_help_resources_content | Update internal help resources content
[**update_legacy_feature**](ConfigApi.md#update_legacy_feature) | **PATCH** /legacy_features/{legacy_feature_id} | Update Legacy Feature
[**update_whitelabel_configuration**](ConfigApi.md#update_whitelabel_configuration) | **PUT** /whitelabel_configuration | Update Whitelabel configuration
[**versions**](ConfigApi.md#versions) | **GET** /versions | Get ApiVersion
[**whitelabel_configuration**](ConfigApi.md#whitelabel_configuration) | **GET** /whitelabel_configuration | Get Whitelabel configuration



## all_legacy_features

> Vec<crate::models::LegacyFeature> all_legacy_features()
Get All Legacy Features

### Get all legacy features. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LegacyFeature>**](LegacyFeature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_locales

> Vec<crate::models::Locale> all_locales()
Get All Locales

### Get a list of locales that Looker supports. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Locale>**](Locale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_timezones

> Vec<crate::models::Timezone> all_timezones()
Get All Timezones

### Get a list of timezones that Looker supports (e.g. useful for scheduling tasks). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Timezone>**](Timezone.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_configuration

> crate::models::BackupConfiguration backup_configuration()
Get Backup Configuration

### WARNING: The Looker internal database backup function has been deprecated. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BackupConfiguration**](BackupConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_storage_configuration

> crate::models::BackupConfiguration cloud_storage_configuration()
Get Cloud Storage

Get the current Cloud Storage Configuration. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BackupConfiguration**](BackupConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_digest_email_send

> crate::models::DigestEmailSend create_digest_email_send()
Deliver digest email contents

### Trigger the generation of digest email records and send them to Looker's internal system. This does not send any actual emails, it generates records containing content which may be of interest for users who have become inactive. Emails will be sent at a later time from Looker's internal system if the Digest Emails feature is enabled in settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DigestEmailSend**](DigestEmailSend.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_welcome_email

> crate::models::CustomWelcomeEmail custom_welcome_email()
Get Custom Welcome Email

### Get the current status and content of custom welcome emails 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CustomWelcomeEmail**](CustomWelcomeEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## digest_emails_enabled

> crate::models::DigestEmails digest_emails_enabled()
Get Digest_emails

### Retrieve the value for whether or not digest emails is enabled 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DigestEmails**](DigestEmails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## internal_help_resources

> crate::models::InternalHelpResources internal_help_resources()
Get Internal Help Resources

### Get and set the options for internal help resources 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InternalHelpResources**](InternalHelpResources.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## internal_help_resources_content

> crate::models::InternalHelpResourcesContent internal_help_resources_content()
Get Internal Help Resources Content

### Set the menu item name and content for internal help resources 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InternalHelpResourcesContent**](InternalHelpResourcesContent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## legacy_feature

> crate::models::LegacyFeature legacy_feature(legacy_feature_id)
Get Legacy Feature

### Get information about the legacy feature with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legacy_feature_id** | **i64** | id of legacy feature | [required] |

### Return type

[**crate::models::LegacyFeature**](LegacyFeature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_backup_configuration

> crate::models::BackupConfiguration update_backup_configuration(body)
Update Backup Configuration

### WARNING: The Looker internal database backup function has been deprecated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BackupConfiguration**](BackupConfiguration.md) | Options for Backup Configuration | [required] |

### Return type

[**crate::models::BackupConfiguration**](BackupConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_storage_configuration

> crate::models::BackupConfiguration update_cloud_storage_configuration(body)
Update Cloud Storage

Update the current Cloud Storage Configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BackupConfiguration**](BackupConfiguration.md) | Options for Cloud Storage Configuration | [required] |

### Return type

[**crate::models::BackupConfiguration**](BackupConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_welcome_email

> crate::models::CustomWelcomeEmail update_custom_welcome_email(body, send_test_welcome_email)
Update Custom Welcome Email Content

Update custom welcome email setting and values. Optionally send a test email with the new content to the currently logged in user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CustomWelcomeEmail**](CustomWelcomeEmail.md) | Custom Welcome Email setting and value to save | [required] |
**send_test_welcome_email** | Option<**bool**> | If true a test email with the content from the request will be sent to the current user after saving |  |

### Return type

[**crate::models::CustomWelcomeEmail**](CustomWelcomeEmail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_welcome_email_test

> crate::models::WelcomeEmailTest update_custom_welcome_email_test(body)
Send a test welcome email to the currently logged in user with the supplied content 

Requests to this endpoint will send a welcome email with the custom content provided in the body to the currently logged in user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WelcomeEmailTest**](WelcomeEmailTest.md) | Subject, header, and Body of the email to be sent. | [required] |

### Return type

[**crate::models::WelcomeEmailTest**](WelcomeEmailTest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_digest_emails_enabled

> crate::models::DigestEmails update_digest_emails_enabled(body)
Update Digest_emails

### Update the setting for enabling/disabling digest emails 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DigestEmails**](DigestEmails.md) | Digest_emails | [required] |

### Return type

[**crate::models::DigestEmails**](DigestEmails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_internal_help_resources

> crate::models::InternalHelpResources update_internal_help_resources(body)
Update internal help resources configuration

Update internal help resources settings 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InternalHelpResources**](InternalHelpResources.md) | Custom Welcome Email | [required] |

### Return type

[**crate::models::InternalHelpResources**](InternalHelpResources.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_internal_help_resources_content

> crate::models::InternalHelpResourcesContent update_internal_help_resources_content(body)
Update internal help resources content

Update internal help resources content 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InternalHelpResourcesContent**](InternalHelpResourcesContent.md) | Internal Help Resources Content | [required] |

### Return type

[**crate::models::InternalHelpResourcesContent**](InternalHelpResourcesContent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_legacy_feature

> crate::models::LegacyFeature update_legacy_feature(legacy_feature_id, body)
Update Legacy Feature

### Update information about the legacy feature with a specific id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legacy_feature_id** | **i64** | id of legacy feature | [required] |
**body** | [**LegacyFeature**](LegacyFeature.md) | Legacy Feature | [required] |

### Return type

[**crate::models::LegacyFeature**](LegacyFeature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_whitelabel_configuration

> crate::models::WhitelabelConfiguration update_whitelabel_configuration(body)
Update Whitelabel configuration

### Update the whitelabel configuration 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WhitelabelConfiguration**](WhitelabelConfiguration.md) | Whitelabel configuration | [required] |

### Return type

[**crate::models::WhitelabelConfiguration**](WhitelabelConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## versions

> crate::models::ApiVersion versions(fields)
Get ApiVersion

### Get information about all API versions supported by this Looker instance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ApiVersion**](ApiVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## whitelabel_configuration

> crate::models::WhitelabelConfiguration whitelabel_configuration(fields)
Get Whitelabel configuration

### This feature is enabled only by special license. ### Gets the whitelabel configuration, which includes hiding documentation links, custom favicon uploading, etc. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::WhitelabelConfiguration**](WhitelabelConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

