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
pub struct RunningQueries {
    /// Operations the current user is able to perform on this object
    #[serde(rename = "can", skip_serializing_if = "Option::is_none")]
    pub can: Option<::std::collections::HashMap<String, bool>>,
    /// Unique Id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::UserPublic>>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::models::Query>>,
    #[serde(rename = "sql_query", skip_serializing_if = "Option::is_none")]
    pub sql_query: Option<Box<crate::models::SqlQuery>>,
    #[serde(rename = "look", skip_serializing_if = "Option::is_none")]
    pub look: Option<Box<crate::models::LookBasic>>,
    /// Date/Time Query was initiated
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date/Time Query was completed
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Query Id
    #[serde(rename = "query_id", skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    /// Source (look, dashboard, queryrunner, explore, etc.)
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Node Id
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Slug
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// ID of a Query Task
    #[serde(rename = "query_task_id", skip_serializing_if = "Option::is_none")]
    pub query_task_id: Option<String>,
    /// Cache Key
    #[serde(rename = "cache_key", skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    /// Connection
    #[serde(rename = "connection_name", skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// Dialect
    #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    /// Connection ID
    #[serde(rename = "connection_id", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// Additional Information(Error message or verbose status)
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status description
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Number of seconds elapsed running the Query
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<f64>,
    /// SQL text of the query as run
    #[serde(rename = "sql", skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
}

impl RunningQueries {
    pub fn new() -> RunningQueries {
        RunningQueries {
            can: None,
            id: None,
            user: None,
            query: None,
            sql_query: None,
            look: None,
            created_at: None,
            completed_at: None,
            query_id: None,
            source: None,
            node_id: None,
            slug: None,
            query_task_id: None,
            cache_key: None,
            connection_name: None,
            dialect: None,
            connection_id: None,
            message: None,
            status: None,
            runtime: None,
            sql: None,
        }
    }
}


