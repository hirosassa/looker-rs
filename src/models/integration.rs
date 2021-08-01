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
pub struct Integration {
    /// Operations the current user is able to perform on this object
    #[serde(rename = "can", skip_serializing_if = "Option::is_none")]
    pub can: Option<::std::collections::HashMap<String, bool>>,
    /// ID of the integration.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the integration hub.
    #[serde(rename = "integration_hub_id", skip_serializing_if = "Option::is_none")]
    pub integration_hub_id: Option<i64>,
    /// Label for the integration.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Description of the integration.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the integration is available to users.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Array of params for the integration.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<crate::models::IntegrationParam>>,
    /// A list of data formats the integration supports. If unspecified, the default is all data formats. Valid values are: \"txt\", \"csv\", \"inline_json\", \"json\", \"json_label\", \"json_detail\", \"json_detail_lite_stream\", \"xlsx\", \"html\", \"wysiwyg_pdf\", \"assembled_pdf\", \"wysiwyg_png\", \"csv_zip\".
    #[serde(rename = "supported_formats", skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<Vec<String>>,
    /// A list of action types the integration supports. Valid values are: \"cell\", \"query\", \"dashboard\".
    #[serde(rename = "supported_action_types", skip_serializing_if = "Option::is_none")]
    pub supported_action_types: Option<Vec<String>>,
    /// A list of formatting options the integration supports. If unspecified, defaults to all formats. Valid values are: \"formatted\", \"unformatted\".
    #[serde(rename = "supported_formattings", skip_serializing_if = "Option::is_none")]
    pub supported_formattings: Option<Vec<String>>,
    /// A list of visualization formatting options the integration supports. If unspecified, defaults to all formats. Valid values are: \"apply\", \"noapply\".
    #[serde(rename = "supported_visualization_formattings", skip_serializing_if = "Option::is_none")]
    pub supported_visualization_formattings: Option<Vec<String>>,
    /// A list of all the download mechanisms the integration supports. The order of values is not significant: Looker will select the most appropriate supported download mechanism for a given query. The integration must ensure it can handle any of the mechanisms it claims to support. If unspecified, this defaults to all download setting values. Valid values are: \"push\", \"url\".
    #[serde(rename = "supported_download_settings", skip_serializing_if = "Option::is_none")]
    pub supported_download_settings: Option<Vec<String>>,
    /// URL to an icon for the integration.
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Whether the integration uses oauth.
    #[serde(rename = "uses_oauth", skip_serializing_if = "Option::is_none")]
    pub uses_oauth: Option<bool>,
    /// A list of descriptions of required fields that this integration is compatible with. If there are multiple entries in this list, the integration requires more than one field. If unspecified, no fields will be required.
    #[serde(rename = "required_fields", skip_serializing_if = "Option::is_none")]
    pub required_fields: Option<Vec<crate::models::IntegrationRequiredField>>,
    /// Whether the integration uses delegate oauth, which allows federation between an integration installation scope specific entity (like org, group, and team, etc.) and Looker.
    #[serde(rename = "delegate_oauth", skip_serializing_if = "Option::is_none")]
    pub delegate_oauth: Option<bool>,
    /// Whether the integration is available to users.
    #[serde(rename = "installed_delegate_oauth_targets", skip_serializing_if = "Option::is_none")]
    pub installed_delegate_oauth_targets: Option<Vec<i64>>,
}

impl Integration {
    pub fn new() -> Integration {
        Integration {
            can: None,
            id: None,
            integration_hub_id: None,
            label: None,
            description: None,
            enabled: None,
            params: None,
            supported_formats: None,
            supported_action_types: None,
            supported_formattings: None,
            supported_visualization_formattings: None,
            supported_download_settings: None,
            icon_url: None,
            uses_oauth: None,
            required_fields: None,
            delegate_oauth: None,
            installed_delegate_oauth_targets: None,
        }
    }
}

