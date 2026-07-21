use rust_cloud::compute::{
    ComputeDecisionError, ResourceRequest, WorkloadRequirements, recommend_compute,
};

fn main() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(2, 4096).unwrap()),
        requires_operating_system_control: true,
        wants_low_operational_load: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(
        recommend_compute(workload),
        Err(ComputeDecisionError::ConflictingRequirements(
            "control de sistema operativo y baja carga operativa requieren decidir el tradeoff",
        ))
    );

    println!("El modelo rechaza supuestos contradictorios antes de recomendar.");
}
