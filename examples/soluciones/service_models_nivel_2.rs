use rust_cloud::service_models::{DecisionContext, ServiceModel, recommend_model};

fn main() {
    let event_driven = DecisionContext {
        wants_low_operational_load: true,
        event_driven_workload: true,
        ..DecisionContext::default()
    };

    let os_control = DecisionContext {
        requires_operating_system_control: true,
        ..DecisionContext::default()
    };

    assert_eq!(recommend_model(event_driven), Ok(ServiceModel::Serverless));
    assert_eq!(recommend_model(os_control), Ok(ServiceModel::Iaas));

    println!("Serverless cuando pesan eventos y baja operación.");
    println!("IaaS cuando el sistema operativo es parte del control requerido.");
}
