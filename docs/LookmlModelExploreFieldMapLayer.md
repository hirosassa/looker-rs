# LookmlModelExploreFieldMapLayer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | URL to the map layer resource. | [optional][readonly]
**name** | Option<**String**> | Name of the map layer, as defined in LookML. | [optional][readonly]
**feature_key** | Option<**String**> | Specifies the name of the TopoJSON object that the map layer references. If not specified, use the first object.. | [optional][readonly]
**property_key** | Option<**String**> | Selects which property from the TopoJSON data to plot against. TopoJSON supports arbitrary metadata for each region. When null, the first matching property should be used. | [optional][readonly]
**property_label_key** | Option<**String**> | Which property from the TopoJSON data to use to label the region. When null, property_key should be used. | [optional][readonly]
**projection** | Option<**String**> | The preferred geographic projection of the map layer when displayed in a visualization that supports multiple geographic projections. | [optional][readonly]
**format** | Option<**String**> | Specifies the data format of the region information. Valid values are: \"topojson\", \"vector_tile_region\". | [optional][readonly]
**extents_json_url** | Option<**String**> | Specifies the URL to a JSON file that defines the geographic extents of each region available in the map layer. This data is used to automatically center the map on the available data for visualization purposes. The JSON file must be a JSON object where the keys are the mapping value of the feature (as specified by property_key) and the values are arrays of four numbers representing the west longitude, south latitude, east longitude, and north latitude extents of the region. The object must include a key for every possible value of property_key. | [optional][readonly]
**max_zoom_level** | Option<**i64**> | The minimum zoom level that the map layer may be displayed at, for visualizations that support zooming. | [optional][readonly]
**min_zoom_level** | Option<**i64**> | The maximum zoom level that the map layer may be displayed at, for visualizations that support zooming. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


