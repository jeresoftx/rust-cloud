//! Fundamentos ejecutables del curso `rust-cloud`.
//!
//! Este crate empieza pequeño por diseño: el repositorio primero establece la
//! gobernanza, el plan y el contrato educativo. Los capítulos posteriores
//! agregarán módulos por concepto de cloud conforme a RFC-0001 §14.

pub mod compute;
pub mod iam;
pub mod managed_services;
pub mod networking;
pub mod serverless;
pub mod service_models;
pub mod storage;

/// Nombre público del curso dentro de Jeresoft Academy.
pub const COURSE_NAME: &str = "Cloud";

/// Identificador estable del repositorio/curso.
pub const COURSE_SLUG: &str = "rust-cloud";

/// Estado editorial de un capítulo del curso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChapterStatus {
    /// El capítulo existe en el plan, pero aún no tiene implementación.
    Planned,
    /// El capítulo tiene una primera especificación editorial.
    Draft,
    /// El capítulo ya expone un modelo Rust mínimo verificable.
    Implemented,
}

/// Describe un capítulo planeado del curso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chapter {
    /// Número humano del capítulo.
    pub number: u8,
    /// Título editorial del capítulo.
    pub title: &'static str,
    /// Estado actual del capítulo.
    pub status: ChapterStatus,
}

/// Devuelve el mapa inicial de capítulos decidido por RFC-0001 §10.
///
/// # Examples
///
/// ```
/// let chapters = rust_cloud::planned_chapters();
/// assert_eq!(chapters.len(), 10);
/// assert_eq!(chapters[0].title, "Modelos de servicio");
/// ```
pub fn planned_chapters() -> &'static [Chapter] {
    &PLANNED_CHAPTERS
}

const PLANNED_CHAPTERS: [Chapter; 10] = [
    Chapter {
        number: 1,
        title: "Modelos de servicio",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 2,
        title: "Cómputo",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 3,
        title: "Almacenamiento",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 4,
        title: "Redes y VPC",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 5,
        title: "Identidad y accesos",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 6,
        title: "Servicios manejados",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 7,
        title: "Serverless",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 8,
        title: "Costos y FinOps",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 9,
        title: "AWS en la práctica",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 10,
        title: "GCP en la práctica",
        status: ChapterStatus::Planned,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_course_identity() {
        assert_eq!(COURSE_NAME, "Cloud");
        assert_eq!(COURSE_SLUG, "rust-cloud");
    }

    #[test]
    fn exposes_ten_planned_chapters() {
        let chapters = planned_chapters();

        assert_eq!(chapters.len(), 10);
        assert_eq!(chapters[7].title, "Costos y FinOps");
        assert!(
            chapters
                .iter()
                .skip(7)
                .all(|chapter| chapter.status == ChapterStatus::Planned)
        );
        assert_eq!(chapters[0].status, ChapterStatus::Implemented);
        assert_eq!(chapters[1].status, ChapterStatus::Implemented);
        assert_eq!(chapters[2].status, ChapterStatus::Implemented);
        assert_eq!(chapters[3].status, ChapterStatus::Implemented);
        assert_eq!(chapters[4].status, ChapterStatus::Implemented);
        assert_eq!(chapters[5].status, ChapterStatus::Implemented);
        assert_eq!(chapters[6].status, ChapterStatus::Implemented);
    }
}
