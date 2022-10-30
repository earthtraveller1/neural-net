use super::DataLayer;
use super::WeightLayer;

#[test]
fn data_layer_csv_deserialization() {
    let test_data = DataLayer::from_csv_file("data/testdata.csv").expect("Failed to load data/testdata.csv");
    assert_eq!(test_data[0], 1.0);
    assert_eq!(test_data[1], 2.0);
    assert_eq!(test_data[2], 3.0);
    assert_eq!(test_data[3], 4.0);
    assert_eq!(test_data[4], 5.0);
}

#[test]
fn weight_layer_csv_and_get_methods() {
    let test_data = WeightLayer::from_csv_file("data/testweights.csv").expect("Failed to load data/testweights.csv");
    assert_eq!(test_data.get_value(0, 0), 1.1);
    assert_eq!(test_data.get_value(1, 0), 1.2);
    assert_eq!(test_data.get_value(2, 0), 3.0);
    assert_eq!(test_data.get_value(3, 0), 4.4);
    
    assert_eq!(test_data.get_value(0, 1), 3.1);
    assert_eq!(test_data.get_value(1, 1), 1.1);
    assert_eq!(test_data.get_value(2, 1), 2.3);
    assert_eq!(test_data.get_value(3, 1), 1.2);
    
    assert_eq!(test_data.get_value(0, 2), 4.5);
    assert_eq!(test_data.get_value(1, 2), 2.2);
    assert_eq!(test_data.get_value(2, 2), 9.9);
    assert_eq!(test_data.get_value(3, 2), 8.8);
}