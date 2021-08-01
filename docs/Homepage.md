# Homepage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**content_metadata_id** | Option<**i64**> | Id of associated content_metadata record | [optional][readonly]
**created_at** | Option<**String**> | Date of homepage creation | [optional][readonly]
**deleted_at** | Option<**String**> | Date of homepage deletion | [optional]
**description** | Option<**String**> | Description of the homepage | [optional]
**homepage_sections** | Option<[**Vec<crate::models::HomepageSection>**](HomepageSection.md)> | Sections of the homepage | [optional][readonly]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**section_order** | Option<**Vec<i64>**> | ids of the homepage sections in the order they should be displayed | [optional]
**title** | Option<**String**> | Title of the homepage | [optional]
**updated_at** | Option<**String**> | Date of last homepage update | [optional][readonly]
**user_id** | Option<**i64**> | User id of homepage creator | [optional][readonly]
**primary_homepage** | Option<**bool**> | Whether the homepage is the primary homepage or not | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


