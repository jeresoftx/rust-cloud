use rust_cloud::networking::{
    Exposure, FirewallRule, NetworkProtocol, NetworkRole, Route, RouteTarget, TrafficEndpoint,
    VirtualNetwork,
};

fn main() {
    let mut network = VirtualNetwork::new("academy-prod", "10.60.0.0/16").unwrap();

    for (name, cidr, role) in [
        ("edge-a", "10.60.1.0/24", NetworkRole::PublicEdge),
        ("app-a", "10.60.10.0/24", NetworkRole::PrivateWorkload),
        ("data-a", "10.60.20.0/24", NetworkRole::IsolatedData),
    ] {
        network.add_subnet(name, cidr, "zone-a", role).unwrap();
    }

    network
        .add_route(
            Route::new(
                "entrada-controlada",
                "0.0.0.0/0",
                RouteTarget::InternetGateway,
                "solo el balanceador público recibe tráfico externo",
            )
            .unwrap(),
        )
        .unwrap();
    network
        .add_route(
            Route::new(
                "salida-privada",
                "10.60.0.0/16",
                RouteTarget::Local,
                "las cargas internas se comunican dentro de la VPC",
            )
            .unwrap(),
        )
        .unwrap();

    for rule in [
        FirewallRule::allow(
            "edge-app-https",
            TrafficEndpoint::Subnet("edge-a"),
            TrafficEndpoint::Subnet("app-a"),
            NetworkProtocol::Tcp,
            443,
            "el borde público invoca la API privada",
        )
        .unwrap(),
        FirewallRule::allow(
            "app-data-postgres",
            TrafficEndpoint::Subnet("app-a"),
            TrafficEndpoint::Subnet("data-a"),
            NetworkProtocol::Tcp,
            5432,
            "la API accede a datos internos",
        )
        .unwrap(),
    ] {
        network.add_firewall_rule(rule).unwrap();
    }

    assert_eq!(network.subnets().len(), 3);
    assert_eq!(network.routes().len(), 2);
    assert_eq!(network.firewall_rules().len(), 2);
    assert_eq!(
        network.exposure_for_subnet("edge-a"),
        Some(Exposure::PubliclyRoutable)
    );
    assert_eq!(
        network.exposure_for_subnet("data-a"),
        Some(Exposure::PrivateOnly)
    );
}
