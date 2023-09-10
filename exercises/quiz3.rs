// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

use std::fmt;
use std::fmt::Display;

pub enum Grade<'a> {
    Numeric(f32),
    Alphabetical(&'a str),
}

impl<'a> Display for Grade<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Grade::Numeric(x) => x.to_string(),
            Grade::Alphabetical(s) => s.to_string(),
        };
        write!(f, "{}", str)
    }
}

pub struct ReportCard<'a> {
    pub grade: Grade<'a>,
    pub student_name: String,
    pub student_age: u8,
}

impl<'a> ReportCard<'a> {
    pub fn print(&self) -> String {
        //println!("{}", self.grade);
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_numeric_report_card() {
        println!("num");
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        let _ = report_card.print();
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Grade::Alphabetical("A+"),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
