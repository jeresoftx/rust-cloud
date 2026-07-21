use rust_cloud::compute::{
    ComputeDecisionError, ComputeMode, IsolationBoundary, LifecyclePolicy, ResourceLimit,
    ResourceRequest, ScalingSignal, WorkloadRequirements, compute_modes, recommend_compute,
};

#[test]
fn resource_request_rejects_zero_values() {
    assert_eq!(
        ResourceRequest::new(0, 512),
        Err(ComputeDecisionError::InvalidResourceRequest(
            "vcpu debe ser mayor que cero"
        ))
    );
    assert_eq!(
        ResourceRequest::new(1, 0),
        Err(ComputeDecisionError::InvalidResourceRequest(
            "memory_mib debe ser mayor que cero"
        ))
    );
}

#[test]
fn resource_request_checks_limits() {
    let request = ResourceRequest::new(2, 2048).unwrap();
    let limit = ResourceLimit::new(4, 4096).unwrap();
    let small_limit = ResourceLimit::new(1, 4096).unwrap();

    assert!(request.fits_within(limit));
    assert!(!request.fits_within(small_limit));
}

#[test]
fn modes_are_ordered_by_platform_delegation() {
    let levels: Vec<u8> = compute_modes()
        .iter()
        .map(|mode| mode.platform_delegation_level())
        .collect();

    assert_eq!(levels, vec![1, 2, 3, 4, 5]);
}

#[test]
fn virtual_machine_profile_keeps_machine_boundary_and_manual_scaling() {
    let profile = ComputeMode::VirtualMachine.profile();

    assert_eq!(profile.isolation, IsolationBoundary::Machine);
    assert_eq!(profile.lifecycle, LifecyclePolicy::LongRunning);
    assert_eq!(profile.scaling, ScalingSignal::Manual);
}

#[test]
fn function_profile_is_event_driven_provider_runtime() {
    let profile = ComputeMode::Function.profile();

    assert_eq!(profile.isolation, IsolationBoundary::ProviderManagedRuntime);
    assert_eq!(profile.lifecycle, LifecyclePolicy::EventDriven);
    assert_eq!(profile.scaling, ScalingSignal::EventRate);
}

#[test]
fn recommendation_requires_resources() {
    assert_eq!(
        recommend_compute(WorkloadRequirements::default()),
        Err(ComputeDecisionError::MissingResourceRequest)
    );
}

#[test]
fn recommendation_prefers_virtual_machine_for_os_control() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(2, 4096).unwrap()),
        requires_operating_system_control: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(recommend_compute(workload), Ok(ComputeMode::VirtualMachine));
}

#[test]
fn recommendation_prefers_function_for_event_driven_workload() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(1, 512).unwrap()),
        event_driven: true,
        wants_low_operational_load: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(recommend_compute(workload), Ok(ComputeMode::Function));
}

#[test]
fn recommendation_prefers_managed_container_for_container_low_ops() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(1, 1024).unwrap()),
        packaged_as_container: true,
        wants_low_operational_load: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(
        recommend_compute(workload),
        Ok(ComputeMode::ManagedContainer)
    );
}

#[test]
fn recommendation_prefers_batch_for_scheduled_work() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(4, 8192).unwrap()),
        scheduled_or_batch: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(recommend_compute(workload), Ok(ComputeMode::BatchJob));
}

#[test]
fn recommendation_exposes_conflicting_os_and_low_ops_assumptions() {
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
}

#[test]
fn recommendation_rejects_loads_beyond_educational_limit() {
    let workload = WorkloadRequirements {
        resources: Some(ResourceRequest::new(65, 1024).unwrap()),
        packaged_as_container: true,
        ..WorkloadRequirements::default()
    };

    assert_eq!(
        recommend_compute(workload),
        Err(ComputeDecisionError::ExceedsResourceLimit)
    );
}
