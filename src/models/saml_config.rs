/*
 * Looker API 3.1 Reference
 *
 * ### Authorization  The classic method of API authorization uses Looker **API3** credentials for authorization and access control. Looker admins can create API3 credentials on Looker's **Admin/Users** page.  API 4.0 adds additional ways to authenticate API requests, including OAuth and CORS requests.  For details, see [Looker API Authorization](https://looker.com/docs/r/api/authorization).   ### API Explorer  The API Explorer is a Looker-provided utility with many new and unique features for learning and using the Looker API and SDKs. It is a replacement for the 'api-docs' page currently provided on Looker instances.  For details, see the [API Explorer documentation](https://looker.com/docs/r/api/explorer).   ### Looker Language SDKs  The Looker API is a RESTful system that should be usable by any programming language capable of making HTTPS requests. SDKs for a variety of programming languages are also provided to streamline using the API. Looker has an OpenSource [sdk-codegen project](https://github.com/looker-open-source/sdk-codegen) that provides several language SDKs. Language SDKs generated by `sdk-codegen` have an Authentication manager that can automatically authenticate API requests when needed.  For details on available Looker SDKs, see [Looker API Client SDKs](https://looker.com/docs/r/api/client_sdks).   ### API Versioning  Future releases of Looker expand the latest API version release-by-release to securely expose more and more of the core power of the Looker platform to API client applications. API endpoints marked as \"beta\" may receive breaking changes without warning (but we will try to avoid doing that). Stable (non-beta) API endpoints should not receive breaking changes in future releases.  For details, see [Looker API Versioning](https://looker.com/docs/r/api/versioning).   ### Try It Out!  This section describes the existing 'api-docs' page available on Looker instances. We recommend using the [API Explorer](https://looker.com/docs/r/api/explorer) instead.  The 'api-docs' page served by the Looker instance includes 'Try It Out!' buttons for each API method. After logging in with API3 credentials, you can use the \"Try It Out!\" buttons to call the API directly from the documentation page to interactively explore API features and responses.  **NOTE**! With great power comes great responsibility: The \"Try It Out!\" button makes API calls to your live Looker instance. Be especially careful with destructive API operations such as `delete_user` or similar. There is no \"undo\" for API operations. (API Explorer's \"Run It\" feature requires a check mark before running API operations that can change data.)   ### In This Release  The following are a few examples of noteworthy items that have changed between API 3.0 and API 3.1. For more comprehensive coverage of API changes, please see the release notes for your Looker release.  ### Examples of new things added in API 3.1 (compared to API 3.0):  * [Dashboard construction](#!/3.1/Dashboard/) APIs * [Themes](#!/3.1/Theme/) and [custom color collections](#!/3.1/ColorCollection) APIs * Create and run [SQL Runner](#!/3.1/Query/run_sql_query) queries * Create and run [merged results](#!/3.1/Query/create_merge_query) queries * Create and modify [dashboard filters](#!/3.1/Dashboard/create_dashboard_filter) * Create and modify [password requirements](#!/3.1/Auth/password_config)  ### Deprecated in API 3.0  The following functions and properties have been deprecated in API 3.0.  They continue to exist and work in API 3.0 for the next several Looker releases but they have not been carried forward to API 3.1:  * Dashboard Prefetch functions * User access_filter functions * User API 1.0 credentials functions * Space.is_root and Space.is_user_root properties. Use Space.is_shared_root and Space.is_users_root instead.  ### Semantic changes in API 3.1:  * [all_looks()](#!/3.1/Look/all_looks) no longer includes soft-deleted looks, matching [all_dashboards()](#!/3.1/Dashboard/all_dashboards) behavior. You can find soft-deleted looks using [search_looks()](#!/3.1/Look/search_looks) with the `deleted` param set to True. * [all_spaces()](#!/3.1/Space/all_spaces) no longer includes duplicate items * [search_users()](#!/3.1/User/search_users) no longer accepts Y,y,1,0,N,n for Boolean params, only \"true\" and \"false\". * For greater client and network compatibility, [render_task_results](#!/3.1/RenderTask/render_task_results) now returns HTTP status **202 Accepted** instead of HTTP status **102 Processing** * [all_running_queries()](#!/3.1/Query/all_running_queries) and [kill_query](#!/3.1/Query/kill_query) functions have moved into the [Query](#!/3.1/Query/) function group.  The API Explorer can be used to [interactively compare](https://looker.com/docs/r/api/explorer#comparing_api_versions) the differences between API 3.1 and 4.0.   
 *
 * The version of the OpenAPI document: 3.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlConfig {
    /// Operations the current user is able to perform on this object
    #[serde(rename = "can", skip_serializing_if = "Option::is_none")]
    pub can: Option<::std::collections::HashMap<String, bool>>,
    /// Enable/Disable Saml authentication for the server
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Identity Provider Certificate (provided by IdP)
    #[serde(rename = "idp_cert", skip_serializing_if = "Option::is_none")]
    pub idp_cert: Option<String>,
    /// Identity Provider Url (provided by IdP)
    #[serde(rename = "idp_url", skip_serializing_if = "Option::is_none")]
    pub idp_url: Option<String>,
    /// Identity Provider Issuer (provided by IdP)
    #[serde(rename = "idp_issuer", skip_serializing_if = "Option::is_none")]
    pub idp_issuer: Option<String>,
    /// Identity Provider Audience (set in IdP config). Optional in Looker. Set this only if you want Looker to validate the audience value returned by the IdP.
    #[serde(rename = "idp_audience", skip_serializing_if = "Option::is_none")]
    pub idp_audience: Option<String>,
    /// Count of seconds of clock drift to allow when validating timestamps of assertions.
    #[serde(rename = "allowed_clock_drift", skip_serializing_if = "Option::is_none")]
    pub allowed_clock_drift: Option<i64>,
    /// Name of user record attributes used to indicate email address field
    #[serde(rename = "user_attribute_map_email", skip_serializing_if = "Option::is_none")]
    pub user_attribute_map_email: Option<String>,
    /// Name of user record attributes used to indicate first name
    #[serde(rename = "user_attribute_map_first_name", skip_serializing_if = "Option::is_none")]
    pub user_attribute_map_first_name: Option<String>,
    /// Name of user record attributes used to indicate last name
    #[serde(rename = "user_attribute_map_last_name", skip_serializing_if = "Option::is_none")]
    pub user_attribute_map_last_name: Option<String>,
    /// Merge first-time saml login to existing user account by email addresses. When a user logs in for the first time via saml this option will connect this user into their existing account by finding the account with a matching email address by testing the given types of credentials for existing users. Otherwise a new user account will be created for the user. This list (if provided) must be a comma separated list of string like 'email,ldap,google'
    #[serde(rename = "new_user_migration_types", skip_serializing_if = "Option::is_none")]
    pub new_user_migration_types: Option<String>,
    /// Allow alternate email-based login via '/login/email' for admins and for specified users with the 'login_special_email' permission. This option is useful as a fallback during ldap setup, if ldap config problems occur later, or if you need to support some users who are not in your ldap directory. Looker email/password logins are always disabled for regular users when ldap is enabled.
    #[serde(rename = "alternate_email_login_allowed", skip_serializing_if = "Option::is_none")]
    pub alternate_email_login_allowed: Option<bool>,
    /// Slug to identify configurations that are created in order to run a Saml config test
    #[serde(rename = "test_slug", skip_serializing_if = "Option::is_none")]
    pub test_slug: Option<String>,
    /// When this config was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// User id of user who last modified this config
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// (Read-only) Roles that will be applied to new users the first time they login via Saml
    #[serde(rename = "default_new_user_roles", skip_serializing_if = "Option::is_none")]
    pub default_new_user_roles: Option<Vec<crate::models::Role>>,
    /// (Read-only) Groups that will be applied to new users the first time they login via Saml
    #[serde(rename = "default_new_user_groups", skip_serializing_if = "Option::is_none")]
    pub default_new_user_groups: Option<Vec<crate::models::Group>>,
    /// (Write-Only) Array of ids of roles that will be applied to new users the first time they login via Saml
    #[serde(rename = "default_new_user_role_ids", skip_serializing_if = "Option::is_none")]
    pub default_new_user_role_ids: Option<Vec<i64>>,
    /// (Write-Only) Array of ids of groups that will be applied to new users the first time they login via Saml
    #[serde(rename = "default_new_user_group_ids", skip_serializing_if = "Option::is_none")]
    pub default_new_user_group_ids: Option<Vec<i64>>,
    /// Set user roles in Looker based on groups from Saml
    #[serde(rename = "set_roles_from_groups", skip_serializing_if = "Option::is_none")]
    pub set_roles_from_groups: Option<bool>,
    /// Name of user record attributes used to indicate groups. Used when 'groups_finder_type' is set to 'grouped_attribute_values'
    #[serde(rename = "groups_attribute", skip_serializing_if = "Option::is_none")]
    pub groups_attribute: Option<String>,
    /// (Read-only) Array of mappings between Saml Groups and Looker Roles
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::SamlGroupRead>>,
    /// (Read/Write) Array of mappings between Saml Groups and arrays of Looker Role ids
    #[serde(rename = "groups_with_role_ids", skip_serializing_if = "Option::is_none")]
    pub groups_with_role_ids: Option<Vec<crate::models::SamlGroupWrite>>,
    /// Users will not be allowed to login at all unless a role for them is found in Saml if set to true
    #[serde(rename = "auth_requires_role", skip_serializing_if = "Option::is_none")]
    pub auth_requires_role: Option<bool>,
    /// (Read-only) Array of mappings between Saml User Attributes and Looker User Attributes
    #[serde(rename = "user_attributes", skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<crate::models::SamlUserAttributeRead>>,
    /// (Read/Write) Array of mappings between Saml User Attributes and arrays of Looker User Attribute ids
    #[serde(rename = "user_attributes_with_ids", skip_serializing_if = "Option::is_none")]
    pub user_attributes_with_ids: Option<Vec<crate::models::SamlUserAttributeWrite>>,
    /// Identifier for a strategy for how Looker will find groups in the SAML response. One of ['grouped_attribute_values', 'individual_attributes']
    #[serde(rename = "groups_finder_type", skip_serializing_if = "Option::is_none")]
    pub groups_finder_type: Option<String>,
    /// Value for group attribute used to indicate membership. Used when 'groups_finder_type' is set to 'individual_attributes'
    #[serde(rename = "groups_member_value", skip_serializing_if = "Option::is_none")]
    pub groups_member_value: Option<String>,
    /// Bypass the login page when user authentication is required. Redirect to IdP immediately instead.
    #[serde(rename = "bypass_login_page", skip_serializing_if = "Option::is_none")]
    pub bypass_login_page: Option<bool>,
    /// Allow SAML auth'd users to be members of non-reflected Looker groups. If 'false', user will be removed from non-reflected groups on login.
    #[serde(rename = "allow_normal_group_membership", skip_serializing_if = "Option::is_none")]
    pub allow_normal_group_membership: Option<bool>,
    /// SAML auth'd users will inherit roles from non-reflected Looker groups.
    #[serde(rename = "allow_roles_from_normal_groups", skip_serializing_if = "Option::is_none")]
    pub allow_roles_from_normal_groups: Option<bool>,
    /// Allows roles to be directly assigned to SAML auth'd users.
    #[serde(rename = "allow_direct_roles", skip_serializing_if = "Option::is_none")]
    pub allow_direct_roles: Option<bool>,
    /// Link to get this item
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SamlConfig {
    pub fn new() -> SamlConfig {
        SamlConfig {
            can: None,
            enabled: None,
            idp_cert: None,
            idp_url: None,
            idp_issuer: None,
            idp_audience: None,
            allowed_clock_drift: None,
            user_attribute_map_email: None,
            user_attribute_map_first_name: None,
            user_attribute_map_last_name: None,
            new_user_migration_types: None,
            alternate_email_login_allowed: None,
            test_slug: None,
            modified_at: None,
            modified_by: None,
            default_new_user_roles: None,
            default_new_user_groups: None,
            default_new_user_role_ids: None,
            default_new_user_group_ids: None,
            set_roles_from_groups: None,
            groups_attribute: None,
            groups: None,
            groups_with_role_ids: None,
            auth_requires_role: None,
            user_attributes: None,
            user_attributes_with_ids: None,
            groups_finder_type: None,
            groups_member_value: None,
            bypass_login_page: None,
            allow_normal_group_membership: None,
            allow_roles_from_normal_groups: None,
            allow_direct_roles: None,
            url: None,
        }
    }
}


