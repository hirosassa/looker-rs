# Integration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | ID of the integration. | [optional][readonly]
**integration_hub_id** | Option<**i64**> | ID of the integration hub. | [optional][readonly]
**label** | Option<**String**> | Label for the integration. | [optional][readonly]
**description** | Option<**String**> | Description of the integration. | [optional][readonly]
**enabled** | Option<**bool**> | Whether the integration is available to users. | [optional]
**params** | Option<[**Vec<crate::models::IntegrationParam>**](IntegrationParam.md)> | Array of params for the integration. | [optional]
**supported_formats** | Option<**Vec<String>**> | A list of data formats the integration supports. If unspecified, the default is all data formats. Valid values are: \"txt\", \"csv\", \"inline_json\", \"json\", \"json_label\", \"json_detail\", \"json_detail_lite_stream\", \"xlsx\", \"html\", \"wysiwyg_pdf\", \"assembled_pdf\", \"wysiwyg_png\", \"csv_zip\". | [optional][readonly]
**supported_action_types** | Option<**Vec<String>**> | A list of action types the integration supports. Valid values are: \"cell\", \"query\", \"dashboard\". | [optional][readonly]
**supported_formattings** | Option<**Vec<String>**> | A list of formatting options the integration supports. If unspecified, defaults to all formats. Valid values are: \"formatted\", \"unformatted\". | [optional][readonly]
**supported_visualization_formattings** | Option<**Vec<String>**> | A list of visualization formatting options the integration supports. If unspecified, defaults to all formats. Valid values are: \"apply\", \"noapply\". | [optional][readonly]
**supported_download_settings** | Option<**Vec<String>**> | A list of all the download mechanisms the integration supports. The order of values is not significant: Looker will select the most appropriate supported download mechanism for a given query. The integration must ensure it can handle any of the mechanisms it claims to support. If unspecified, this defaults to all download setting values. Valid values are: \"push\", \"url\". | [optional][readonly]
**icon_url** | Option<**String**> | URL to an icon for the integration. | [optional][readonly]
**uses_oauth** | Option<**bool**> | Whether the integration uses oauth. | [optional][readonly]
**required_fields** | Option<[**Vec<crate::models::IntegrationRequiredField>**](IntegrationRequiredField.md)> | A list of descriptions of required fields that this integration is compatible with. If there are multiple entries in this list, the integration requires more than one field. If unspecified, no fields will be required. | [optional][readonly]
**delegate_oauth** | Option<**bool**> | Whether the integration uses delegate oauth, which allows federation between an integration installation scope specific entity (like org, group, and team, etc.) and Looker. | [optional][readonly]
**installed_delegate_oauth_targets** | Option<**Vec<i64>**> | Whether the integration is available to users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


