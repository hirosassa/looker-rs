# LookmlModelExploreJoins

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of this join (and name of the view to join) | [optional][readonly]
**dependent_fields** | Option<**Vec<String>**> | Fields referenced by the join | [optional][readonly]
**fields** | Option<**Vec<String>**> | Fields of the joined view to pull into this explore | [optional][readonly]
**foreign_key** | Option<**String**> | Name of the dimension in this explore whose value is in the primary key of the joined view | [optional][readonly]
**from** | Option<**String**> | Name of view to join | [optional][readonly]
**outer_only** | Option<**bool**> | Specifies whether all queries must use an outer join | [optional][readonly]
**relationship** | Option<**String**> | many_to_one, one_to_one, one_to_many, many_to_many | [optional][readonly]
**required_joins** | Option<**Vec<String>**> | Names of joins that must always be included in SQL queries | [optional][readonly]
**sql_foreign_key** | Option<**String**> | SQL expression that produces a foreign key | [optional][readonly]
**sql_on** | Option<**String**> | SQL ON expression describing the join condition | [optional][readonly]
**sql_table_name** | Option<**String**> | SQL table name to join | [optional][readonly]
**_type** | Option<**String**> | The join type: left_outer, full_outer, inner, or cross | [optional][readonly]
**view_label** | Option<**String**> | Label to display in UI selectors | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


