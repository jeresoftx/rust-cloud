use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeFinding, AwsPracticeRequirements, AwsService, AwsWorkload,
    CloudConcept, DataLifecycle, NetworkExposure,
};

fn main() {
    let workload = AwsWorkload::new(
        "preview-runner",
        AwsPracticeRequirements {
            concept: CloudConcept::Serverless,
            service: AwsService::Ec2,
            environment: AwsEnvironment::Development,
            region: "us-east-1",
            owner: "",
            purpose: "ejecutar previews",
            least_privilege: false,
            temporary_credentials: false,
            network_exposure: NetworkExposure::PublicUnbounded,
            has_limit: false,
            observability: false,
            cost_tags: false,
            data_lifecycle: DataLifecycle::Indefinite,
            uses_real_credentials: true,
        },
    )
    .unwrap();

    let findings = workload.evaluate().findings().to_vec();

    assert!(
        findings.contains(&AwsPracticeFinding::ServiceDoesNotMatchConcept(
            "preview-runner",
        ))
    );
    assert!(findings.contains(&AwsPracticeFinding::MissingOwner("preview-runner")));
    assert!(findings.contains(&AwsPracticeFinding::BroadPermissions("preview-runner",)));
    assert!(findings.contains(&AwsPracticeFinding::PermanentCredentials("preview-runner",)));
    assert!(
        findings.contains(&AwsPracticeFinding::RealCredentialsInExample(
            "preview-runner",
        ))
    );
    assert!(
        findings.contains(&AwsPracticeFinding::PublicNetworkWithoutBoundary(
            "preview-runner",
        ))
    );
    assert!(findings.contains(&AwsPracticeFinding::MissingLimit("preview-runner")));
    assert!(findings.contains(&AwsPracticeFinding::MissingObservability("preview-runner",)));
    assert!(findings.contains(&AwsPracticeFinding::MissingCostTags("preview-runner")));
    assert!(
        findings.contains(&AwsPracticeFinding::IndefiniteDataLifecycle(
            "preview-runner",
        ))
    );
}
