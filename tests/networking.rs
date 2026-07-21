use rust_cloud::networking::{
    CidrBlock, Exposure, FirewallRule, NetworkProtocol, NetworkRole, NetworkingDecisionError,
    Route, RouteTarget, TrafficEndpoint, VirtualNetwork,
};

#[test]
fn vpc_separates_public_and_private_subnets_by_route() {
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

    assert_eq!(
        network.exposure_for_subnet("web-a"),
        Some(Exposure::PubliclyRoutable)
    );
    assert_eq!(
        network.exposure_for_subnet("orders-a"),
        Some(Exposure::PrivateOnly)
    );
}

#[test]
fn vpc_rejects_duplicate_or_overlapping_subnet_ranges() {
    let mut network = VirtualNetwork::new("academy-prod", "10.30.0.0/16").unwrap();

    network
        .add_subnet("web-a", "10.30.1.0/24", "zone-a", NetworkRole::PublicEdge)
        .unwrap();

    assert_eq!(
        network.add_subnet("web-b", "10.30.1.0/24", "zone-b", NetworkRole::PublicEdge),
        Err(NetworkingDecisionError::OverlappingSubnet {
            existing: CidrBlock::new("10.30.1.0/24").unwrap(),
            candidate: CidrBlock::new("10.30.1.0/24").unwrap(),
        })
    );
}

#[test]
fn route_requires_destination_and_purpose() {
    assert_eq!(
        Route::new(
            "ruta-sin-destino",
            "",
            RouteTarget::NatGateway,
            "salida privada"
        ),
        Err(NetworkingDecisionError::MissingDestination)
    );

    assert_eq!(
        Route::new(
            "ruta-sin-proposito",
            "0.0.0.0/0",
            RouteTarget::NatGateway,
            ""
        ),
        Err(NetworkingDecisionError::MissingPurpose)
    );
}

#[test]
fn firewall_rejects_public_ssh_as_default_design() {
    let rule = FirewallRule::allow(
        "ssh-publico",
        TrafficEndpoint::Internet,
        TrafficEndpoint::Subnet("web-a"),
        NetworkProtocol::Tcp,
        22,
        "atajo operativo",
    );

    assert_eq!(
        rule,
        Err(NetworkingDecisionError::PublicSshExposure(
            "no publiques SSH a Internet como diseño base",
        ))
    );
}

#[test]
fn firewall_keeps_intention_visible_for_private_traffic() {
    let rule = FirewallRule::allow(
        "api-a-orders",
        TrafficEndpoint::Subnet("web-a"),
        TrafficEndpoint::Subnet("orders-a"),
        NetworkProtocol::Tcp,
        443,
        "API interna detrás del balanceador",
    )
    .unwrap();

    assert_eq!(rule.port(), 443);
    assert_eq!(rule.protocol(), NetworkProtocol::Tcp);
    assert_eq!(rule.purpose(), "API interna detrás del balanceador");
}
