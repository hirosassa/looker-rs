# HomepageSection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**created_at** | Option<**String**> | Time at which this section was created. | [optional][readonly]
**deleted_at** | Option<**String**> | Time at which this section was deleted. | [optional]
**detail_url** | Option<**String**> | A URL pointing to a page showing further information about the content in the section. | [optional][readonly]
**homepage_id** | Option<**i64**> | Id reference to parent homepage | [optional]
**homepage_items** | Option<[**Vec<crate::models::HomepageItem>**](HomepageItem.md)> | Items in the homepage section | [optional][readonly]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**is_header** | Option<**bool**> | Is this a header section (has no items) | [optional][readonly]
**item_order** | Option<**Vec<i64>**> | ids of the homepage items in the order they should be displayed | [optional]
**title** | Option<**String**> | Name of row | [optional]
**updated_at** | Option<**String**> | Time at which this section was last updated. | [optional][readonly]
**description** | Option<**String**> | Description of the content found in this section. | [optional]
**visible_item_order** | Option<**Vec<i64>**> | ids of the homepage items the user can see in the order they should be displayed | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


