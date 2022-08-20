use geojson::{Feature, GeoJson, Geometry, Value, JsonObject, JsonValue};

fn print_some_test_geojson() {
    let geometry = Geometry::new(
        Value::Point(vec![-120.66029,35.2812])
    );

    let mut properties = JsonObject::new();
    properties.insert(
        String::from("name"),
        JsonValue::from("Firestone Grill"),
    );

    let geojson = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: Some(properties),
        foreign_members: None,
    });

    let geojson_string = geojson.to_string();

    println!("{}", geojson_string);
}

fn main() {
    print_some_test_geojson();
}
