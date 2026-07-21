import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = join(root, "course.manifest.json");
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const summaryPath = join(root, "docs", "SUMMARY.md");
const summary = readFileSync(summaryPath, "utf8");

const requiredChapterFields = [
  "document",
  "exercises",
  "costs",
  "module",
  "example",
  "tests",
  "diagram",
];

const missing = [];
const summaryLinks = [...summary.matchAll(/\[[^\]]+\]\((\.\/[^)]+)\)/g)].map(
  (match) => normalize(join("docs", match[1])),
);

for (const chapter of manifest.chapters) {
  for (const field of requiredChapterFields) {
    const value = chapter[field];

    if (!value) {
      missing.push(`${chapter.number}. ${chapter.title}: falta campo ${field}`);
      continue;
    }

    if (!existsSync(join(root, value))) {
      missing.push(
        `${chapter.number}. ${chapter.title}: no existe ${field} (${value})`,
      );
    }
  }
}

for (const link of summaryLinks) {
  if (!existsSync(join(root, link))) {
    missing.push(`SUMMARY.md: no existe enlace ${link}`);
  }
}

if (missing.length > 0) {
  console.error("Rutas faltantes en navegación del curso:");
  for (const item of missing) {
    console.error(`- ${item}`);
  }
  process.exit(1);
}

console.log(
  `Rutas verificadas: ${manifest.chapters.length} capítulos y ${summaryLinks.length} enlaces de SUMMARY.md.`,
);
