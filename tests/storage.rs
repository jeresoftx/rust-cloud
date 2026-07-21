use rust_cloud::storage::{
    AccessFrequency, AccessPattern, DataLifecycle, DataRequirements, DurabilityExpectation,
    StorageDecisionError, StorageMode, recommend_storage, storage_modes,
};

#[test]
fn storage_modes_expose_expected_profiles() {
    assert_eq!(
        StorageMode::Object.profile().access_pattern,
        AccessPattern::KeyAddressed
    );
    assert_eq!(
        StorageMode::Block.profile().access_pattern,
        AccessPattern::MountedVolume
    );
    assert_eq!(
        StorageMode::File.profile().access_pattern,
        AccessPattern::SharedFilesystem
    );
    assert_eq!(
        StorageMode::Ephemeral.profile().durability,
        DurabilityExpectation::Ephemeral
    );
}

#[test]
fn modes_include_archive_as_slowest_recovery() {
    let levels: Vec<u8> = storage_modes()
        .iter()
        .map(|mode| mode.recovery_delay_level())
        .collect();

    assert_eq!(levels, vec![2, 1, 1, 0, 3]);
    assert_eq!(
        StorageMode::Archive.profile().retrieval_cost,
        rust_cloud::storage::StorageScore::High
    );
}

#[test]
fn recommendation_requires_size() {
    assert_eq!(
        recommend_storage(DataRequirements::default()),
        Err(StorageDecisionError::MissingSize)
    );
}

#[test]
fn recommendation_rejects_zero_size() {
    let data = DataRequirements {
        size_gib: Some(0),
        source_of_truth: true,
        accessed_by_key: true,
        ..DataRequirements::default()
    };

    assert_eq!(
        recommend_storage(data),
        Err(StorageDecisionError::InvalidSize(
            "size_gib debe ser mayor que cero",
        ))
    );
}

#[test]
fn recommendation_requires_truth_declaration() {
    let data = DataRequirements {
        size_gib: Some(1),
        accessed_by_key: true,
        ..DataRequirements::default()
    };

    assert_eq!(
        recommend_storage(data),
        Err(StorageDecisionError::MissingTruthDeclaration)
    );
}

#[test]
fn recommendation_prefers_ephemeral_for_temporary_data() {
    let data = DataRequirements {
        size_gib: Some(1),
        temporary_data: true,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(data), Ok(StorageMode::Ephemeral));
}

#[test]
fn recommendation_prefers_archive_for_archived_lifecycle() {
    let data = DataRequirements {
        size_gib: Some(100),
        source_of_truth: true,
        accessed_by_key: true,
        lifecycle: DataLifecycle::Archived,
        read_frequency: AccessFrequency::Archive,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(data), Ok(StorageMode::Archive));
}

#[test]
fn recommendation_prefers_file_for_shared_mounts() {
    let data = DataRequirements {
        size_gib: Some(50),
        source_of_truth: true,
        requires_mount: true,
        shared_between_workloads: true,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(data), Ok(StorageMode::File));
}

#[test]
fn recommendation_prefers_block_for_single_mounted_volume() {
    let data = DataRequirements {
        size_gib: Some(20),
        source_of_truth: true,
        requires_mount: true,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(data), Ok(StorageMode::Block));
}

#[test]
fn recommendation_prefers_object_for_key_addressed_data() {
    let data = DataRequirements {
        size_gib: Some(5),
        source_of_truth: true,
        accessed_by_key: true,
        ..DataRequirements::default()
    };

    assert_eq!(recommend_storage(data), Ok(StorageMode::Object));
}

#[test]
fn recommendation_exposes_source_of_truth_and_temporary_conflict() {
    let data = DataRequirements {
        size_gib: Some(1),
        source_of_truth: true,
        temporary_data: true,
        ..DataRequirements::default()
    };

    assert_eq!(
        recommend_storage(data),
        Err(StorageDecisionError::ConflictingRequirements(
            "un dato no puede ser fuente de verdad y temporal al mismo tiempo",
        ))
    );
}

#[test]
fn recommendation_requires_access_pattern_for_durable_data() {
    let data = DataRequirements {
        size_gib: Some(1),
        source_of_truth: true,
        ..DataRequirements::default()
    };

    assert_eq!(
        recommend_storage(data),
        Err(StorageDecisionError::MissingAccessPattern)
    );
}
