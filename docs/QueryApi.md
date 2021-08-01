# \QueryApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_running_queries**](QueryApi.md#all_running_queries) | **GET** /running_queries | Get All Running Queries
[**create_merge_query**](QueryApi.md#create_merge_query) | **POST** /merge_queries | Create Merge Query
[**create_query**](QueryApi.md#create_query) | **POST** /queries | Create Query
[**create_query_task**](QueryApi.md#create_query_task) | **POST** /query_tasks | Run Query Async
[**create_sql_query**](QueryApi.md#create_sql_query) | **POST** /sql_queries | Create SQL Runner Query
[**kill_query**](QueryApi.md#kill_query) | **DELETE** /running_queries/{query_task_id} | Kill Running Query
[**merge_query**](QueryApi.md#merge_query) | **GET** /merge_queries/{merge_query_id} | Get Merge Query
[**query**](QueryApi.md#query) | **GET** /queries/{query_id} | Get Query
[**query_for_slug**](QueryApi.md#query_for_slug) | **GET** /queries/slug/{slug} | Get Query for Slug
[**query_task**](QueryApi.md#query_task) | **GET** /query_tasks/{query_task_id} | Get Async Query Info
[**query_task_multi_results**](QueryApi.md#query_task_multi_results) | **GET** /query_tasks/multi_results | Get Multiple Async Query Results
[**query_task_results**](QueryApi.md#query_task_results) | **GET** /query_tasks/{query_task_id}/results | Get Async Query Results
[**run_inline_query**](QueryApi.md#run_inline_query) | **POST** /queries/run/{result_format} | Run Inline Query
[**run_query**](QueryApi.md#run_query) | **GET** /queries/{query_id}/run/{result_format} | Run Query
[**run_sql_query**](QueryApi.md#run_sql_query) | **POST** /sql_queries/{slug}/run/{result_format} | Run SQL Runner Query
[**run_url_encoded_query**](QueryApi.md#run_url_encoded_query) | **GET** /queries/models/{model_name}/views/{view_name}/run/{result_format} | Run Url Encoded Query
[**sql_query**](QueryApi.md#sql_query) | **GET** /sql_queries/{slug} | Get SQL Runner Query



## all_running_queries

> Vec<crate::models::RunningQueries> all_running_queries()
Get All Running Queries

Get information about all running queries. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RunningQueries>**](RunningQueries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_merge_query

> crate::models::MergeQuery create_merge_query(fields, body)
Create Merge Query

### Create Merge Query  Creates a new merge query object.  A merge query takes the results of one or more queries and combines (merges) the results according to field mapping definitions. The result is similar to a SQL left outer join.  A merge query can merge results of queries from different SQL databases.  The order that queries are defined in the source_queries array property is significant. The first query in the array defines the primary key into which the results of subsequent queries will be merged.  Like model/view query objects, merge queries are immutable and have structural identity - if you make a request to create a new merge query that is identical to an existing merge query, the existing merge query will be returned instead of creating a duplicate. Conversely, any change to the contents of a merge query will produce a new object with a new id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields |  |
**body** | Option<[**MergeQuery**](MergeQuery.md)> | Merge Query |  |

### Return type

[**crate::models::MergeQuery**](MergeQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_query

> crate::models::Query create_query(body, fields)
Create Query

### Create a query.  This allows you to create a new query that you can later run. Looker queries are immutable once created and are not deleted. If you create a query that is exactly like an existing query then the existing query will be returned and no new query will be created. Whether a new query is created or not, you can use the 'id' in the returned query with the 'run' method.  The query parameters are passed as json in the body of the request.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Query**](Query.md) | Query | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Query**](Query.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_query_task

> crate::models::QueryTask create_query_task(body, limit, apply_formatting, apply_vis, cache, image_width, image_height, generate_drill_links, force_production, cache_only, path_prefix, rebuild_pdts, server_table_calcs, fields)
Run Query Async

### Create an async query task  Creates a query task (job) to run a previously created query asynchronously. Returns a Query Task ID.  Use [query_task(query_task_id)](#!/Query/query_task) to check the execution status of the query task. After the query task status reaches \"Complete\", use [query_task_results(query_task_id)](#!/Query/query_task_results) to fetch the results of the query. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateQueryTask**](CreateQueryTask.md) | Query parameters | [required] |
**limit** | Option<**i64**> | Row limit (may override the limit in the saved query). |  |
**apply_formatting** | Option<**bool**> | Apply model-specified formatting to each result. |  |
**apply_vis** | Option<**bool**> | Apply visualization options to results. |  |
**cache** | Option<**bool**> | Get results from cache if available. |  |
**image_width** | Option<**i64**> | Render width for image formats. |  |
**image_height** | Option<**i64**> | Render height for image formats. |  |
**generate_drill_links** | Option<**bool**> | Generate drill links (only applicable to 'json_detail' format. |  |
**force_production** | Option<**bool**> | Force use of production models even if the user is in development mode. |  |
**cache_only** | Option<**bool**> | Retrieve any results from cache even if the results have expired. |  |
**path_prefix** | Option<**String**> | Prefix to use for drill links (url encoded). |  |
**rebuild_pdts** | Option<**bool**> | Rebuild PDTS used in query. |  |
**server_table_calcs** | Option<**bool**> | Perform table calculations on query results |  |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::QueryTask**](QueryTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sql_query

> crate::models::SqlQuery create_sql_query(body)
Create SQL Runner Query

### Create a SQL Runner Query  Either the `connection_name` or `model_name` parameter MUST be provided. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SqlQueryCreate**](SqlQueryCreate.md) | SQL Runner Query | [required] |

### Return type

[**crate::models::SqlQuery**](SqlQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kill_query

> String kill_query(query_task_id)
Kill Running Query

Kill a query with a specific query_task_id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_task_id** | **String** | Query task id. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_query

> crate::models::MergeQuery merge_query(merge_query_id, fields)
Get Merge Query

### Get Merge Query  Returns a merge query object given its id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merge_query_id** | **String** | Merge Query Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::MergeQuery**](MergeQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query

> crate::models::Query query(query_id, fields)
Get Query

### Get a previously created query by id.  A Looker query object includes the various parameters that define a database query that has been run or could be run in the future. These parameters include: model, view, fields, filters, pivots, etc. Query *results* are not part of the query object.  Query objects are unique and immutable. Query objects are created automatically in Looker as users explore data. Looker does not delete them; they become part of the query history. When asked to create a query for any given set of parameters, Looker will first try to find an existing query object with matching parameters and will only create a new object when an appropriate object can not be found.  This 'get' method is used to get the details about a query for a given id. See the other methods here to 'create' and 'run' queries.  Note that some fields like 'filter_config' and 'vis_config' etc are specific to how the Looker UI builds queries and visualizations and are not generally useful for API use. They are not required when creating new queries and can usually just be ignored.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **i64** | Id of query | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Query**](Query.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_for_slug

> crate::models::Query query_for_slug(slug, fields)
Get Query for Slug

### Get the query for a given query slug.  This returns the query for the 'slug' in a query share URL.  The 'slug' is a randomly chosen short string that is used as an alternative to the query's id value for use in URLs etc. This method exists as a convenience to help you use the API to 'find' queries that have been created using the Looker UI.  You can use the Looker explore page to build a query and then choose the 'Share' option to show the share url for the query. Share urls generally look something like 'https://looker.yourcompany/x/vwGSbfc'. The trailing 'vwGSbfc' is the share slug. You can pass that string to this api method to get details about the query. Those details include the 'id' that you can use to run the query. Or, you can copy the query body (perhaps with your own modification) and use that as the basis to make/run new queries.  This will also work with slugs from Looker explore urls like 'https://looker.yourcompany/explore/ecommerce/orders?qid=aogBgL6o3cKK1jN3RoZl5s'. In this case 'aogBgL6o3cKK1jN3RoZl5s' is the slug. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of query | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Query**](Query.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_task

> crate::models::QueryTask query_task(query_task_id, fields)
Get Async Query Info

### Get Query Task details  Use this function to check the status of an async query task. After the status reaches \"Complete\", you can call [query_task_results(query_task_id)](#!/Query/query_task_results) to retrieve the results of the query.  Use [create_query_task()](#!/Query/create_query_task) to create an async query task. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_task_id** | **String** | ID of the Query Task | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::QueryTask**](QueryTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_task_multi_results

> ::std::collections::HashMap<String, String> query_task_multi_results(query_task_ids)
Get Multiple Async Query Results

### Fetch results of multiple async queries  Returns the results of multiple async queries in one request.  For Query Tasks that are not completed, the response will include the execution status of the Query Task but will not include query results. Query Tasks whose results have expired will have a status of 'expired'. If the user making the API request does not have sufficient privileges to view a Query Task result, the result will have a status of 'missing' 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_task_ids** | [**Vec<String>**](String.md) | List of Query Task IDs | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_task_results

> String query_task_results(query_task_id)
Get Async Query Results

### Get Async Query Results  Returns the results of an async query task if the query has completed.  If the query task is still running or waiting to run, this function returns 204 No Content.  If the query task ID is invalid or the cached results of the query task have expired, this function returns 404 Not Found.  Use [query_task(query_task_id)](#!/Query/query_task) to check the execution status of the query task Call query_task_results only after the query task status reaches \"Complete\".  You can also use [query_task_multi_results()](#!/Query/query_task_multi_results) retrieve the results of multiple async query tasks at the same time.  #### SQL Error Handling: If the query fails due to a SQL db error, how this is communicated depends on the result_format you requested in `create_query_task()`.  For `json_detail` result_format: `query_task_results()` will respond with HTTP status '200 OK' and db SQL error info will be in the `errors` property of the response object. The 'data' property will be empty.  For all other result formats: `query_task_results()` will respond with HTTP status `400 Bad Request` and some db SQL error info will be in the message of the 400 error response, but not as detailed as expressed in `json_detail.errors`. These data formats can only carry row data, and error info is not row data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_task_id** | **String** | ID of the Query Task | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_inline_query

> String run_inline_query(result_format, body, limit, apply_formatting, apply_vis, cache, image_width, image_height, generate_drill_links, force_production, cache_only, path_prefix, rebuild_pdts, server_table_calcs)
Run Inline Query

### Run the query that is specified inline in the posted body.  This allows running a query as defined in json in the posted body. This combines the two actions of posting & running a query into one step.  Here is an example body in json: ``` {   \"model\":\"thelook\",   \"view\":\"inventory_items\",   \"fields\":[\"category.name\",\"inventory_items.days_in_inventory_tier\",\"products.count\"],   \"filters\":{\"category.name\":\"socks\"},   \"sorts\":[\"products.count desc 0\"],   \"limit\":\"500\",   \"query_timezone\":\"America/Los_Angeles\" } ```  When using the Ruby SDK this would be passed as a Ruby hash like: ``` {  :model=>\"thelook\",  :view=>\"inventory_items\",  :fields=>   [\"category.name\",    \"inventory_items.days_in_inventory_tier\",    \"products.count\"],  :filters=>{:\"category.name\"=>\"socks\"},  :sorts=>[\"products.count desc 0\"],  :limit=>\"500\",  :query_timezone=>\"America/Los_Angeles\", } ```  This will return the result of running the query in the format specified by the 'result_format' parameter.  Supported formats:  | result_format | Description | :-----------: | :--- | | json | Plain json | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | md | Simple markdown | xlsx | MS Excel spreadsheet | sql | Returns the generated SQL rather than running the query | png | A PNG image of the visualization of the query | jpg | A JPG image of the visualization of the query   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**result_format** | **String** | Format of result | [required] |
**body** | [**Query**](Query.md) | inline query | [required] |
**limit** | Option<**i64**> | Row limit (may override the limit in the saved query). |  |
**apply_formatting** | Option<**bool**> | Apply model-specified formatting to each result. |  |
**apply_vis** | Option<**bool**> | Apply visualization options to results. |  |
**cache** | Option<**bool**> | Get results from cache if available. |  |
**image_width** | Option<**i64**> | Render width for image formats. |  |
**image_height** | Option<**i64**> | Render height for image formats. |  |
**generate_drill_links** | Option<**bool**> | Generate drill links (only applicable to 'json_detail' format. |  |
**force_production** | Option<**bool**> | Force use of production models even if the user is in development mode. |  |
**cache_only** | Option<**bool**> | Retrieve any results from cache even if the results have expired. |  |
**path_prefix** | Option<**String**> | Prefix to use for drill links (url encoded). |  |
**rebuild_pdts** | Option<**bool**> | Rebuild PDTS used in query. |  |
**server_table_calcs** | Option<**bool**> | Perform table calculations on query results |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text, application/json, image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_query

> String run_query(query_id, result_format, limit, apply_formatting, apply_vis, cache, image_width, image_height, generate_drill_links, force_production, cache_only, path_prefix, rebuild_pdts, server_table_calcs)
Run Query

### Run a saved query.  This runs a previously saved query. You can use this on a query that was generated in the Looker UI or one that you have explicitly created using the API. You can also use a query 'id' from a saved 'Look'.  The 'result_format' parameter specifies the desired structure and format of the response.  Supported formats:  | result_format | Description | :-----------: | :--- | | json | Plain json | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | md | Simple markdown | xlsx | MS Excel spreadsheet | sql | Returns the generated SQL rather than running the query | png | A PNG image of the visualization of the query | jpg | A JPG image of the visualization of the query   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **i64** | Id of query | [required] |
**result_format** | **String** | Format of result | [required] |
**limit** | Option<**i64**> | Row limit (may override the limit in the saved query). |  |
**apply_formatting** | Option<**bool**> | Apply model-specified formatting to each result. |  |
**apply_vis** | Option<**bool**> | Apply visualization options to results. |  |
**cache** | Option<**bool**> | Get results from cache if available. |  |
**image_width** | Option<**i64**> | Render width for image formats. |  |
**image_height** | Option<**i64**> | Render height for image formats. |  |
**generate_drill_links** | Option<**bool**> | Generate drill links (only applicable to 'json_detail' format. |  |
**force_production** | Option<**bool**> | Force use of production models even if the user is in development mode. |  |
**cache_only** | Option<**bool**> | Retrieve any results from cache even if the results have expired. |  |
**path_prefix** | Option<**String**> | Prefix to use for drill links (url encoded). |  |
**rebuild_pdts** | Option<**bool**> | Rebuild PDTS used in query. |  |
**server_table_calcs** | Option<**bool**> | Perform table calculations on query results |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text, application/json, image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_sql_query

> String run_sql_query(slug, result_format, download)
Run SQL Runner Query

Execute a SQL Runner query in a given result_format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | slug of query | [required] |
**result_format** | **String** | Format of result, options are: [\"inline_json\", \"json\", \"json_detail\", \"json_fe\", \"csv\", \"html\", \"md\", \"txt\", \"xlsx\", \"gsxml\", \"json_label\"] | [required] |
**download** | Option<**String**> | Defaults to false. If set to true, the HTTP response will have content-disposition and other headers set to make the HTTP response behave as a downloadable attachment instead of as inline content. |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text, application/json, image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_url_encoded_query

> String run_url_encoded_query(model_name, view_name, result_format)
Run Url Encoded Query

### Run an URL encoded query.  This requires the caller to encode the specifiers for the query into the URL query part using Looker-specific syntax as explained below.  Generally, you would want to use one of the methods that takes the parameters as json in the POST body for creating and/or running queries. This method exists for cases where one really needs to encode the parameters into the URL of a single 'GET' request. This matches the way that the Looker UI formats 'explore' URLs etc.  The parameters here are very similar to the json body formatting except that the filter syntax is tricky. Unfortunately, this format makes this method not currently callable via the 'Try it out!' button in this documentation page. But, this is callable when creating URLs manually or when using the Looker SDK.  Here is an example inline query URL:  ``` https://looker.mycompany.com:19999/api/3.0/queries/models/thelook/views/inventory_items/run/json?fields=category.name,inventory_items.days_in_inventory_tier,products.count&f[category.name]=socks&sorts=products.count+desc+0&limit=500&query_timezone=America/Los_Angeles ```  When invoking this endpoint with the Ruby SDK, pass the query parameter parts as a hash. The hash to match the above would look like:  ```ruby query_params = {   :fields => \"category.name,inventory_items.days_in_inventory_tier,products.count\",   :\"f[category.name]\" => \"socks\",   :sorts => \"products.count desc 0\",   :limit => \"500\",   :query_timezone => \"America/Los_Angeles\" } response = ruby_sdk.run_url_encoded_query('thelook','inventory_items','json', query_params)  ```  Again, it is generally easier to use the variant of this method that passes the full query in the POST body. This method is available for cases where other alternatives won't fit the need.  Supported formats:  | result_format | Description | :-----------: | :--- | | json | Plain json | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | md | Simple markdown | xlsx | MS Excel spreadsheet | sql | Returns the generated SQL rather than running the query | png | A PNG image of the visualization of the query | jpg | A JPG image of the visualization of the query   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_name** | **String** | Model name | [required] |
**view_name** | **String** | View name | [required] |
**result_format** | **String** | Format of result | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text, application/json, image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sql_query

> crate::models::SqlQuery sql_query(slug)
Get SQL Runner Query

Get a SQL Runner query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | slug of query | [required] |

### Return type

[**crate::models::SqlQuery**](SqlQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

