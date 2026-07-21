use rust_cloud::storage::{
    AccessFrequency, DataLifecycle, DataRequirements, StorageMode, recommend_storage, storage_modes,
};

fn main() {
    println!("Almacenamiento como promesa del dato\n");

    for mode in storage_modes() {
        let profile = mode.profile();

        println!(
            "- {mode:?}: acceso={:?}, durabilidad={:?}, consistencia={:?}, ciclo={:?}, recuperación={:?}, operación={:?}",
            profile.access_pattern,
            profile.durability,
            profile.consistency,
            profile.lifecycle,
            profile.retrieval_cost,
            profile.team_operational_load
        );
    }

    let data_sets = [
        (
            "assets públicos del sitio",
            DataRequirements {
                size_gib: Some(10),
                source_of_truth: true,
                accessed_by_key: true,
                read_frequency: AccessFrequency::Warm,
                ..DataRequirements::default()
            },
        ),
        (
            "scratch de generación de assets",
            DataRequirements {
                size_gib: Some(2),
                temporary_data: true,
                ..DataRequirements::default()
            },
        ),
        (
            "retención histórica de entregables",
            DataRequirements {
                size_gib: Some(500),
                source_of_truth: true,
                accessed_by_key: true,
                read_frequency: AccessFrequency::Archive,
                lifecycle: DataLifecycle::Archived,
                ..DataRequirements::default()
            },
        ),
    ];

    println!("\nRecomendaciones educativas");
    for (name, data) in data_sets {
        println!("- {name}: {:?}", recommend_storage(data));
    }

    assert_eq!(recommend_storage(data_sets[0].1), Ok(StorageMode::Object));
    assert_eq!(
        recommend_storage(data_sets[1].1),
        Ok(StorageMode::Ephemeral)
    );
    assert_eq!(recommend_storage(data_sets[2].1), Ok(StorageMode::Archive));
}
