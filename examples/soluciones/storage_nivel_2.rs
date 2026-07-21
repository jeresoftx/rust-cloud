use rust_cloud::storage::{
    AccessFrequency, DataLifecycle, DataRequirements, StorageMode, recommend_storage,
};

fn main() {
    let assets = DataRequirements {
        size_gib: Some(10),
        source_of_truth: true,
        accessed_by_key: true,
        read_frequency: AccessFrequency::Warm,
        ..DataRequirements::default()
    };

    let scratch = DataRequirements {
        size_gib: Some(2),
        temporary_data: true,
        ..DataRequirements::default()
    };

    let archive = DataRequirements {
        size_gib: Some(500),
        source_of_truth: true,
        accessed_by_key: true,
        read_frequency: AccessFrequency::Archive,
        lifecycle: DataLifecycle::Archived,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(assets), Ok(StorageMode::Object));
    assert_eq!(recommend_storage(scratch), Ok(StorageMode::Ephemeral));
    assert_eq!(recommend_storage(archive), Ok(StorageMode::Archive));

    println!("Assets, scratch y archivo histórico producen promesas distintas.");
}
