# IntegrationParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the parameter. | [optional]
**label** | Option<**String**> | Label of the parameter. | [optional][readonly]
**description** | Option<**String**> | Short description of the parameter. | [optional][readonly]
**required** | Option<**bool**> | Whether the parameter is required to be set to use the destination. If unspecified, this defaults to false. | [optional][readonly]
**has_value** | Option<**bool**> | Whether the parameter has a value set. | [optional][readonly]
**value** | Option<**String**> | The current value of the parameter. Always null if the value is sensitive. When writing, null values will be ignored. Set the value to an empty string to clear it. | [optional]
**user_attribute_name** | Option<**String**> | When present, the param's value comes from this user attribute instead of the 'value' parameter. Set to null to use the 'value'. | [optional]
**sensitive** | Option<**bool**> | Whether the parameter contains sensitive data like API credentials. If unspecified, this defaults to true. | [optional][readonly]
**per_user** | Option<**bool**> | When true, this parameter must be assigned to a user attribute in the admin panel (instead of a constant value), and that value may be updated by the user as part of the integration flow. | [optional][readonly]
**delegate_oauth_url** | Option<**String**> | When present, the param represents the oauth url the user will be taken to. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


