use rust_cloud::networking::{
    CidrBlock, FirewallRule, NetworkProtocol, NetworkRole, NetworkingDecisionError,
    TrafficEndpoint, VirtualNetwork,
};

fn main() {
    let mut network = VirtualNetwork::new("academy-prod", "10.50.0.0/16").unwrap();

    network
        .add_subnet("edge-a", "10.50.1.0/24", "zone-a", NetworkRole::PublicEdge)
        .unwrap();

    assert_eq!(
        network.add_subnet("edge-b", "10.50.1.0/24", "zone-b", NetworkRole::PublicEdge),
        Err(NetworkingDecisionError::OverlappingSubnet {
            existing: CidrBlock::new("10.50.1.0/24").unwrap(),
            candidate: CidrBlock::new("10.50.1.0/24").unwrap(),
        })
    );

    assert_eq!(
        FirewallRule::allow(
            "ssh-publico",
            TrafficEndpoint::Internet,
            TrafficEndpoint::Subnet("edge-a"),
            NetworkProtocol::Tcp,
            22,
            "atajo operativo",
        ),
        Err(NetworkingDecisionError::PublicSshExposure(
            "no publiques SSH a Internet como diseño base",
        ))
    );
}
