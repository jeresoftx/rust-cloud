use rust_cloud::service_models::{DecisionContext, DecisionError, recommend_model};

fn main() {
    let context = DecisionContext {
        requires_operating_system_control: true,
        wants_low_operational_load: true,
        ..DecisionContext::default()
    };

    assert_eq!(
        recommend_model(context),
        Err(DecisionError::ConflictingAssumptions(
            "control de sistema operativo y baja carga operativa requieren decidir el tradeoff",
        ))
    );

    println!("La recomendación falla a propósito: falta decidir el tradeoff.");
}
