# LookmlModelExplore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Fully qualified explore name (model name plus explore name) | [optional][readonly]
**name** | Option<**String**> | Explore name | [optional][readonly]
**description** | Option<**String**> | Description | [optional][readonly]
**label** | Option<**String**> | Label | [optional][readonly]
**scopes** | Option<**Vec<String>**> | Scopes | [optional][readonly]
**can_total** | Option<**bool**> | Can Total | [optional][readonly]
**can_save** | Option<**bool**> | Can Save | [optional][readonly]
**can_explain** | Option<**bool**> | Can Explain | [optional][readonly]
**can_pivot_in_db** | Option<**bool**> | Can pivot in the DB | [optional][readonly]
**can_subtotal** | Option<**bool**> | Can use subtotals | [optional][readonly]
**has_timezone_support** | Option<**bool**> | Has timezone support | [optional][readonly]
**supports_cost_estimate** | Option<**bool**> | Cost estimates supported | [optional][readonly]
**connection_name** | Option<**String**> | Connection name | [optional][readonly]
**null_sort_treatment** | Option<**String**> | How nulls are sorted, possible values are \"low\", \"high\", \"first\" and \"last\" | [optional][readonly]
**files** | Option<**Vec<String>**> | List of model source files | [optional][readonly]
**source_file** | Option<**String**> | Primary source_file file | [optional][readonly]
**project_name** | Option<**String**> | Name of project | [optional][readonly]
**model_name** | Option<**String**> | Name of model | [optional][readonly]
**view_name** | Option<**String**> | Name of view | [optional][readonly]
**hidden** | Option<**bool**> | Is hidden | [optional][readonly]
**sql_table_name** | Option<**String**> | A sql_table_name expression that defines what sql table the view/explore maps onto. Example: \"prod_orders2 AS orders\" in a view named orders. | [optional][readonly]
**access_filter_fields** | Option<**Vec<String>**> | (DEPRECATED) Array of access filter field names | [optional][readonly]
**access_filters** | Option<[**Vec<crate::models::LookmlModelExploreAccessFilter>**](LookmlModelExploreAccessFilter.md)> | Access filters | [optional][readonly]
**aliases** | Option<[**Vec<crate::models::LookmlModelExploreAlias>**](LookmlModelExploreAlias.md)> | Aliases | [optional][readonly]
**always_filter** | Option<[**Vec<crate::models::LookmlModelExploreAlwaysFilter>**](LookmlModelExploreAlwaysFilter.md)> | Always filter | [optional][readonly]
**conditionally_filter** | Option<[**Vec<crate::models::LookmlModelExploreConditionallyFilter>**](LookmlModelExploreConditionallyFilter.md)> | Conditionally filter | [optional][readonly]
**index_fields** | Option<**Vec<String>**> | Array of index fields | [optional][readonly]
**sets** | Option<[**Vec<crate::models::LookmlModelExploreSet>**](LookmlModelExploreSet.md)> | Sets | [optional][readonly]
**tags** | Option<**Vec<String>**> | An array of arbitrary string tags provided in the model for this explore. | [optional][readonly]
**errors** | Option<[**Vec<crate::models::LookmlModelExploreError>**](LookmlModelExploreError.md)> | Errors | [optional][readonly]
**fields** | Option<[**crate::models::LookmlModelExploreFieldset**](LookmlModelExploreFieldset.md)> |  | [optional]
**joins** | Option<[**Vec<crate::models::LookmlModelExploreJoins>**](LookmlModelExploreJoins.md)> | Views joined into this explore | [optional][readonly]
**group_label** | Option<**String**> | Label used to group explores in the navigation menus | [optional][readonly]
**supported_measure_types** | Option<[**Vec<crate::models::LookmlModelExploreSupportedMeasureType>**](LookmlModelExploreSupportedMeasureType.md)> | An array of items describing which custom measure types are supported for creating a custom measure 'based_on' each possible dimension type. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


