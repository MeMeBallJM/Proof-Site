#[derive(Clone, Debug)]
pub enum Statement {
    Axiom(String),
    Not(Box<Statement>),
    And(Box<(Statement, Statement)>),
    Or(Box<(Statement, Statement)>),
}

pub type LogicalRule = Box<dyn Fn(Vec<Statement>) -> Result<Statement, EntryError>>;

pub struct Entry {
    assumptions: Vec<Statement>,
    logical_rules: Vec<LogicalRule>,
    construction: Vec<(Vec<usize>, usize)>,
}

enum EntryError {
    IndexNonexistentStatement(usize),
    IndexNonexistentRule(usize),
}

use EntryError::*;

impl Entry {
    fn build_proof(&self) -> Result<Option<Statement>, EntryError> {
        let mut ledger = self.assumptions.clone();

        for (premises_indices, logical_rule_index) in &self.construction {
            let logical_rule = self
                .logical_rules
                .get(*logical_rule_index)
                .ok_or(IndexNonexistentRule(*logical_rule_index))?;

            let mut premises = Vec::with_capacity(premises_indices.len());
            for &i in premises_indices {
                premises.push(ledger.get(i).ok_or(IndexNonexistentStatement(i))?.clone());
            }

            let new_statement = logical_rule(premises)?;

            ledger.push(new_statement);
        }

        Ok(ledger.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
