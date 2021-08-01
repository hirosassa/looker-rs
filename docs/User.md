# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**avatar_url** | Option<**String**> | URL for the avatar image (may be generic) | [optional][readonly]
**avatar_url_without_sizing** | Option<**String**> | URL for the avatar image (may be generic), does not specify size | [optional][readonly]
**credentials_api3** | Option<[**Vec<crate::models::CredentialsApi3>**](CredentialsApi3.md)> | API 3 credentials | [optional][readonly]
**credentials_email** | Option<[**crate::models::CredentialsEmail**](CredentialsEmail.md)> |  | [optional]
**credentials_embed** | Option<[**Vec<crate::models::CredentialsEmbed>**](CredentialsEmbed.md)> | Embed credentials | [optional][readonly]
**credentials_google** | Option<[**crate::models::CredentialsGoogle**](CredentialsGoogle.md)> |  | [optional]
**credentials_ldap** | Option<[**crate::models::CredentialsLdap**](CredentialsLDAP.md)> |  | [optional]
**credentials_looker_openid** | Option<[**crate::models::CredentialsLookerOpenid**](CredentialsLookerOpenid.md)> |  | [optional]
**credentials_oidc** | Option<[**crate::models::CredentialsOidc**](CredentialsOIDC.md)> |  | [optional]
**credentials_saml** | Option<[**crate::models::CredentialsSaml**](CredentialsSaml.md)> |  | [optional]
**credentials_totp** | Option<[**crate::models::CredentialsTotp**](CredentialsTotp.md)> |  | [optional]
**display_name** | Option<**String**> | Full name for display (available only if both first_name and last_name are set) | [optional][readonly]
**email** | Option<**String**> | EMail address | [optional][readonly]
**embed_group_space_id** | Option<**i64**> | (Embed only) ID of user's group space based on the external_group_id optionally specified during embed user login | [optional][readonly]
**first_name** | Option<**String**> | First name | [optional]
**group_ids** | Option<**Vec<i64>**> | Array of ids of the groups for this user | [optional][readonly]
**home_space_id** | Option<**String**> | ID string for user's home space | [optional]
**home_folder_id** | Option<**String**> | ID string for user's home folder | [optional]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**is_disabled** | Option<**bool**> | Account has been disabled | [optional]
**last_name** | Option<**String**> | Last name | [optional]
**locale** | Option<**String**> | User's preferred locale. User locale takes precedence over Looker's system-wide default locale. Locale determines language of display strings and date and numeric formatting in API responses. Locale string must be a 2 letter language code or a combination of language code and region code: 'en' or 'en-US', for example. | [optional]
**looker_versions** | Option<**Vec<String>**> | Array of strings representing the Looker versions that this user has used (this only goes back as far as '3.54.0') | [optional][readonly]
**models_dir_validated** | Option<**bool**> | User's dev workspace has been checked for presence of applicable production projects | [optional]
**personal_space_id** | Option<**i64**> | ID of user's personal space | [optional][readonly]
**personal_folder_id** | Option<**i64**> | ID of user's personal folder | [optional][readonly]
**presumed_looker_employee** | Option<**bool**> | User is identified as an employee of Looker | [optional][readonly]
**role_ids** | Option<**Vec<i64>**> | Array of ids of the roles for this user | [optional][readonly]
**sessions** | Option<[**Vec<crate::models::Session>**](Session.md)> | Active sessions | [optional][readonly]
**ui_state** | Option<**::std::collections::HashMap<String, String>**> | Per user dictionary of undocumented state information owned by the Looker UI. | [optional]
**verified_looker_employee** | Option<**bool**> | User is identified as an employee of Looker who has been verified via Looker corporate authentication | [optional][readonly]
**roles_externally_managed** | Option<**bool**> | User's roles are managed by an external directory like SAML or LDAP and can not be changed directly. | [optional][readonly]
**allow_direct_roles** | Option<**bool**> | User can be directly assigned a role. | [optional][readonly]
**allow_normal_group_membership** | Option<**bool**> | User can be a direct member of a normal Looker group. | [optional][readonly]
**allow_roles_from_normal_groups** | Option<**bool**> | User can inherit roles from a normal Looker group. | [optional][readonly]
**url** | Option<**String**> | Link to get this item | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


