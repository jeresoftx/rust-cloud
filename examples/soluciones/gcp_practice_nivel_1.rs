use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpNetworkExposure, GcpPracticeRequirements, GcpService,
    GcpWorkload,
};

fn main() {
    let workload = GcpWorkload::new(
        "academy-api",
        GcpPracticeRequirements {
            capability: CloudCapability::Compute,
            service: GcpService::CloudRun,
            project: "academy-prod",
            region: "us-central1",
            environment: GcpEnvironment::Production,
            owner: "equipo academy",
            purpose: "servir rutas de aprendizaje a estudiantes",
            least_privilege: true,
            managed_identity: true,
            network_exposure: GcpNetworkExposure::PublicEntrypoint,
            has_limit: true,
            observability: true,
            cost_labels: true,
            lifecycle_policy: true,
            uses_real_credentials: false,
        },
    )
    .unwrap();

    assert!(workload.evaluate().is_low_risk());
}
