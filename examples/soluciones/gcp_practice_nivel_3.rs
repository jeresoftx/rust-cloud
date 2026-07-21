use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpNetworkExposure, GcpPracticeRequirements, GcpService,
    GcpWorkload,
};

fn main() {
    let workloads = [
        GcpWorkload::new(
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
        .unwrap(),
        GcpWorkload::new(
            "academy-assets",
            GcpPracticeRequirements {
                capability: CloudCapability::Storage,
                service: GcpService::CloudStorage,
                project: "academy-prod",
                region: "us-central1",
                environment: GcpEnvironment::Production,
                owner: "equipo academy",
                purpose: "guardar assets publicados del curso",
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
        .unwrap(),
        GcpWorkload::new(
            "staging-budget",
            GcpPracticeRequirements {
                capability: CloudCapability::FinOps,
                service: GcpService::Budgets,
                project: "academy-staging",
                region: "us-central1",
                environment: GcpEnvironment::Staging,
                owner: "equipo academy",
                purpose: "alertar gasto de staging",
                least_privilege: true,
                managed_identity: true,
                network_exposure: GcpNetworkExposure::InternalOnly,
                has_limit: true,
                observability: true,
                cost_labels: true,
                lifecycle_policy: true,
                uses_real_credentials: false,
            },
        )
        .unwrap(),
    ];

    assert!(
        workloads
            .iter()
            .all(|workload| workload.evaluate().is_low_risk())
    );
}
