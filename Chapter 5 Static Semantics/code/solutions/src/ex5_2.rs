use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Person {
    name: String,
}

#[derive(Debug, Clone)]
struct Relationships {
    parents: Vec<String>,
    children: Vec<String>,
}

struct FamilyTree {
    people: HashMap<String, Person>,
    relationships: HashMap<String, Relationships>,
}

impl FamilyTree {
    fn new() -> Self {
        FamilyTree {
            people: HashMap::new(),
            relationships: HashMap::new(),
        }
    }

    fn add_person(&mut self, name: &str) {
        let person = Person {
            name: name.to_string(),
        };
        self.people.insert(name.to_string(), person);
        self.relationships.insert(name.to_string(), Relationships {
            parents: Vec::new(),
            children: Vec::new(),
        });
    }

    fn add_parent_child_relationship(&mut self, parent: &str, child: &str) {
        if let Some(rel) = self.relationships.get_mut(child) {
            rel.parents.push(parent.to_string());
        }
        if let Some(rel) = self.relationships.get_mut(parent) {
            rel.children.push(child.to_string());
        }
    }

    fn display(&self) {
        for (name, rel) in &self.relationships {
            println!("{} has parents {:?} and children {:?}", name, rel.parents, rel.children);
        }
    }
}

fn main() {
    let mut family_tree = FamilyTree::new();
    // Adding persons
    family_tree.add_person("A");
    family_tree.add_person("B");
    family_tree.add_person("C");
    family_tree.add_person("D");
    family_tree.add_person("E");
    family_tree.add_person("Alice");
    family_tree.add_person("Bob");

    family_tree.add_parent_child_relationship("A", "B");
    family_tree.add_parent_child_relationship("B", "A");

    family_tree.add_parent_child_relationship("D", "C");
    family_tree.add_parent_child_relationship("C", "D");

    family_tree.add_parent_child_relationship("Alice", "Bob");
    family_tree.add_parent_child_relationship("Bob", "E");
    family_tree.add_parent_child_relationship("E", "Alice");

    family_tree.display();
}
