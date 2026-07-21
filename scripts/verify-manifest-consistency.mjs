import { readFileSync } from "node:fs";
import { basename, join } from "node:path";

const root = join(import.meta.dirname, "..");
const manifest = JSON.parse(
  readFileSync(join(root, "course.manifest.json"), "utf8"),
);
const failures = [];

function fail(message) {
  failures.push(message);
}

function assertUnique(field) {
  const seen = new Set();

  for (const chapter of manifest.chapters) {
    const value = chapter[field];

    if (seen.has(value)) {
      fail(`chapters: ${field} duplicado ${value}`);
    }

    seen.add(value);
  }
}

if (manifest.course.id !== "rust-cloud") {
  fail(`course.id inesperado: ${manifest.course.id}`);
}

if (manifest.course.language !== "rust") {
  fail(`course.language inesperado: ${manifest.course.language}`);
}

if (manifest.course.area !== "cloud") {
  fail(`course.area inesperado: ${manifest.course.area}`);
}

if (!Array.isArray(manifest.chapters) || manifest.chapters.length !== 10) {
  fail("chapters debe contener exactamente 10 capítulos");
}

for (const field of ["number", "id", "slug", "title", "document"]) {
  assertUnique(field);
}

for (const [index, chapter] of manifest.chapters.entries()) {
  const expectedNumber = index + 1;
  const prefix = String(expectedNumber).padStart(2, "0");
  const expectedStem = `${prefix}-${chapter.slug}`;

  if (chapter.number !== expectedNumber) {
    fail(`${chapter.title}: número esperado ${expectedNumber}`);
  }

  if (!chapter.id || !/^[a-z0-9-]+$/.test(chapter.id)) {
    fail(`${chapter.title}: id inválido ${chapter.id}`);
  }

  if (!chapter.slug || !/^[a-z0-9-]+$/.test(chapter.slug)) {
    fail(`${chapter.title}: slug inválido ${chapter.slug}`);
  }

  const expectedPaths = {
    document: `docs/${expectedStem}.md`,
    exercises: `docs/ejercicios/${expectedStem}.md`,
    costs: `docs/costos/${expectedStem}.md`,
    diagram: `diagrams/${expectedStem}.mmd`,
  };

  for (const [field, expectedPath] of Object.entries(expectedPaths)) {
    if (chapter[field] !== expectedPath) {
      fail(`${chapter.title}: ${field} debe ser ${expectedPath}`);
    }
  }

  if (!chapter.module.startsWith("src/") || !chapter.module.endsWith(".rs")) {
    fail(`${chapter.title}: module debe apuntar a src/*.rs`);
  }

  if (!chapter.example.startsWith("examples/") || !chapter.example.endsWith(".rs")) {
    fail(`${chapter.title}: example debe apuntar a examples/*.rs`);
  }

  if (!chapter.tests.startsWith("tests/") || !chapter.tests.endsWith(".rs")) {
    fail(`${chapter.title}: tests debe apuntar a tests/*.rs`);
  }

  if (!chapter.milestone.startsWith(prefix)) {
    fail(`${chapter.title}: milestone debe iniciar con ${prefix}`);
  }

  if (basename(chapter.document, ".md") !== expectedStem) {
    fail(`${chapter.title}: document no coincide con número y slug`);
  }
}

if (failures.length > 0) {
  console.error("Manifiesto inconsistente:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("Manifiesto verificado: capítulos ordenados, únicos y coherentes.");
