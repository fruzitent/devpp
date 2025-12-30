pub mod instr;

use std::fmt::Display;
use std::fmt::Formatter;

use crate::instr::Instr;
use crate::instr::directive::Directive;

#[derive(Debug, Default)]
pub struct Containerfile(Vec<Instr>);

impl Containerfile {
    pub fn append(&mut self, other: &mut Vec<Instr>) {
        self.0.append(other);
    }

    pub fn push(&mut self, value: Instr) {
        self.0.push(value);
    }
}

impl Display for Containerfile {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        assert!(!self.0.is_empty());
        let mut escape = None;
        for instr in &self.0 {
            if let Instr::Directive(directive) = instr
                && let Directive::Escape(c) = directive
            {
                escape = Some(*c);
            }
            instr.display(escape).fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containerfile() {
        let mut cf = Containerfile::default();
        cf.push(Instr::Directive(Directive::Escape('`')));
        cf.push(Instr::Arg(vec![(String::from("foo"), Some(String::from("test`123")))]));
        assert_eq!(format!("{cf}"), String::from("# escape=`\nARG foo=\"test``123\"\n"))
    }
}
