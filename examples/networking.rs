use rust_cloud::networking::{
    Exposure, FirewallRule, NetworkProtocol, NetworkRole, Route, RouteTarget, TrafficEndpoint,
    VirtualNetwork,
};

fn main() {
    let mut network = VirtualNetwork::new("academy-prod", "10.20.0.0/16").unwrap();

    network
        .add_subnet("web-a", "10.20.1.0/24", "zone-a", NetworkRole::PublicEdge)
        .unwrap();
    network
        .add_subnet(
            "orders-a",
            "10.20.2.0/24",
            "zone-a",
            NetworkRole::PrivateWorkload,
        )
        .unwrap();
    network
        .add_route(
            Route::new(
                "salida-publica",
                "0.0.0.0/0",
                RouteTarget::InternetGateway,
                "ingreso controlado por balanceador",
            )
            .unwrap(),
        )
        .unwrap();

    let public_https = FirewallRule::allow(
        "https-publico",
        TrafficEndpoint::Internet,
        TrafficEndpoint::Subnet("web-a"),
        NetworkProtocol::Tcp,
        443,
        "permitir HTTPS público",
    )
    .unwrap();
    network.add_firewall_rule(public_https).unwrap();

    let app_traffic = FirewallRule::allow(
        "web-a-orders",
        TrafficEndpoint::Subnet("web-a"),
        TrafficEndpoint::Subnet("orders-a"),
        NetworkProtocol::Tcp,
        443,
        "API interna detrás del borde público",
    )
    .unwrap();
    network.add_firewall_rule(app_traffic).unwrap();

    assert_eq!(
        network.exposure_for_subnet("web-a"),
        Some(Exposure::PubliclyRoutable)
    );
    assert_eq!(
        network.exposure_for_subnet("orders-a"),
        Some(Exposure::PrivateOnly)
    );
    assert_eq!(network.firewall_rules().len(), 2);
}
