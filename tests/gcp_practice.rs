use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpFinding, GcpNetworkExposure, GcpPracticeDecisionError,
    GcpPracticeRequirements, GcpService, GcpWorkload,
};

#[test]
fn cloud_run_api_with_labels_and_service_account_is_low_risk() {
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

    assert_eq!(workload.name(), "academy-api");
    assert!(workload.evaluate().is_low_risk());
}

#[test]
fn risky_function_example_exposes_identity_network_and_cost_findings() {
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
    assert!(findings.contains(&GcpFinding::PublicNetworkWithoutBoundary("preview-runner",)));
    assert!(findings.contains(&GcpFinding::MissingCostLabels("preview-runner")));
    assert!(findings.contains(&GcpFinding::RealCredentialsInExample("preview-runner",)));
}

#[test]
fn workload_requires_name_project_and_region() {
    let requirements = GcpPracticeRequirements {
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
    };

    assert_eq!(
        GcpWorkload::new("", requirements),
        Err(GcpPracticeDecisionError::MissingName)
    );
    assert_eq!(
        GcpWorkload::new(
            "staging-budget",
            GcpPracticeRequirements {
                project: "",
                ..requirements
            },
        ),
        Err(GcpPracticeDecisionError::MissingProject)
    );
    assert_eq!(
        GcpWorkload::new(
            "staging-budget",
            GcpPracticeRequirements {
                region: "",
                ..requirements
            },
        ),
        Err(GcpPracticeDecisionError::MissingRegion)
    );
}
