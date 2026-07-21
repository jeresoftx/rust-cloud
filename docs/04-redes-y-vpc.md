# Redes y VPC

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 04. Redes y VPC
- **Issues:** #14, #15
- **Módulo Rust:** `src/networking.rs`
- **Diagrama:** `diagrams/04-redes-y-vpc.mmd`
- **Ejemplo:** `examples/networking.rs`

## Concepto

Una red cloud no es solo una colección de subredes, direcciones IP y pantallas
de configuración. Es el contrato que define qué cargas de trabajo existen
dentro de un límite, cómo se encuentran, qué caminos puede tomar el tráfico y
qué exposición queda permitida.

En este curso, una VPC se estudia como una frontera de alcance y confianza:
agrupa recursos, organiza subredes, rutas, reglas de tráfico, puntos de entrada,
conectividad privada y resolución de nombres. El objetivo no es memorizar la
consola de un proveedor, sino razonar sobre alcance, aislamiento,
disponibilidad y operación.

## Imagen mental

Piensa en un edificio técnico.

- La **VPC** es el perímetro del edificio: no garantiza seguridad por sí sola,
  pero define dónde empieza y termina el espacio controlado.
- Las **subredes** son pisos o áreas: recepción, oficinas internas, cuarto de
  servidores y zonas operativas.
- Las **rutas** son pasillos y salidas: hacen posible moverse de un lugar a
  otro, pero no dicen quién está autorizado.
- Las **reglas de tráfico** son puertas con criterio: origen, destino,
  protocolo, puerto y propósito.
- La **exposición pública** es una puerta hacia la calle. Debe existir por una
  razón clara, no por accidente.

La metáfora sirve para separar tres preguntas que suelen mezclarse:
direccionamiento, camino posible y autorización. Tener una dirección no implica
tener permiso; tener una ruta no implica que cualquier tráfico deba pasar.

## Problema

La enseñanza de redes cloud suele caer en dos extremos:

- Convertirse en una lista de servicios de proveedor sin explicar el modelo
  mental.
- Repetir redes tradicionales sin mostrar qué cambia cuando la infraestructura
  se vuelve programable.

Ambos caminos ocultan las preguntas de ingeniería que importan: ¿qué puede
hablar con qué?, ¿por dónde entra y sale el tráfico?, ¿qué rutas existen?, ¿qué
reglas autorizan el flujo?, ¿qué nombres resuelven los destinos?, ¿qué pasa si
una zona falla? y ¿qué costo operativo introduce cada nivel de aislamiento?

## Alternativas consideradas

1. **Empezar desde una consola de proveedor.** Es práctico, pero ata el
   aprendizaje a nombres comerciales y deja débil el criterio transferible.
2. **Empezar desde redes clásicas.** Refuerza fundamentos, pero puede ignorar
   decisiones propias de cloud como VPC, subredes privadas, gateways, reglas
   administradas y conectividad entre cuentas o proyectos.
3. **Empezar desde alcanzabilidad y límites de confianza.** Permite explicar
   productos concretos más adelante sin perder la idea rectora.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se define el criterio:
límites, rutas, reglas, exposición y operación. Después se podrán mapear esos
conceptos a AWS, GCP u otro proveedor sin que el curso dependa de una interfaz
específica.

La decisión sigue RFC-0001 §2: concepto antes de implementación, problema antes
de herramienta, alternativas antes de elección. También conserva RFC-0001 §10:
el currículum debe formar criterio de ingeniería, no solo memoria operativa.

## Invariantes del capítulo

- Toda carga de trabajo vive dentro de una frontera de red explícita o implícita.
- El direccionamiento define identidad y alcanzabilidad, pero no debe tratarse
  como autorización suficiente.
- Una subred expresa ubicación, alcance de rutas y dominio de fallo; no es una
  política de seguridad por sí sola.
- Las rutas determinan caminos posibles; las reglas de tráfico determinan qué
  comunicación se permite.
- Todo ingreso público debe ser explícito, justificable y observable.
- La conectividad privada reduce exposición pública, pero agrega decisiones de
  ruteo, DNS, operación y costo.
- El balanceo de carga cambia el modelo de entrada, salud y tolerancia a fallos.
- La resolución de nombres forma parte de la conectividad; no es un detalle
  decorativo.
