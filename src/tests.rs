use super::DataLayer;

#[test]
fn data_layer_csv_deserialization() {
    let test_data = DataLayer::from_csv_file("data/testdata.csv").expect("Failed to load data/testdata.csv");
    assert_eq!(test_data[0], 1.0);
    assert_eq!(test_data[1], 2.0);
    assert_eq!(test_data[2], 3.0);
    assert_eq!(test_data[3], 4.0);
    assert_eq!(test_data[4], 5.0);
}