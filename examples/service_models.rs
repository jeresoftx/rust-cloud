use rust_cloud::service_models::{
    DecisionContext, ResponsibilityLayer, ResponsibilityOwner, ServiceModel, recommend_model,
    service_models,
};

fn main() {
    println!("Modelos de servicio como contratos de responsabilidad\n");

    for model in service_models() {
        let profile = model.profile();
        let team_layers = profile.team_owned_layers().count();

        println!(
            "- {model:?}: control={:?}, operación={:?}, portabilidad={:?}, capas del equipo={team_layers}",
            profile.team_control, profile.team_operational_load, profile.portability
        );
    }

    let context = DecisionContext {
        wants_low_operational_load: true,
        event_driven_workload: true,
        ..DecisionContext::default()
    };

    println!(
        "\nRecomendación educativa para carga orientada a eventos con baja operación: {:?}",
        recommend_model(context)
    );

    let iaas = ServiceModel::Iaas.profile();
    let operating_system_owner = iaas
        .owner_of(ResponsibilityLayer::OperatingSystem)
        .unwrap_or(ResponsibilityOwner::Shared);

    println!("En IaaS, el sistema operativo queda en: {operating_system_owner:?}");
}
