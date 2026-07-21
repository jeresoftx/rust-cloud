# Ejercicios: redes y VPC

- **Curso:** rust-cloud
- **Capítulo:** 04. Redes y VPC
- **Estado:** implemented
- **Issue:** #16

Estos ejercicios practican redes cloud como frontera, rutas, reglas y
exposición. La meta no es memorizar nombres de servicios, sino declarar qué
puede hablar con qué, por qué camino y con qué intención.

## Ejercicio 1: separar subred pública y privada `[Nivel 1]`

Construye una `VirtualNetwork` con:

- una subred pública `edge-a`;
- una subred privada `app-a`;
- una ruta `0.0.0.0/0` hacia `RouteTarget::InternetGateway`.

Usa `exposure_for_subnet` para comprobar que solo `edge-a` queda públicamente
ruteable.

**Entrada/Salida esperada:** `edge-a` debe ser `PubliclyRoutable` y `app-a`
debe ser `PrivateOnly`.

<details>
<summary>Pista</summary>
La ruta pública no vuelve pública a cualquier subred. En este modelo, la
exposición pública requiere rol `PublicEdge` y ruta pública explícita.
</details>

Solución compilable: `examples/soluciones/networking_nivel_1.rs`.

## Ejercicio 2: detectar decisiones riesgosas `[Nivel 2]`

Modela dos errores:

- declarar dos subredes con el mismo rango CIDR;
- permitir SSH público desde Internet hacia una subred.

**Entrada/Salida esperada:** el primer caso debe producir
`NetworkingDecisionError::OverlappingSubnet`; el segundo,
`NetworkingDecisionError::PublicSshExposure`.

<details>
<summary>Pista</summary>
El modelo no pretende ser un firewall real. Es una herramienta pedagógica para
hacer visibles errores de criterio antes de tocar infraestructura.
</details>

Solución compilable: `examples/soluciones/networking_nivel_2.rs`.

## Ejercicio 3: diseñar una red mínima para la academia `[Nivel 3]`

Diseña una red con:

- borde público `edge-a`;
- aplicación privada `app-a`;
- datos aislados `data-a`;
- ruta pública controlada;
- ruta local interna;
- regla HTTPS de `edge-a` hacia `app-a`;
- regla de base de datos de `app-a` hacia `data-a`.

Comprueba cuántas subredes, rutas y reglas quedaron declaradas, y valida que la
subred de datos siga privada.

**Entrada/Salida esperada:** tres subredes, dos rutas, dos reglas y `data-a`
como `PrivateOnly`.

<details>
<summary>Pista</summary>
No abras una regla desde Internet hacia datos. El borde público debe ser la
frontera de entrada; la aplicación y los datos quedan internos.
</details>

Solución compilable: `examples/soluciones/networking_nivel_3.rs`.

## Ejercicio 4: razonar conectividad privada y costos `[Nivel 4]`

Escribe una propuesta breve para conectar:

- aplicación web;
- API privada;
- base de datos administrada;
- almacenamiento de objetos;
- observabilidad;
- acceso administrativo.

Para cada flujo, declara origen, destino, ruta esperada, regla necesaria,
exposición pública o privada, observabilidad mínima y costo operativo.

**Entrada/Salida esperada:** una tabla con siete columnas: flujo, origen,
destino, ruta, regla, exposición y costo/riesgo.

<details>
<summary>Pista</summary>
La respuesta correcta no es "todo privado" ni "todo público". El criterio está
en justificar qué debe exponerse, qué debe quedar interno y qué costo aparece
por cada gateway, balanceador, salto entre zonas o enlace privado.
</details>

Discusión sugerida:

- El ingreso público suele concentrarse en un borde explícito: balanceador,
  gateway o CDN.
- Las APIs privadas reducen exposición, pero agregan rutas, DNS, permisos y
  observabilidad.
- Una base de datos administrada no debe recibir tráfico directo desde
  Internet como diseño base.
- La salida a almacenamiento de objetos puede requerir NAT, endpoint privado o
  reglas específicas según proveedor.
- El acceso administrativo debe preferir caminos auditables y temporales, no
  SSH público abierto.
