// Este quiz testa:
// - Generics
// - Traits
//
// Uma escola de magia imaginária tem um novo sistema de geração de boletins
// escrito em Rust! Atualmente, o sistema só oferece suporte à criação de
// boletins em que a nota do aluno é representada numericamente (ex.: 1.0 -> 5.5).
// Porém, a escola também emite notas alfabéticas (A+ -> F-) e precisa conseguir
// imprimir os dois tipos de boletim!
//
// Faça as alterações necessárias na struct `ReportCard` e no bloco impl
// para dar suporte a boletins alfabéticos além dos numéricos.

// TODO: Ajuste a struct conforme descrito acima.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Ajuste o bloco impl conforme descrito acima.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
