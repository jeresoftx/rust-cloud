use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpFinding, GcpNetworkExposure, GcpPracticeRequirements,
    GcpService, GcpWorkload,
};

fn main() {
    let api = GcpWorkload::new(
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

    let preview_runner = GcpWorkload::new(
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

    assert!(api.evaluate().is_low_risk());
    print_evaluation(&api);

    let preview_findings = preview_runner.evaluate().findings().to_vec();
    assert!(
        preview_findings.contains(&GcpFinding::ServiceDoesNotMatchCapability("preview-runner",))
    );
    assert!(preview_findings.contains(&GcpFinding::RealCredentialsInExample("preview-runner",)));
    print_evaluation(&preview_runner);
}

fn print_evaluation(workload: &GcpWorkload) {
    let evaluation = workload.evaluate();

    if evaluation.is_low_risk() {
        println!("{}: mapeo GCP gobernable", workload.name());
    } else {
        println!(
            "{}: {} hallazgos educativos",
            workload.name(),
            evaluation.findings().len()
        );
    }
}
