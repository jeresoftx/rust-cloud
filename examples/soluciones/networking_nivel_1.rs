use rust_cloud::networking::{Exposure, NetworkRole, Route, RouteTarget, VirtualNetwork};

fn main() {
    let mut network = VirtualNetwork::new("academy-prod", "10.40.0.0/16").unwrap();

    network
        .add_subnet("edge-a", "10.40.1.0/24", "zone-a", NetworkRole::PublicEdge)
        .unwrap();
    network
        .add_subnet(
            "app-a",
            "10.40.10.0/24",
            "zone-a",
            NetworkRole::PrivateWorkload,
        )
        .unwrap();
    network
        .add_route(
            Route::new(
                "entrada-controlada",
                "0.0.0.0/0",
                RouteTarget::InternetGateway,
                "solo el borde público recibe tráfico externo",
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(
        network.exposure_for_subnet("edge-a"),
        Some(Exposure::PubliclyRoutable)
    );
    assert_eq!(
        network.exposure_for_subnet("app-a"),
        Some(Exposure::PrivateOnly)
    );
}
