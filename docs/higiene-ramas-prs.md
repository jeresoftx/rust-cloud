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

El commit principal y el squash merge deben incluir:

```text
Co-authored-by: Joel Alvarez D. <124008575+joelalvarezduenas@users.noreply.github.com>
```

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
- el squash merge preserva `Closes #N` y el `Co-authored-by`.

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
- el siguiente PR debe salir desde `main` actualizado y volver a pasar las
  comprobaciones de un solo commit.

## Después del merge

Después de fusionar, el agente debe sincronizar `main`, comprobar que el issue
quedó cerrado y continuar con el siguiente issue del bloque solo si el árbol
local está limpio.

Si GitHub no cierra el issue automáticamente, se debe cerrar manualmente y
registrar la causa. Para evitarlo, el PR debe usar siempre `Closes #N`; puede
incluir texto en español, pero la palabra clave de cierre debe estar en inglés.
