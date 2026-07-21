use rust_cloud::storage::{DataRequirements, StorageDecisionError, recommend_storage};

fn main() {
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

    println!("El modelo rechaza una promesa de dato contradictoria.");
}
