use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeDecisionError, AwsPracticeFinding, AwsPracticeRequirements,
    AwsService, AwsWorkload, CloudConcept, DataLifecycle, NetworkExposure,
};

#[test]
fn private_s3_assets_with_tags_and_lifecycle_are_low_risk() {
    let workload = AwsWorkload::new(
        "academy-assets",
        AwsPracticeRequirements {
            concept: CloudConcept::Storage,
            service: AwsService::S3,
            environment: AwsEnvironment::Production,
            region: "us-east-1",
            owner: "equipo academy",
            purpose: "guardar assets publicados del curso",
            least_privilege: true,
            temporary_credentials: true,
            network_exposure: NetworkExposure::Private,
            has_limit: true,
            observability: true,
            cost_tags: true,
            data_lifecycle: DataLifecycle::Retained,
            uses_real_credentials: false,
        },
    )
    .unwrap();

    assert_eq!(workload.name(), "academy-assets");
    assert!(workload.evaluate().is_low_risk());
}

#[test]
fn risky_lambda_example_exposes_architecture_and_security_findings() {
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
    let evaluation = workload.evaluate();

    assert!(
        evaluation
            .findings()
            .contains(&AwsPracticeFinding::ServiceDoesNotMatchConcept(
                "preview-runner"
            ))
    );
    assert!(
        evaluation
            .findings()
            .contains(&AwsPracticeFinding::MissingOwner("preview-runner"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&AwsPracticeFinding::BroadPermissions("preview-runner"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&AwsPracticeFinding::RealCredentialsInExample(
                "preview-runner"
            ))
    );
    assert!(
        evaluation
            .findings()
            .contains(&AwsPracticeFinding::MissingCostTags("preview-runner"))
    );
}

#[test]
fn workload_requires_name_and_region() {
    let requirements = AwsPracticeRequirements {
        concept: CloudConcept::FinOps,
        service: AwsService::Budgets,
        environment: AwsEnvironment::Staging,
        region: "us-east-1",
        owner: "equipo academy",
        purpose: "alertar gasto de staging",
        least_privilege: true,
        temporary_credentials: true,
        network_exposure: NetworkExposure::Private,
        has_limit: true,
        observability: true,
        cost_tags: true,
        data_lifecycle: DataLifecycle::NotApplicable,
        uses_real_credentials: false,
    };

    assert_eq!(
        AwsWorkload::new("", requirements),
        Err(AwsPracticeDecisionError::MissingName)
    );

    assert_eq!(
        AwsWorkload::new(
            "staging-budget",
            AwsPracticeRequirements {
                region: "",
                ..requirements
            },
        ),
        Err(AwsPracticeDecisionError::MissingRegion)
    );
}
