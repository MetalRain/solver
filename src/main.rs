/*
SAT instance is built from N clauses

Clauses can either have AND or OR operator
and N literals.

Literal is either positive or negative and has name
*/
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
struct Literal {
    negated: bool,
    name: String
}

impl Literal {
    fn same_name_as(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn inverse_of(&self, other: &Self) -> bool {
        self.same_name_as(other) && self.negated != other.negated
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.negated == other.negated
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.name.cmp(&other.name);
        if ord == Ordering::Equal {
            return self.negated.cmp(&other.negated);
        }
        return ord;
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



#[derive(Debug, Clone)]
enum Operator {
    OR,
    AND
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Operator::OR, &Operator::OR) => true,
            (&Operator::AND, &Operator::AND) => true,
            _ => false
        }
    }
}


#[derive(Debug, Clone)]
struct Clause {
    operator: Operator,
    literals: Vec<Literal>
}


impl Clause {
    fn satisfied_by(self: Self, state: &InstanceState) -> bool {
        // Collect states for this clause
        let clause_literal_states: Vec<Option<bool>> =
            self.literals.into_iter().map(|clause_literal| {
                let state: Option<LiteralState> = state.states.clone()
                    .into_iter()
                    .find(|state| {
                        match state {
                            LiteralState {
                                literal: state_literal,
                                value: _
                            } => clause_literal.same_name_as(state_literal)
                        }
                    });
                match state {
                    Some(LiteralState {
                        literal: _,
                        value
                    }) => {
                        match (value, clause_literal.negated) {
                            (Some(state_bool), true) => Some(!state_bool),
                            (Some(state_bool), false) => Some(state_bool),
                            (None, _) => None
                        }
                    },
                    _ => None
                }
            }).collect();

        // State has all required literals
        let needed_literals_set = clause_literal_states.clone()
            .into_iter()
            .all(|v| { 
                match v {
                    Some(_) => true,
                    _ => false
                }
            });
        
        if !needed_literals_set {
            return false
        }

        match self.operator {
            Operator::OR => {
                clause_literal_states
                    .into_iter()
                    .any(|v| {
                        match v {
                            Some(true) => true,
                            _ => false
                        }
                    })
            },
            Operator::AND => {
                clause_literal_states
                    .into_iter()
                    .all(|v| {
                        match v {
                            Some(true) => true,
                            _ => false
                        }
                    })
            }
        }
    }
}


#[derive(Debug, Clone)]
struct SatInstance {
    clauses: Vec<Clause>
}

impl SatInstance {
    fn inspect(self: Self) -> Vec<Literal> {
        let mut literals = self.clauses
            .into_iter()
            .flat_map(|c| c.literals)
            .collect::<Vec<Literal>>();
        literals.sort();
        literals.dedup_by(|a, b| a.inverse_of(b));
        return literals
    }

    fn satisfied_by(self: Self, state: &InstanceState) -> bool {
        self.clauses.into_iter().all(|c| c.satisfied_by(&state))
    }
}


#[derive(Debug, Clone)]
struct LiteralState {
    literal: Literal,
    value: Option<bool>
}

impl PartialEq for LiteralState {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal
            && self.value == other.value
    }
}

#[derive(Debug, Clone)]
struct InstanceState {
    states: Vec<LiteralState>
}


fn main() {
    // (a or b) and (c or (not b)) -> true
    // solution a = true, b = true/false, c = true
    let instance = SatInstance {
        clauses: vec![
            Clause {
                operator: Operator::OR,
                literals: vec![
                    Literal {
                        name: String::from("a"),
                        negated: false
                    },
                    Literal {
                        name: String::from("b"),
                        negated: false
                    }
                ]
            },
            Clause {
                operator: Operator::AND,
                literals: vec![
                    Literal {
                        name: String::from("c"),
                        negated: false
                    },
                    Literal {
                        name: String::from("b"),
                        negated: true
                    }
                ]
            }
        ]
    };

    //println!("{:#?}", instance);

    let literals = instance.clone().inspect();

    //println!("{:#?}", literals);

    let state = InstanceState {
        states: vec![
            LiteralState {
                literal: literals[0].clone(),
                value: Some(true)
            },
            LiteralState {
                literal: literals[1].clone(),
                value: Some(false)
            },
            LiteralState {
                literal: literals[2].clone(),
                value: Some(true)
            },
        ]
    };

    //println!("{:#?}", state);

    println!("{:#?}", instance.satisfied_by(&state));
}
