# LookmlTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**model_name** | Option<**String**> | Name of model containing this test. | [optional][readonly]
**test_name** | Option<**String**> | Name of this test. | [optional][readonly]
**assertions_count** | Option<**i64**> | Number of assertions in this test | [optional][readonly]
**assertions_failed** | Option<**i64**> | Number of assertions passed in this test | [optional][readonly]
**errors** | Option<[**Vec<crate::models::ProjectError>**](ProjectError.md)> | A list of any errors encountered by the test. | [optional][readonly]
**warnings** | Option<[**Vec<crate::models::ProjectError>**](ProjectError.md)> | A list of any warnings encountered by the test. | [optional][readonly]
**success** | Option<**bool**> | True if this test passsed without errors. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


