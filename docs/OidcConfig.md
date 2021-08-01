# OidcConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**alternate_email_login_allowed** | Option<**bool**> | Allow alternate email-based login via '/login/email' for admins and for specified users with the 'login_special_email' permission. This option is useful as a fallback during ldap setup, if ldap config problems occur later, or if you need to support some users who are not in your ldap directory. Looker email/password logins are always disabled for regular users when ldap is enabled. | [optional]
**audience** | Option<**String**> | OpenID Provider Audience | [optional]
**auth_requires_role** | Option<**bool**> | Users will not be allowed to login at all unless a role for them is found in OIDC if set to true | [optional]
**authorization_endpoint** | Option<**String**> | OpenID Provider Authorization Url | [optional]
**default_new_user_group_ids** | Option<**Vec<i64>**> | (Write-Only) Array of ids of groups that will be applied to new users the first time they login via OIDC | [optional]
**default_new_user_groups** | Option<[**Vec<crate::models::Group>**](Group.md)> | (Read-only) Groups that will be applied to new users the first time they login via OIDC | [optional][readonly]
**default_new_user_role_ids** | Option<**Vec<i64>**> | (Write-Only) Array of ids of roles that will be applied to new users the first time they login via OIDC | [optional]
**default_new_user_roles** | Option<[**Vec<crate::models::Role>**](Role.md)> | (Read-only) Roles that will be applied to new users the first time they login via OIDC | [optional][readonly]
**enabled** | Option<**bool**> | Enable/Disable OIDC authentication for the server | [optional]
**groups** | Option<[**Vec<crate::models::OidcGroupRead>**](OIDCGroupRead.md)> | (Read-only) Array of mappings between OIDC Groups and Looker Roles | [optional][readonly]
**groups_attribute** | Option<**String**> | Name of user record attributes used to indicate groups. Used when 'groups_finder_type' is set to 'grouped_attribute_values' | [optional]
**groups_with_role_ids** | Option<[**Vec<crate::models::OidcGroupWrite>**](OIDCGroupWrite.md)> | (Read/Write) Array of mappings between OIDC Groups and arrays of Looker Role ids | [optional]
**identifier** | Option<**String**> | Relying Party Identifier (provided by OpenID Provider) | [optional]
**issuer** | Option<**String**> | OpenID Provider Issuer | [optional]
**modified_at** | Option<**String**> | When this config was last modified | [optional][readonly]
**modified_by** | Option<**i64**> | User id of user who last modified this config | [optional][readonly]
**new_user_migration_types** | Option<**String**> | Merge first-time oidc login to existing user account by email addresses. When a user logs in for the first time via oidc this option will connect this user into their existing account by finding the account with a matching email address by testing the given types of credentials for existing users. Otherwise a new user account will be created for the user. This list (if provided) must be a comma separated list of string like 'email,ldap,google' | [optional]
**scopes** | Option<**Vec<String>**> | Array of scopes to request. | [optional]
**secret** | Option<**String**> | (Write-Only) Relying Party Secret (provided by OpenID Provider) | [optional]
**set_roles_from_groups** | Option<**bool**> | Set user roles in Looker based on groups from OIDC | [optional]
**test_slug** | Option<**String**> | Slug to identify configurations that are created in order to run a OIDC config test | [optional][readonly]
**token_endpoint** | Option<**String**> | OpenID Provider Token Url | [optional]
**user_attribute_map_email** | Option<**String**> | Name of user record attributes used to indicate email address field | [optional]
**user_attribute_map_first_name** | Option<**String**> | Name of user record attributes used to indicate first name | [optional]
**user_attribute_map_last_name** | Option<**String**> | Name of user record attributes used to indicate last name | [optional]
**user_attributes** | Option<[**Vec<crate::models::OidcUserAttributeRead>**](OIDCUserAttributeRead.md)> | (Read-only) Array of mappings between OIDC User Attributes and Looker User Attributes | [optional][readonly]
**user_attributes_with_ids** | Option<[**Vec<crate::models::OidcUserAttributeWrite>**](OIDCUserAttributeWrite.md)> | (Read/Write) Array of mappings between OIDC User Attributes and arrays of Looker User Attribute ids | [optional]
**userinfo_endpoint** | Option<**String**> | OpenID Provider User Information Url | [optional]
**allow_normal_group_membership** | Option<**bool**> | Allow OIDC auth'd users to be members of non-reflected Looker groups. If 'false', user will be removed from non-reflected groups on login. | [optional]
**allow_roles_from_normal_groups** | Option<**bool**> | OIDC auth'd users will inherit roles from non-reflected Looker groups. | [optional]
**allow_direct_roles** | Option<**bool**> | Allows roles to be directly assigned to OIDC auth'd users. | [optional]
**url** | Option<**String**> | Link to get this item | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


