# Higiene de ramas y PRs autónomos

- **Curso:** rust-cloud
- **Estado:** implemented
- **Issue:** #112
- **Milestone:** 14. Paquete de revisión humana

Esta guía protege el flujo de revisión diferida cuando la IA trabaja por
bloques autónomos. No publica el curso, no aprueba capítulos y no cambia
estados editoriales a `reviewed` ni `published`.

El objetivo es que cada cambio conserve una historia fácil de revisar:

```text
1 issue -> 1 rama -> 1 commit principal -> 1 PR -> squash merge
```

## Regla base

Cada PR autónomo debe resolver un solo issue. El issue debe estar asignado a
`jeresoftx`, tener milestone y labels coherentes. El PR debe conservar la misma
asignación, milestone y labels, además de cerrar el issue con `Closes #N`.

## Coautoría para Pair Extraordinaire

Cuando un bloque real requiera acreditar la coautoría a una identidad concreta,
esa identidad no debe quedar como autora principal. GitHub puede reconocer la
coautoría cuando el trailer `Co-authored-by` utiliza un correo asociado a la
cuenta; la elegibilidad final la determina GitHub.

El commit principal y el squash merge deben conservar el trailer de la cuenta
destinataria:

| Cuenta destinataria | Trailer válido |
| --- | --- |
| `jeresoftx` | `Co-authored-by: Joel Alvarez Mexia <139817810+jeresoftx@users.noreply.github.com>` |
| `joelalvarezduenas` | `Co-authored-by: Joel Alvarez D. <124008575+joelalvarezduenas@users.noreply.github.com>` |

El correo válido de una cuenta no sustituye al de la otra: se elige según la
identidad que deba recibir la atribución. Antes de fusionar, confirmar que el
autor principal corresponde a una identidad distinta de la destinataria.

Esta regla no justifica PRs vacíos, commits artificiales ni cambios sin valor.
Los achievements son secundarios; la trazabilidad, la calidad del curso y la
revisión humana siguen siendo la prioridad.

### Flujo revisable validado

Cuando la contribución de una segunda identidad requiera revisión humana, se
usa este flujo completo:

1. La autora principal crea una rama desde `main` actualizado y registra el
   cambio real con el trailer `Co-authored-by` de la identidad colaboradora.
2. El PR se asigna a `jeresoftx`, conserva el milestone y los labels del issue,
   y solicita revisión de la identidad coautora.
3. La identidad coautora revisa el cambio en GitHub. No se fusiona con una
   solicitud pendiente ni con una aprobación ausente.
4. Después de la aprobación humana y de las compuertas en verde, el squash
   merge conserva tanto `Closes #N` como el trailer `Co-authored-by`.

El [PR #136](https://github.com/jeresoftx/rust-cloud/pull/136) valida este
flujo: fue asignado a `jeresoftx`, revisado y aprobado por
`joelalvarezduenas`, y fusionado solo después de esa aprobación. Este patrón
documenta colaboración real; no autoriza crear trabajo artificial para obtener
achievements.

## Antes de crear el PR

El agente debe verificar que la rama nace de `main` actualizado y que solo
contiene el commit del issue actual.

Comprobaciones obligatorias:

- `git status --short --branch` debe mostrar una rama limpia.
- `git rev-list --count origin/main..HEAD` debe devolver `1`.
- El mensaje del commit debe incluir `Closes #N` con el issue correcto.
- `git diff --check` debe pasar antes de abrir el PR.
- Las compuertas aplicables del repositorio deben pasar localmente.

Si el conteo de commits es mayor que `1`, el PR no debe crearse todavía. La
rama debe corregirse antes, normalmente rebasándola sobre `origin/main` o
recreándola desde `main` sin arrastrar commits de otro issue.

## Antes de fusionar el PR

Antes del squash merge, el agente debe confirmar:

- el PR está abierto y en estado mergeable;
- el PR tiene exactamente un commit visible;
- el PR está asignado a `jeresoftx`;
- el PR tiene el milestone del issue;
- el PR tiene labels coherentes;
- el check remoto aplicable está en verde;
- el cuerpo del PR declara que se fusiona en modo de revisión diferida;
- el squash merge preserva `Closes #N` y el `Co-authored-by` correcto para el
  objetivo de coautoría declarado.

Un PR con commits de issues anteriores no debe fusionarse. Primero se corrige
la rama y se vuelve a verificar la metadata.

## PRs duplicados o vacíos

Si por error se abre un PR duplicado, vacío o con historia incorrecta, el agente
debe registrarlo de forma transparente en la bitácora de revisión diferida. No
se debe ocultar el evento ni presentarlo como avance de contenido.

Reglas para manejar duplicados:

- si el PR todavía está abierto y no aporta cambios, se cierra sin fusionar;
- si GitHub ya lo fusionó como PR vacío durante una corrección de rama, se
  documenta como higiene de proceso;
- no se deben fusionar PRs vacíos deliberadamente para aumentar conteos de
  achievements;
- no se debe reescribir historial solo para corregir un achievement;
- el siguiente PR debe salir desde `main` actualizado y volver a pasar las
  comprobaciones de un solo commit.

## Después del merge

Después de fusionar, el agente debe sincronizar `main`, comprobar que el issue
quedó cerrado y continuar con el siguiente issue del bloque solo si el árbol
local está limpio.

Si GitHub no cierra el issue automáticamente, se debe cerrar manualmente y
registrar la causa. Para evitarlo, el PR debe usar siempre `Closes #N`; puede
incluir texto en español, pero la palabra clave de cierre debe estar en inglés.
