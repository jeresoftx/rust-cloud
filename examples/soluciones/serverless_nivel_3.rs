use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile, ServerlessRequirements,
    ServerlessWorkload, StateAccess, TriggerKind,
};

fn main() {
    let cleanup = ServerlessWorkload::new(
        "cleanup-expired-sessions",
        ServerlessRequirements {
            trigger: TriggerKind::Schedule,
            runtime: RuntimeProfile::Function,
            timeout_seconds: Some(20),
            max_concurrency: Some(1),
            retry_policy: RetryPolicy::None,
            idempotency: IdempotencyStrategy::None,
            state_access: StateAccess::ExternalRead,
            observability: ObservabilityPlan::standard(),
            purpose: "limpiar sesiones expiradas",
        },
    )
    .unwrap();

    let workflow = ServerlessWorkload::new(
        "publish-course-workflow",
        ServerlessRequirements {
            trigger: TriggerKind::DomainEvent,
            runtime: RuntimeProfile::Workflow,
            timeout_seconds: Some(300),
            max_concurrency: Some(10),
            retry_policy: RetryPolicy::Backoff { attempts: 3 },
            idempotency: IdempotencyStrategy::Compensation,
            state_access: StateAccess::ExternalWrite,
            observability: ObservabilityPlan::standard(),
            purpose: "orquestar publicación de capítulo validado",
        },
    )
    .unwrap();

    assert!(cleanup.evaluate().is_low_risk());
    assert!(workflow.evaluate().is_low_risk());
}
