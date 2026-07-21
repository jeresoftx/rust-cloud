import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..");
const manifest = JSON.parse(
  readFileSync(join(root, "course.manifest.json"), "utf8"),
);
const failures = [];

function fail(message) {
  failures.push(message);
}

function hasHeading(markdown, patterns) {
  return patterns.some((pattern) => pattern.test(markdown));
}

function hasText(markdown, value) {
  return markdown.includes(value);
}

const requiredHeadingGroups = [
  {
    name: "Concepto",
    patterns: [/^## Concepto$/m],
  },
  {
    name: "Problema",
    patterns: [/^## Problema$/m],
  },
  {
    name: "Alternativas",
    patterns: [/^## Alternativas(?: consideradas)?$/m],
  },
  {
    name: "Justificación",
    patterns: [/^## Justificación$/m],
  },
  {
    name: "Invariantes",
    patterns: [/^## Invariantes del capítulo$/m],
  },
  {
    name: "Modelo Rust",
    patterns: [
      /^## Modelo Rust mínimo$/m,
      /^## Requisitos funcionales del modelo Rust$/m,
    ],
  },
  {
    name: "Lectura del módulo Rust",
    patterns: [/^## Cómo leer el módulo Rust$/m, /^## Lectura del modelo$/m],
  },
  {
    name: "Diagrama",
    patterns: [/^## Diagrama$/m],
  },
  {
    name: "Ejemplo ejecutable",
    patterns: [/^## Ejemplo ejecutable$/m],
  },
  {
    name: "Práctica o ejercicios",
    patterns: [/^## Práctica sugerida$/m, /^## Ejercicios y costos$/m],
  },
  {
    name: "Cierre editorial",
    patterns: [/^## Estado editorial$/m, /^## Nota editorial$/m],
  },
];

for (const chapter of manifest.chapters) {
  const markdown = readFileSync(join(root, chapter.document), "utf8");
  const label = `${chapter.number}. ${chapter.title}`;

  if (!hasText(markdown, "- **Curso:** rust-cloud")) {
    fail(`${label}: falta metadato de curso`);
  }

  if (!hasText(markdown, "- **Estado:** implemented")) {
    fail(`${label}: falta estado implemented`);
  }

  for (const [field, title] of [
    ["module", "Módulo Rust"],
    ["diagram", "Diagrama"],
    ["example", "Ejemplo"],
    ["exercises", "Ejercicios"],
    ["costs", "Costos"],
  ]) {
    if (!hasText(markdown, `- **${title}:** \`${chapter[field]}\``)) {
      fail(`${label}: falta referencia ${title} a ${chapter[field]}`);
    }
  }

  for (const group of requiredHeadingGroups) {
    if (!hasHeading(markdown, group.patterns)) {
      fail(`${label}: falta sección ${group.name}`);
    }
  }

  if (!hasText(markdown, "reviewed") || !hasText(markdown, "published")) {
    fail(`${label}: el cierre editorial debe mencionar reviewed y published`);
  }
}

if (failures.length > 0) {
  console.error("Anatomía editorial incompleta:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `Anatomía editorial verificada: ${manifest.chapters.length} capítulos listos para revisión humana.`,
);
