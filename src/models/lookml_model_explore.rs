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
pub struct LookmlModelExplore {
    /// Fully qualified explore name (model name plus explore name)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Explore name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Scopes
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Can Total
    #[serde(rename = "can_total", skip_serializing_if = "Option::is_none")]
    pub can_total: Option<bool>,
    /// Can Save
    #[serde(rename = "can_save", skip_serializing_if = "Option::is_none")]
    pub can_save: Option<bool>,
    /// Can Explain
    #[serde(rename = "can_explain", skip_serializing_if = "Option::is_none")]
    pub can_explain: Option<bool>,
    /// Can pivot in the DB
    #[serde(rename = "can_pivot_in_db", skip_serializing_if = "Option::is_none")]
    pub can_pivot_in_db: Option<bool>,
    /// Can use subtotals
    #[serde(rename = "can_subtotal", skip_serializing_if = "Option::is_none")]
    pub can_subtotal: Option<bool>,
    /// Has timezone support
    #[serde(
        rename = "has_timezone_support",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_timezone_support: Option<bool>,
    /// Cost estimates supported
    #[serde(
        rename = "supports_cost_estimate",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_cost_estimate: Option<bool>,
    /// Connection name
    #[serde(rename = "connection_name", skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// How nulls are sorted, possible values are \"low\", \"high\", \"first\" and \"last\"
    #[serde(
        rename = "null_sort_treatment",
        skip_serializing_if = "Option::is_none"
    )]
    pub null_sort_treatment: Option<String>,
    /// List of model source files
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Primary source_file file
    #[serde(rename = "source_file", skip_serializing_if = "Option::is_none")]
    pub source_file: Option<String>,
    /// Name of project
    #[serde(rename = "project_name", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// Name of model
    #[serde(rename = "model_name", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// Name of view
    #[serde(rename = "view_name", skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// Is hidden
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// A sql_table_name expression that defines what sql table the view/explore maps onto. Example: \"prod_orders2 AS orders\" in a view named orders.
    #[serde(rename = "sql_table_name", skip_serializing_if = "Option::is_none")]
    pub sql_table_name: Option<String>,
    /// (DEPRECATED) Array of access filter field names
    #[serde(
        rename = "access_filter_fields",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_filter_fields: Option<Vec<String>>,
    /// Access filters
    #[serde(rename = "access_filters", skip_serializing_if = "Option::is_none")]
    pub access_filters: Option<Vec<crate::models::LookmlModelExploreAccessFilter>>,
    /// Aliases
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<crate::models::LookmlModelExploreAlias>>,
    /// Always filter
    #[serde(rename = "always_filter", skip_serializing_if = "Option::is_none")]
    pub always_filter: Option<Vec<crate::models::LookmlModelExploreAlwaysFilter>>,
    /// Conditionally filter
    #[serde(
        rename = "conditionally_filter",
        skip_serializing_if = "Option::is_none"
    )]
    pub conditionally_filter: Option<Vec<crate::models::LookmlModelExploreConditionallyFilter>>,
    /// Array of index fields
    #[serde(rename = "index_fields", skip_serializing_if = "Option::is_none")]
    pub index_fields: Option<Vec<String>>,
    /// Sets
    #[serde(rename = "sets", skip_serializing_if = "Option::is_none")]
    pub sets: Option<Vec<crate::models::LookmlModelExploreSet>>,
    /// An array of arbitrary string tags provided in the model for this explore.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Errors
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::LookmlModelExploreError>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<crate::models::LookmlModelExploreFieldset>>,
    /// Views joined into this explore
    #[serde(rename = "joins", skip_serializing_if = "Option::is_none")]
    pub joins: Option<Vec<crate::models::LookmlModelExploreJoins>>,
    /// Label used to group explores in the navigation menus
    #[serde(rename = "group_label", skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    /// An array of items describing which custom measure types are supported for creating a custom measure 'based_on' each possible dimension type.
    #[serde(
        rename = "supported_measure_types",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_measure_types: Option<Vec<crate::models::LookmlModelExploreSupportedMeasureType>>,
}

impl LookmlModelExplore {
    pub fn new() -> LookmlModelExplore {
        LookmlModelExplore {
            id: None,
            name: None,
            description: None,
            label: None,
            scopes: None,
            can_total: None,
            can_save: None,
            can_explain: None,
            can_pivot_in_db: None,
            can_subtotal: None,
            has_timezone_support: None,
            supports_cost_estimate: None,
            connection_name: None,
            null_sort_treatment: None,
            files: None,
            source_file: None,
            project_name: None,
            model_name: None,
            view_name: None,
            hidden: None,
            sql_table_name: None,
            access_filter_fields: None,
            access_filters: None,
            aliases: None,
            always_filter: None,
            conditionally_filter: None,
            index_fields: None,
            sets: None,
            tags: None,
            errors: None,
            fields: None,
            joins: None,
            group_label: None,
            supported_measure_types: None,
        }
    }
}
