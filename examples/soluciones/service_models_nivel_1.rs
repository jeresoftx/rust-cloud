use rust_cloud::service_models::{ResponsibilityLayer, ServiceModel};

fn main() {
    let profile = ServiceModel::Iaas.profile();
    let layers = [
        ResponsibilityLayer::PhysicalFacilities,
        ResponsibilityLayer::OperatingSystem,
        ResponsibilityLayer::Runtime,
        ResponsibilityLayer::ApplicationCode,
        ResponsibilityLayer::ScalingPolicy,
    ];

    for layer in layers {
        let owner = profile
            .owner_of(layer)
            .expect("toda capa del ejercicio debe existir en el perfil IaaS");

        println!("{layer:?}: {owner:?}");
    }
}
