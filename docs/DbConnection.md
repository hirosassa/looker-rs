# DbConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**name** | Option<**String**> | Name of the connection. Also used as the unique identifier | [optional]
**dialect** | Option<[**crate::models::Dialect**](Dialect.md)> |  | [optional]
**snippets** | Option<[**Vec<crate::models::Snippet>**](Snippet.md)> | SQL Runner snippets for this connection | [optional][readonly]
**pdts_enabled** | Option<**bool**> | True if PDTs are enabled on this connection | [optional][readonly]
**host** | Option<**String**> | Host name/address of server | [optional]
**port** | Option<**String**> | Port number on server | [optional]
**username** | Option<**String**> | Username for server authentication | [optional]
**password** | Option<**String**> | (Write-Only) Password for server authentication | [optional]
**uses_oauth** | Option<**bool**> | Whether the connection uses OAuth for authentication. | [optional][readonly]
**certificate** | Option<**String**> | (Write-Only) Base64 encoded Certificate body for server authentication (when appropriate for dialect). | [optional]
**file_type** | Option<**String**> | (Write-Only) Certificate keyfile type - .json or .p12 | [optional]
**database** | Option<**String**> | Database name | [optional]
**db_timezone** | Option<**String**> | Time zone of database | [optional]
**query_timezone** | Option<**String**> | Timezone to use in queries | [optional]
**schema** | Option<**String**> | Scheme name | [optional]
**max_connections** | Option<**i64**> | Maximum number of concurrent connection to use | [optional]
**max_billing_gigabytes** | Option<**String**> | Maximum size of query in GBs (BigQuery only, can be a user_attribute name) | [optional]
**ssl** | Option<**bool**> | Use SSL/TLS when connecting to server | [optional]
**verify_ssl** | Option<**bool**> | Verify the SSL | [optional]
**tmp_db_name** | Option<**String**> | Name of temporary database (if used) | [optional]
**jdbc_additional_params** | Option<**String**> | Additional params to add to JDBC connection string | [optional]
**pool_timeout** | Option<**i64**> | Connection Pool Timeout, in seconds | [optional]
**dialect_name** | Option<**String**> | (Read/Write) SQL Dialect name | [optional]
**created_at** | Option<**String**> | Creation date for this connection | [optional][readonly]
**user_id** | Option<**String**> | Id of user who last modified this connection configuration | [optional][readonly]
**example** | Option<**bool**> | Is this an example connection? | [optional][readonly]
**user_db_credentials** | Option<**bool**> | (Limited access feature) Are per user db credentials enabled. Enabling will remove previously set username and password | [optional]
**user_attribute_fields** | Option<**Vec<String>**> | Fields whose values map to user attribute names | [optional]
**maintenance_cron** | Option<**String**> | Cron string specifying when maintenance such as PDT trigger checks and drops should be performed | [optional]
**last_regen_at** | Option<**String**> | Unix timestamp at start of last completed PDT trigger check process | [optional][readonly]
**last_reap_at** | Option<**String**> | Unix timestamp at start of last completed PDT reap process | [optional][readonly]
**sql_runner_precache_tables** | Option<**bool**> | Precache tables in the SQL Runner | [optional]
**sql_writing_with_info_schema** | Option<**bool**> | Fetch Information Schema For SQL Writing | [optional]
**after_connect_statements** | Option<**String**> | SQL statements (semicolon separated) to issue after connecting to the database. Requires `custom_after_connect_statements` license feature | [optional]
**pdt_context_override** | Option<[**crate::models::DbConnectionOverride**](DBConnectionOverride.md)> |  | [optional]
**managed** | Option<**bool**> | Is this connection created and managed by Looker | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


