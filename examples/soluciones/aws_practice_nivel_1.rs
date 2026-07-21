use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeRequirements, AwsService, AwsWorkload, CloudConcept, DataLifecycle,
    NetworkExposure,
};

fn main() {
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

    assert!(workload.evaluate().is_low_risk());
}
