# DataActionFormField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name | [optional][readonly]
**label** | Option<**String**> | Human-readable label | [optional][readonly]
**description** | Option<**String**> | Description of field | [optional][readonly]
**_type** | Option<**String**> | Type of field. | [optional][readonly]
**default** | Option<**String**> | Default value of the field. | [optional][readonly]
**oauth_url** | Option<**String**> | The URL for an oauth link, if type is 'oauth_link'. | [optional][readonly]
**interactive** | Option<**bool**> | Whether or not a field supports interactive forms. | [optional][readonly]
**required** | Option<**bool**> | Whether or not the field is required. This is a user-interface hint. A user interface displaying this form should not submit it without a value for this field. The action server must also perform this validation. | [optional][readonly]
**options** | Option<[**Vec<crate::models::DataActionFormSelectOption>**](DataActionFormSelectOption.md)> | If the form type is 'select', a list of options to be selected from. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


