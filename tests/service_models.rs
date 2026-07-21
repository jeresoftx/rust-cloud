use rust_cloud::service_models::{
    DecisionContext, DecisionError, ResponsibilityLayer, ResponsibilityOwner, ServiceModel,
    recommend_model, responsibility_layers, service_models,
};

#[test]
fn every_model_declares_every_responsibility_layer() {
    for model in service_models() {
        let profile = model.profile();

        for layer in responsibility_layers() {
            assert!(
                profile.owner_of(*layer).is_some(),
                "{model:?} does not declare {layer:?}"
            );
        }
    }
}

#[test]
fn provider_keeps_physical_layers_in_every_model() {
    let physical_layers = [
        ResponsibilityLayer::PhysicalFacilities,
        ResponsibilityLayer::PhysicalNetwork,
        ResponsibilityLayer::PhysicalCompute,
    ];

    for model in service_models() {
        let profile = model.profile();

        for layer in physical_layers {
            assert_eq!(profile.owner_of(layer), Some(ResponsibilityOwner::Provider));
        }
    }
}

#[test]
fn iaas_keeps_operating_system_with_the_team() {
    let profile = ServiceModel::Iaas.profile();

    assert_eq!(
        profile.owner_of(ResponsibilityLayer::OperatingSystem),
        Some(ResponsibilityOwner::Team)
    );
    assert_eq!(
        profile.owner_of(ResponsibilityLayer::Runtime),
        Some(ResponsibilityOwner::Team)
    );
}

#[test]
fn saas_delegates_application_code_to_the_provider() {
    let profile = ServiceModel::Saas.profile();

    assert_eq!(
        profile.owner_of(ResponsibilityLayer::ApplicationCode),
        Some(ResponsibilityOwner::Provider)
    );
    assert_eq!(
        profile.owner_of(ResponsibilityLayer::ApplicationData),
        Some(ResponsibilityOwner::Shared)
    );
}

#[test]
fn models_are_ordered_by_growing_abstraction() {
    let levels: Vec<u8> = service_models()
        .iter()
        .map(|model| model.abstraction_level())
        .collect();

    assert_eq!(levels, vec![1, 2, 3, 4]);
}

#[test]
fn team_owned_layers_shrink_when_more_operation_is_delegated() {
    let iaas_layers = ServiceModel::Iaas.profile().team_owned_layers().count();
    let paas_layers = ServiceModel::Paas.profile().team_owned_layers().count();
    let saas_layers = ServiceModel::Saas.profile().team_owned_layers().count();

    assert!(iaas_layers > paas_layers);
    assert!(paas_layers > saas_layers);
}

#[test]
fn recommendation_requires_assumptions() {
    assert_eq!(
        recommend_model(DecisionContext::default()),
        Err(DecisionError::MissingAssumptions)
    );
}

#[test]
fn recommendation_prefers_iaas_when_os_control_is_required() {
    let context = DecisionContext {
        requires_operating_system_control: true,
        ..DecisionContext::default()
    };

    assert_eq!(recommend_model(context), Ok(ServiceModel::Iaas));
}

#[test]
fn recommendation_prefers_serverless_for_event_driven_low_ops() {
    let context = DecisionContext {
        wants_low_operational_load: true,
        event_driven_workload: true,
        ..DecisionContext::default()
    };

    assert_eq!(recommend_model(context), Ok(ServiceModel::Serverless));
}

#[test]
fn recommendation_exposes_conflicting_os_and_low_ops_assumptions() {
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
}
