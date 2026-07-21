use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile, ServerlessDecisionError,
    ServerlessFinding, ServerlessRequirements, ServerlessWorkload, StateAccess, TriggerKind,
};

#[test]
fn queue_handler_with_idempotency_and_limits_is_low_risk() {
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

    assert_eq!(workload.name(), "process-publication");
    assert!(workload.evaluate().is_low_risk());
}

#[test]
fn retrying_stateful_function_without_idempotency_is_visible_risk() {
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
    let evaluation = workload.evaluate();

    assert!(
        evaluation
            .findings()
            .contains(&ServerlessFinding::RetryWithoutIdempotency(
                "charge-payment"
            ),)
    );
    assert!(
        evaluation
            .findings()
            .contains(&ServerlessFinding::StatefulWriteWithoutIdempotency(
                "charge-payment"
            ),)
    );
    assert!(
        evaluation
            .findings()
            .contains(&ServerlessFinding::UnboundedConcurrency("charge-payment"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&ServerlessFinding::HighFunctionTimeout("charge-payment"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&ServerlessFinding::MissingObservability("charge-payment"))
    );
}

#[test]
fn workload_requires_name_purpose_and_positive_timeout() {
    let requirements = ServerlessRequirements {
        trigger: TriggerKind::Schedule,
        runtime: RuntimeProfile::Function,
        timeout_seconds: Some(1),
        max_concurrency: Some(1),
        retry_policy: RetryPolicy::None,
        idempotency: IdempotencyStrategy::None,
        state_access: StateAccess::Stateless,
        observability: ObservabilityPlan::standard(),
        purpose: "limpiar sesiones expiradas",
    };

    assert_eq!(
        ServerlessWorkload::new("", requirements),
        Err(ServerlessDecisionError::MissingName)
    );

    assert_eq!(
        ServerlessWorkload::new(
            "cleanup",
            ServerlessRequirements {
                purpose: "",
                ..requirements
            },
        ),
        Err(ServerlessDecisionError::MissingPurpose)
    );

    assert_eq!(
        ServerlessWorkload::new(
            "cleanup",
            ServerlessRequirements {
                timeout_seconds: Some(0),
                ..requirements
            },
        ),
        Err(ServerlessDecisionError::InvalidTimeout(
            "timeout_seconds debe ser mayor que cero",
        ))
    );
}
