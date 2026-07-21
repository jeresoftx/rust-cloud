use rust_cloud::storage::{AccessFrequency, DataRequirements, StorageMode, recommend_storage};

fn main() {
    let assets = DataRequirements {
        size_gib: Some(10),
        source_of_truth: true,
        accessed_by_key: true,
        read_frequency: AccessFrequency::Warm,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(assets), Ok(StorageMode::Object));

    println!("Los assets por clave encajan con almacenamiento de objetos.");
}
