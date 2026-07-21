use rust_cloud::compute::{ComputeMode, ResourceRequest, WorkloadRequirements, recommend_compute};

fn main() {
    let api = WorkloadRequirements {
        resources: Some(ResourceRequest::new(1, 1024).unwrap()),
        packaged_as_container: true,
        wants_low_operational_load: true,
        ..WorkloadRequirements::default()
    };

    let event = WorkloadRequirements {
        resources: Some(ResourceRequest::new(1, 512).unwrap()),
        event_driven: true,
        wants_low_operational_load: true,
        ..WorkloadRequirements::default()
    };

    let batch = WorkloadRequirements {
        resources: Some(ResourceRequest::new(4, 8192).unwrap()),
        scheduled_or_batch: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(recommend_compute(api), Ok(ComputeMode::ManagedContainer));
    assert_eq!(recommend_compute(event), Ok(ComputeMode::Function));
    assert_eq!(recommend_compute(batch), Ok(ComputeMode::BatchJob));

    println!("API, evento y batch producen recomendaciones distintas por sus supuestos.");
}
