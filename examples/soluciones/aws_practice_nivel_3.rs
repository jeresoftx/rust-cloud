use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeRequirements, AwsService, AwsWorkload, CloudConcept, DataLifecycle,
    NetworkExposure,
};

fn main() {
    let workloads = [
        AwsWorkload::new(
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
        .unwrap(),
        AwsWorkload::new(
            "academy-vpc",
            AwsPracticeRequirements {
                concept: CloudConcept::Networking,
                service: AwsService::Vpc,
                environment: AwsEnvironment::Production,
                region: "us-east-1",
                owner: "equipo plataforma",
                purpose: "separar tráfico público y privado",
                least_privilege: true,
                temporary_credentials: true,
                network_exposure: NetworkExposure::Private,
                has_limit: true,
                observability: true,
                cost_tags: true,
                data_lifecycle: DataLifecycle::NotApplicable,
                uses_real_credentials: false,
            },
        )
        .unwrap(),
        AwsWorkload::new(
            "staging-budget",
            AwsPracticeRequirements {
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
