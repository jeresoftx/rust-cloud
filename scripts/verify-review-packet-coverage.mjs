import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..");
const manifest = JSON.parse(
  readFileSync(join(root, "course.manifest.json"), "utf8"),
);
const packetPath = join(root, "docs", "paquete-revision-humana.md");
const packet = readFileSync(packetPath, "utf8");
const failures = [];

function fail(message) {
  failures.push(message);
}

const requiredGlobalSections = [
  "# Paquete de revisión humana",
  "## Frontera de decisión",
  "## Compuertas antes de revisar",
  "## Preguntas por capítulo",
  "## Capítulos",
  "## Resultado de la revisión",
];

for (const section of requiredGlobalSections) {
  if (!packet.includes(section)) {
    fail(`falta sección global: ${section}`);
  }
}

for (const command of [
  "node scripts/verify-course-links.mjs",
  "node scripts/verify-manifest-consistency.mjs",
  "node scripts/verify-cargo-examples.mjs",
  "node scripts/verify-chapter-anatomy.mjs",
  "node scripts/verify-review-packet-coverage.mjs",
  "node scripts/verify-deferred-review-log.mjs",
  "node scripts/verify-editorial-status.mjs",
  "node scripts/verify-gate-sync.mjs",
  "cargo fmt --check",
  "cargo clippy --all-targets --all-features -- -D warnings",
  "cargo test --all-targets",
  "cargo test --doc",
  "cargo bench --all-targets",
  "git diff --check",
]) {
  if (!packet.includes(command)) {
    fail(`falta comando de compuerta: ${command}`);
  }
}

for (const chapter of manifest.chapters) {
  const prefix = String(chapter.number).padStart(2, "0");
  const heading = `### ${prefix}. ${chapter.title}`;
  const headingIndex = packet.indexOf(heading);

  if (headingIndex === -1) {
    fail(`${chapter.title}: falta encabezado ${heading}`);
    continue;
  }

  const nextHeadingIndex = packet.indexOf("\n### ", headingIndex + heading.length);
  const chapterSection = packet.slice(
    headingIndex,
    nextHeadingIndex === -1 ? packet.length : nextHeadingIndex,
  );

  for (const [field, label] of [
    ["document", "Documento"],
    ["exercises", "Ejercicios"],
    ["costs", "Costos"],
    ["module", "Módulo Rust"],
    ["example", "Ejemplo"],
    ["tests", "Tests"],
    ["diagram", "Diagrama"],
  ]) {
    const expectedLine = `- ${label}: \`${chapter[field]}\``;

    if (!chapterSection.includes(expectedLine)) {
      fail(`${chapter.title}: falta ${expectedLine}`);
    }
  }

  if (!chapterSection.includes("- Revisión humana:")) {
    fail(`${chapter.title}: falta criterio de revisión humana`);
  }
}

if (!packet.includes("`reviewed`") || !packet.includes("`published`")) {
  fail("el paquete debe conservar explícita la frontera reviewed/published");
}

if (failures.length > 0) {
  console.error("Paquete de revisión humana incompleto:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `Paquete de revisión humana verificado: ${manifest.chapters.length} capítulos cubiertos.`,
);
