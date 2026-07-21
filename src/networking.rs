//! Redes cloud como contrato explícito de alcance y confianza.
//!
//! Este módulo no simula paquetes reales ni reemplaza herramientas de red. Hace
//! visibles decisiones educativas: frontera de red, subredes, rutas, reglas de
//! tráfico, exposición pública y conectividad privada.

/// CIDR educativo usado para declarar rangos de red.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CidrBlock {
    value: &'static str,
}

/// Rol de una subred dentro de una red virtual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkRole {
    /// Borde público controlado, normalmente balanceadores o gateways.
    PublicEdge,
    /// Carga privada que no debe recibir ingreso público directo.
    PrivateWorkload,
    /// Datos o servicios internos con alcance restringido.
    IsolatedData,
}

/// Destino o siguiente salto de una ruta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTarget {
    /// Internet Gateway o equivalente.
    InternetGateway,
    /// NAT Gateway o salida administrada.
    NatGateway,
    /// Ruta local dentro de la red virtual.
    Local,
    /// Conectividad privada entre redes, cuentas o proyectos.
    PrivateConnectivity,
}

/// Exposición efectiva de una subred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// La subred tiene camino público explícito.
    PubliclyRoutable,
    /// La subred queda privada para el modelo educativo.
    PrivateOnly,
}

/// Protocolo de una regla de firewall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    /// TCP.
    Tcp,
    /// UDP.
    Udp,
    /// ICMP.
    Icmp,
}

/// Endpoint educativo para reglas de tráfico.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficEndpoint {
    /// Internet pública.
    Internet,
    /// Una subred por nombre.
    Subnet(&'static str),
    /// Un rango CIDR.
    Cidr(CidrBlock),
}

/// Error educativo al modelar redes cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkingDecisionError {
    /// Falta nombre.
    MissingName,
    /// Falta rango CIDR o destino.
    MissingDestination,
    /// Falta zona.
    MissingZone,
    /// Falta propósito humano.
    MissingPurpose,
    /// El CIDR no tiene una forma mínima aceptable.
    InvalidCidr(&'static str),
    /// Una subred reutiliza un rango ya declarado.
    OverlappingSubnet {
        /// Rango existente.
        existing: CidrBlock,
        /// Rango candidato.
        candidate: CidrBlock,
    },
    /// Diseño base inseguro: SSH público.
    PublicSshExposure(&'static str),
}

/// Subred de una red virtual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subnet {
    name: &'static str,
    cidr: CidrBlock,
    zone: &'static str,
    role: NetworkRole,
}

/// Ruta declarada dentro de una red virtual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    name: &'static str,
    destination: CidrBlock,
    target: RouteTarget,
    purpose: &'static str,
}

/// Regla de firewall educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirewallRule {
    name: &'static str,
    source: TrafficEndpoint,
    destination: TrafficEndpoint,
    protocol: NetworkProtocol,
    port: u16,
    purpose: &'static str,
}

/// Red virtual educativa con subredes, rutas y reglas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualNetwork {
    name: &'static str,
    cidr: CidrBlock,
    subnets: Vec<Subnet>,
    routes: Vec<Route>,
    firewall_rules: Vec<FirewallRule>,
}

impl CidrBlock {
    /// Crea un bloque CIDR con validación mínima de forma.
    ///
    /// ```
    /// let cidr = rust_cloud::networking::CidrBlock::new("10.0.0.0/16").unwrap();
    /// assert_eq!(cidr.as_str(), "10.0.0.0/16");
    /// ```
    pub fn new(value: &'static str) -> Result<Self, NetworkingDecisionError> {
        if value.is_empty() {
            return Err(NetworkingDecisionError::MissingDestination);
        }

        let Some((address, prefix)) = value.split_once('/') else {
            return Err(NetworkingDecisionError::InvalidCidr(value));
        };

        let prefix = prefix
            .parse::<u8>()
            .map_err(|_| NetworkingDecisionError::InvalidCidr(value))?;
        let octets = address.split('.').count();

        if octets != 4 || prefix > 32 {
            return Err(NetworkingDecisionError::InvalidCidr(value));
        }

        Ok(Self { value })
    }

    /// Devuelve el texto CIDR original.
    pub const fn as_str(self) -> &'static str {
        self.value
    }
}

impl Route {
    /// Crea una ruta educativa.
    pub fn new(
        name: &'static str,
        destination: &'static str,
        target: RouteTarget,
        purpose: &'static str,
    ) -> Result<Self, NetworkingDecisionError> {
        if name.is_empty() {
            return Err(NetworkingDecisionError::MissingName);
        }

        if destination.is_empty() {
            return Err(NetworkingDecisionError::MissingDestination);
        }

        if purpose.is_empty() {
            return Err(NetworkingDecisionError::MissingPurpose);
        }

        Ok(Self {
            name,
            destination: CidrBlock::new(destination)?,
            target,
            purpose,
        })
    }

    /// Destino de la ruta.
    pub const fn destination(&self) -> CidrBlock {
        self.destination
    }

    /// Siguiente salto educativo.
    pub const fn target(&self) -> RouteTarget {
        self.target
    }

