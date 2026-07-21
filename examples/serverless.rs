use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile, ServerlessFinding,
    ServerlessRequirements, ServerlessWorkload, StateAccess, TriggerKind,
};

fn main() {
    let publication = ServerlessWorkload::new(
        "process-publication",
        ServerlessRequirements {
            trigger: TriggerKind::Queue,
            runtime: RuntimeProfile::Function,
            timeout_seconds: Some(30),
            max_concurrency: Some(50),
            retry_policy: RetryPolicy::Backoff { attempts: 3 },
            idempotency: IdempotencyStrategy::EventKey,
            state_access: StateAccess::ExternalWrite,
            observability: ObservabilityPlan::standard(),
            purpose: "procesar eventos de publicación de contenido",
        },
    )
    .unwrap();

    assert!(publication.evaluate().is_low_risk());
    println!("{}: flujo serverless acotado", publication.name());

    let payment = ServerlessWorkload::new(
        "charge-payment",
        ServerlessRequirements {
            trigger: TriggerKind::Http,
            runtime: RuntimeProfile::Function,
            timeout_seconds: Some(120),
            max_concurrency: None,
            retry_policy: RetryPolicy::Fixed { attempts: 2 },
            idempotency: IdempotencyStrategy::None,
            state_access: StateAccess::ExternalWrite,
            observability: ObservabilityPlan::none(),
            purpose: "registrar pago de estudiante",
        },
    )
    .unwrap();

    let findings = payment.evaluate().findings().to_vec();
    assert!(
        findings.contains(&ServerlessFinding::RetryWithoutIdempotency(
            "charge-payment",
        ))
    );
    println!(
        "{}: {} hallazgos educativos",
        payment.name(),
        findings.len()
    );
}
