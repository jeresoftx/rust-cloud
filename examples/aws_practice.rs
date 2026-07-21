use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeFinding, AwsPracticeRequirements, AwsService, AwsWorkload,
    CloudConcept, DataLifecycle, NetworkExposure,
};

fn main() {
    let assets = AwsWorkload::new(
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

    let preview_runner = AwsWorkload::new(
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

    assert!(assets.evaluate().is_low_risk());
    print_evaluation(&assets);

    let preview_findings = preview_runner.evaluate().findings().to_vec();
    assert!(
        preview_findings.contains(&AwsPracticeFinding::ServiceDoesNotMatchConcept(
            "preview-runner",
        ))
    );
    assert!(
        preview_findings.contains(&AwsPracticeFinding::RealCredentialsInExample(
            "preview-runner",
        ))
    );
    print_evaluation(&preview_runner);
}

fn print_evaluation(workload: &AwsWorkload) {
    let evaluation = workload.evaluate();

    if evaluation.is_low_risk() {
        println!("{}: mapeo AWS gobernable", workload.name());
    } else {
        println!(
            "{}: {} hallazgos educativos",
            workload.name(),
            evaluation.findings().len()
        );
    }
}
