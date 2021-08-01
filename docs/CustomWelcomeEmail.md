# CustomWelcomeEmail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**enabled** | Option<**bool**> | If true, custom email content will replace the default body of welcome emails | [optional]
**content** | Option<**String**> | The HTML to use as custom content for welcome emails. Script elements and other potentially dangerous markup will be removed. | [optional]
**subject** | Option<**String**> | The text to appear in the email subject line. | [optional]
**header** | Option<**String**> | The text to appear in the header line of the email body. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


