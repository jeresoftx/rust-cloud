use rust_cloud::compute::{
    ComputeMode, ResourceRequest, WorkloadRequirements, compute_modes, recommend_compute,
};

fn main() {
    println!("Cómputo como decisión de ejecución\n");

    for mode in compute_modes() {
        let profile = mode.profile();

        println!(
            "- {mode:?}: aislamiento={:?}, ciclo={:?}, escalado={:?}, control={:?}, operación={:?}, elasticidad={:?}",
            profile.isolation,
            profile.lifecycle,
            profile.scaling,
            profile.team_control,
            profile.team_operational_load,
            profile.elasticity
        );
    }

    let workloads = [
        (
            "api empacada como contenedor",
            WorkloadRequirements {
                resources: Some(ResourceRequest::new(1, 1024).unwrap()),
                packaged_as_container: true,
                wants_low_operational_load: true,
                ..WorkloadRequirements::default()
            },
        ),
        (
            "evento pequeño",
            WorkloadRequirements {
                resources: Some(ResourceRequest::new(1, 512).unwrap()),
                event_driven: true,
                wants_low_operational_load: true,
                ..WorkloadRequirements::default()
            },
        ),
        (
            "proceso batch nocturno",
            WorkloadRequirements {
                resources: Some(ResourceRequest::new(4, 8192).unwrap()),
                scheduled_or_batch: true,
                ..WorkloadRequirements::default()
            },
        ),
    ];

    println!("\nRecomendaciones educativas");
    for (name, workload) in workloads {
        println!("- {name}: {:?}", recommend_compute(workload));
    }

    assert_eq!(
        recommend_compute(workloads[0].1),
        Ok(ComputeMode::ManagedContainer)
    );
    assert_eq!(recommend_compute(workloads[1].1), Ok(ComputeMode::Function));
    assert_eq!(recommend_compute(workloads[2].1), Ok(ComputeMode::BatchJob));
}