- Los límites, precios y nombres de proveedor son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/networking.rs` y modela, sin dependencias
externas:

- redes virtuales con nombre y rango de direcciones educativo;
- subredes con rol, zona lógica y relación con la red virtual;
- rutas con destino, siguiente salto y propósito;
- reglas de tráfico con origen, destino, protocolo y puerto;
- exposición pública o privada como decisión explícita;
- validaciones legibles cuando falte nombre, destino, zona o intención.

El módulo no intenta simular paquetes reales ni reemplazar herramientas de red.
Su función es pedagógica: hacer ejecutables los conceptos del capítulo y
permitir pruebas unitarias claras.

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| VPC | Frontera de alcance y confianza | Autorización fina | Creer que todo dentro de la VPC es seguro |
| Subred | Ubicación lógica, rol y dominio de fallo | Permiso de tráfico | Tratar `public` o `private` como etiqueta decorativa |
| Ruta | Camino posible hacia un destino | Quién puede usarlo | Abrir salida o ingreso sin propósito |
| Firewall | Tráfico permitido | Topología completa | Autorizar reglas amplias por comodidad |
| Gateway | Punto de entrada o salida | Diseño de confianza | Convertirlo en acceso público indiscriminado |
| Conectividad privada | Camino sin Internet público | Simplicidad operativa | Olvidar DNS, rutas, costos y observabilidad |

## Cómo leer el módulo Rust

El módulo `networking` empieza declarando una red virtual:

```rust
use rust_cloud::networking::VirtualNetwork;

let mut network = VirtualNetwork::new("academy-prod", "10.20.0.0/16").unwrap();
```

Después se agregan subredes con rol y zona lógica:

```rust
use rust_cloud::networking::NetworkRole;

# let mut network = rust_cloud::networking::VirtualNetwork::new(
#     "academy-prod",
#     "10.20.0.0/16",
# ).unwrap();
network
    .add_subnet("edge-a", "10.20.1.0/24", "zone-a", NetworkRole::PublicEdge)
    .unwrap();
network
    .add_subnet(
        "app-a",
        "10.20.10.0/24",
        "zone-a",
        NetworkRole::PrivateWorkload,
    )
    .unwrap();
```

Una ruta pública se declara con destino, siguiente salto y propósito:

```rust
use rust_cloud::networking::{Exposure, Route, RouteTarget};

# let mut network = rust_cloud::networking::VirtualNetwork::new(
#     "academy-prod",
#     "10.20.0.0/16",
# ).unwrap();
# network
#     .add_subnet("edge-a", "10.20.1.0/24", "zone-a", rust_cloud::networking::NetworkRole::PublicEdge)
#     .unwrap();
network
    .add_route(
        Route::new(
            "entrada-controlada",
            "0.0.0.0/0",
            RouteTarget::InternetGateway,
            "el balanceador público recibe tráfico HTTPS",
        )
        .unwrap(),
    )
    .unwrap();

assert_eq!(
    network.exposure_for_subnet("edge-a"),
    Some(Exposure::PubliclyRoutable),
);
```

La regla de firewall hace visible la intención. El capítulo rechaza SSH público
desde Internet como diseño base:

```rust
use rust_cloud::networking::{
    FirewallRule, NetworkProtocol, NetworkingDecisionError, TrafficEndpoint,
};

let public_ssh = FirewallRule::allow(
    "ssh-publico",
    TrafficEndpoint::Internet,
    TrafficEndpoint::Subnet("app-a"),
    NetworkProtocol::Tcp,
    22,
    "atajo operativo",
);

assert_eq!(
    public_ssh,
    Err(NetworkingDecisionError::PublicSshExposure(
        "no publiques SSH a Internet como diseño base",
    )),
);
```

El objetivo del modelo no es decidir por proveedor. El objetivo es obligar a
nombrar la frontera, el camino y la intención antes de hablar de AWS, GCP o
cualquier consola.

## Diagrama

El diagrama del capítulo vive en `diagrams/04-redes-y-vpc.mmd`. Resume la
lectura principal:

```text
VPC -> subredes -> rutas posibles -> reglas permitidas -> exposición efectiva
```

## Ejemplo ejecutable

El ejemplo `examples/networking.rs` construye una VPC educativa para una
academia: borde público, aplicación privada, datos aislados, ruta pública
explícita y regla HTTPS interna.

```bash
cargo run --example networking
```

El ejemplo no abre conexiones ni contacta proveedores. Su intención es mostrar
qué decisiones deben quedar visibles antes de crear infraestructura real.

## Práctica sugerida

Antes de dibujar una red cloud, escribe:

1. Frontera: qué vive dentro de la VPC y qué queda fuera.
2. Subredes: rol, zona lógica y dominio de fallo esperado.
3. Ingreso público: qué entra desde Internet y por qué.
4. Salida privada: qué cargas necesitan salir sin aceptar ingreso directo.
5. Reglas: origen, destino, protocolo, puerto y propósito.
6. DNS y nombres: cómo se encontrarán los servicios.
7. Observabilidad: dónde se registran rutas, rechazos y cambios.
8. Costo: gateways, balanceadores, tráfico entre zonas y conectividad privada.

Si una regla no puede explicar su propósito en una frase, todavía no debería
estar en el diseño.

## Decisiones registradas

- El rango de direcciones se modela como `CidrBlock`, un tipo educativo de
  CIDR IPv4 validado sin dependencias externas.
- Las rutas declaran destino, siguiente salto y propósito.
- Las reglas de firewall declaran origen, destino, protocolo, puerto y
  propósito humano.
- SSH público desde Internet se rechaza como diseño base del capítulo.

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
