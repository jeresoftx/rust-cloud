use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpFinding, GcpNetworkExposure, GcpPracticeRequirements,
    GcpService, GcpWorkload,
};

fn main() {
    let workload = GcpWorkload::new(
        "preview-runner",
        GcpPracticeRequirements {
            capability: CloudCapability::Serverless,
            service: GcpService::ComputeEngine,
            project: "academy-dev",
            region: "us-central1",
            environment: GcpEnvironment::Development,
            owner: "",
            purpose: "ejecutar previews",
            least_privilege: false,
            managed_identity: false,
            network_exposure: GcpNetworkExposure::PublicUnbounded,
            has_limit: false,
            observability: false,
            cost_labels: false,
            lifecycle_policy: false,
            uses_real_credentials: true,
        },
    )
    .unwrap();

    let findings = workload.evaluate().findings().to_vec();

    assert!(findings.contains(&GcpFinding::ServiceDoesNotMatchCapability("preview-runner",)));
    assert!(findings.contains(&GcpFinding::MissingOwner("preview-runner")));
    assert!(findings.contains(&GcpFinding::BroadPermissions("preview-runner")));
    assert!(
        findings.contains(&GcpFinding::PermanentOrExportedCredentials(
            "preview-runner",
        ))
    );
    assert!(findings.contains(&GcpFinding::RealCredentialsInExample("preview-runner",)));
    assert!(findings.contains(&GcpFinding::PublicNetworkWithoutBoundary("preview-runner",)));
    assert!(findings.contains(&GcpFinding::MissingLimit("preview-runner")));
    assert!(findings.contains(&GcpFinding::MissingObservability("preview-runner")));
    assert!(findings.contains(&GcpFinding::MissingCostLabels("preview-runner")));
    assert!(findings.contains(&GcpFinding::MissingLifecyclePolicy("preview-runner")));
}
