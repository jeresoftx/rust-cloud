use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile, ServerlessFinding,
    ServerlessRequirements, ServerlessWorkload, StateAccess, TriggerKind,
};

fn main() {
    let workload = ServerlessWorkload::new(
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
    let findings = workload.evaluate().findings().to_vec();

    assert!(
        findings.contains(&ServerlessFinding::RetryWithoutIdempotency(
            "charge-payment",
        ))
    );
    assert!(
        findings.contains(&ServerlessFinding::StatefulWriteWithoutIdempotency(
            "charge-payment",
        ))
    );
    assert!(findings.contains(&ServerlessFinding::UnboundedConcurrency("charge-payment",)));
    assert!(findings.contains(&ServerlessFinding::HighFunctionTimeout("charge-payment",)));
    assert!(findings.contains(&ServerlessFinding::MissingObservability("charge-payment",)));
}
