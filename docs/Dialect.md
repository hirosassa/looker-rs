# Dialect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the dialect | [optional][readonly]
**label** | Option<**String**> | The human-readable label of the connection | [optional][readonly]
**supports_cost_estimate** | Option<**bool**> | Whether the dialect supports query cost estimates | [optional][readonly]
**persistent_table_indexes** | Option<**String**> | PDT index columns | [optional][readonly]
**persistent_table_sortkeys** | Option<**String**> | PDT sortkey columns | [optional][readonly]
**persistent_table_distkey** | Option<**String**> | PDT distkey column | [optional][readonly]
**supports_streaming** | Option<**bool**> | Suports streaming results | [optional][readonly]
**automatically_run_sql_runner_snippets** | Option<**bool**> | Should SQL Runner snippets automatically be run | [optional][readonly]
**connection_tests** | Option<**Vec<String>**> | Array of names of the tests that can be run on a connection using this dialect | [optional][readonly]
**supports_inducer** | Option<**bool**> | Is supported with the inducer (i.e. generate from sql) | [optional][readonly]
**supports_multiple_databases** | Option<**bool**> | Can multiple databases be accessed from a connection using this dialect | [optional][readonly]
**supports_persistent_derived_tables** | Option<**bool**> | Whether the dialect supports allowing Looker to build persistent derived tables | [optional][readonly]
**has_ssl_support** | Option<**bool**> | Does the database have client SSL support settable through the JDBC string explicitly? | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


