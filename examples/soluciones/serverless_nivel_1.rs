use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile, ServerlessRequirements,
    ServerlessWorkload, StateAccess, TriggerKind,
};

fn main() {
    let workload = ServerlessWorkload::new(
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

    assert!(workload.evaluate().is_low_risk());
}
