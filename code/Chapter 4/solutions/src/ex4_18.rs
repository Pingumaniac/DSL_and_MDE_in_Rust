// Define terminal and nonterminal symbols using Enums
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    Terminal(String),
    Nonterminal(String),
}

// Define EBNF Operators
#[derive(Debug, Clone)]
enum EBNFOperator {
    Alternative(Vec<Production>),
    Optional(Box<Production>),
    Iteration(Box<Production>),
    Grouping(Box<Production>),
}

// Define a Production as a nonterminal followed by a sequence of either symbols or EBNF operators
#[derive(Debug, Clone)]
struct Production {
    nonterminal: Symbol,
    sequence: Vec<ProductionElement>,
}

#[derive(Debug, Clone)]
enum ProductionElement {
    Symbol(Symbol),
    Operator(EBNFOperator),
}

// Define the Grammar itself
#[derive(Debug, Clone)]
struct Grammar {
    productions: Vec<Production>,
    start_symbol: Symbol,
}

impl Grammar {
    // Function to add a production to the grammar
    pub fn add_production(&mut self, production: Production) {
        self.productions.push(production);
    }
}

fn main() {
    let mut grammar = Grammar {
        productions: vec![],
        start_symbol: Symbol::Nonterminal("S".to_string()),
    };

    let prod1 = Production {
        nonterminal: Symbol::Nonterminal("S".to_string()),
        sequence: vec![
            ProductionElement::Operator(EBNFOperator::Alternative(vec![
                Production {
                    nonterminal: Symbol::Nonterminal("A".to_string()),
                    sequence: vec![ProductionElement::Symbol(Symbol::Terminal("a".to_string()))],
                },
                Production {
                    nonterminal: Symbol::Nonterminal("B".to_string()),
                    sequence: vec![ProductionElement::Symbol(Symbol::Terminal("b".to_string()))],
                },
            ]))
        ],
    };

    grammar.add_production(prod1);

    println!("{:#?}", grammar);
}