    /// Propósito humano.
    pub const fn purpose(&self) -> &'static str {
        self.purpose
    }
}

impl FirewallRule {
    /// Crea una regla allow con validaciones educativas.
    pub fn allow(
        name: &'static str,
        source: TrafficEndpoint,
        destination: TrafficEndpoint,
        protocol: NetworkProtocol,
        port: u16,
        purpose: &'static str,
    ) -> Result<Self, NetworkingDecisionError> {
        if name.is_empty() {
            return Err(NetworkingDecisionError::MissingName);
        }

        if purpose.is_empty() {
            return Err(NetworkingDecisionError::MissingPurpose);
        }

        if source == TrafficEndpoint::Internet && protocol == NetworkProtocol::Tcp && port == 22 {
            return Err(NetworkingDecisionError::PublicSshExposure(
                "no publiques SSH a Internet como diseño base",
            ));
        }

        Ok(Self {
            name,
            source,
            destination,
            protocol,
            port,
            purpose,
        })
    }

    /// Protocolo permitido.
    pub const fn protocol(&self) -> NetworkProtocol {
        self.protocol
    }

    /// Puerto permitido.
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Propósito humano.
    pub const fn purpose(&self) -> &'static str {
        self.purpose
    }
}

impl VirtualNetwork {
    /// Crea una red virtual educativa.
    pub fn new(name: &'static str, cidr: &'static str) -> Result<Self, NetworkingDecisionError> {
        if name.is_empty() {
            return Err(NetworkingDecisionError::MissingName);
        }

        Ok(Self {
            name,
            cidr: CidrBlock::new(cidr)?,
            subnets: Vec::new(),
            routes: Vec::new(),
            firewall_rules: Vec::new(),
        })
    }

    /// Agrega una subred.
    pub fn add_subnet(
        &mut self,
        name: &'static str,
        cidr: &'static str,
        zone: &'static str,
        role: NetworkRole,
    ) -> Result<(), NetworkingDecisionError> {
        if name.is_empty() {
            return Err(NetworkingDecisionError::MissingName);
        }

        if zone.is_empty() {
            return Err(NetworkingDecisionError::MissingZone);
        }

        let candidate = CidrBlock::new(cidr)?;
        if let Some(existing) = self
            .subnets
            .iter()
            .find(|subnet| subnet.cidr == candidate)
            .map(|subnet| subnet.cidr)
        {
            return Err(NetworkingDecisionError::OverlappingSubnet {
                existing,
                candidate,
            });
        }

        self.subnets.push(Subnet {
            name,
            cidr: candidate,
            zone,
            role,
        });

        Ok(())
    }

    /// Agrega una ruta.
    pub fn add_route(&mut self, route: Route) -> Result<(), NetworkingDecisionError> {
        self.routes.push(route);
        Ok(())
    }

    /// Agrega una regla de firewall.
    pub fn add_firewall_rule(&mut self, rule: FirewallRule) -> Result<(), NetworkingDecisionError> {
        self.firewall_rules.push(rule);
        Ok(())
    }

    /// Calcula exposición efectiva de una subred.
    pub fn exposure_for_subnet(&self, subnet_name: &str) -> Option<Exposure> {
        let subnet = self
            .subnets
            .iter()
            .find(|subnet| subnet.name == subnet_name)?;
        let has_internet_route = self
            .routes
            .iter()
            .any(|route| route.target == RouteTarget::InternetGateway);

        if subnet.role == NetworkRole::PublicEdge && has_internet_route {
            Some(Exposure::PubliclyRoutable)
        } else {
            Some(Exposure::PrivateOnly)
        }
    }

    /// Nombre de la red.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// CIDR de la red.
    pub const fn cidr(&self) -> CidrBlock {
        self.cidr
    }

    /// Subredes declaradas.
    pub fn subnets(&self) -> &[Subnet] {
        &self.subnets
    }

    /// Rutas declaradas.
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Reglas de firewall declaradas.
    pub fn firewall_rules(&self) -> &[FirewallRule] {
        &self.firewall_rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cidr_requires_prefix() {
        assert_eq!(
            CidrBlock::new("10.0.0.0"),
            Err(NetworkingDecisionError::InvalidCidr("10.0.0.0"))
        );
    }

    #[test]
    fn private_subnet_stays_private_without_internet_route() {
        let mut network = VirtualNetwork::new("internal", "10.0.0.0/16").unwrap();
        network
            .add_subnet(
                "app-a",
                "10.0.10.0/24",
                "zone-a",
                NetworkRole::PrivateWorkload,
            )
            .unwrap();

        assert_eq!(
            network.exposure_for_subnet("app-a"),
            Some(Exposure::PrivateOnly)
        );
    }

    #[test]
    fn firewall_requires_visible_purpose() {
        assert_eq!(
            FirewallRule::allow(
                "api",
                TrafficEndpoint::Subnet("web-a"),
                TrafficEndpoint::Subnet("app-a"),
                NetworkProtocol::Tcp,
                443,
                "",
            ),
            Err(NetworkingDecisionError::MissingPurpose)
        );
    }
}
