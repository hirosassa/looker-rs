# WhitelabelConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**logo_file** | Option<**String**> | Customer logo image. Expected base64 encoded data (write-only) | [optional]
**logo_url** | Option<**String**> | Logo image url (read-only) | [optional][readonly]
**favicon_file** | Option<**String**> | Custom favicon image. Expected base64 encoded data (write-only) | [optional]
**favicon_url** | Option<**String**> | Favicon image url (read-only) | [optional][readonly]
**default_title** | Option<**String**> | Default page title | [optional]
**show_help_menu** | Option<**bool**> | Boolean to toggle showing help menus | [optional]
**show_docs** | Option<**bool**> | Boolean to toggle showing docs | [optional]
**show_email_sub_options** | Option<**bool**> | Boolean to toggle showing email subscription options. | [optional]
**allow_looker_mentions** | Option<**bool**> | Boolean to toggle mentions of Looker in emails | [optional]
**allow_looker_links** | Option<**bool**> | Boolean to toggle links to Looker in emails | [optional]
**custom_welcome_email_advanced** | Option<**bool**> | Allow subject line and email heading customization in customized emails” | [optional]
**setup_mentions** | Option<**bool**> | Remove the word Looker from appearing in the account setup page | [optional]
**alerts_logo** | Option<**bool**> | Remove Looker logo from Alerts | [optional]
**alerts_links** | Option<**bool**> | Remove Looker links from Alerts | [optional]
**folders_mentions** | Option<**bool**> | Remove Looker mentions in home folder page when you don’t have any items saved | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


