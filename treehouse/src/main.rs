use std::io::stdin;

// As long as every member of the structure supports
// whatever we are deriving (in this case, Debug), we
// will get nice debug printing of the struct. All
// Rust's built-in types support it so... score!
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome to the tree house, {}", self.name)
            },
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => {
                println!("{} is now a probationary member", self.name)
            },
            VisitorAction::Refuse => {
                println!("Get {} the hell outta here!", self.name)
            },
        }
        println!("{}", self.greeting);
    }
    
}
fn whats_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed reading from stdin");
    return your_name
        .trim()
        .to_lowercase();
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new(
            "alice",
            "ello Alice. how's that looking glasss?",
            VisitorAction::Accept,
            23,
        ),
        Visitor::new(
           "bob",
           "hey, Bob.",
            VisitorAction::Accept,
           50,
        ),
        Visitor::new(
            "astraea",
            "Astraea! Didn't know you would be here!",
            VisitorAction::AcceptWithNote {
                note: String::from("There's milk in the fridge!")
            },
            0,
        ),
        Visitor::new(
            "brandon",
            "*grumble* Brandon... don't cause trouble like last time.",
            VisitorAction::Probation,
            36,
        ),
    ];

    loop {
        println!("ey. wot's ur name, govnah?");
        let name = whats_your_name();
        // {:?} is debug formatting
        println!("nice to meet ya, {} and whatnot. lemme check the list...", name);
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => {
                visitor.greet_visitor()
            },
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", &name);
                    visitor_list.push(Visitor::new(&name, "welcome, new friend!", VisitorAction::Probation, 0));
                }
            },
        }
    }

    println!("The final visitor list was:");
    println!("{:#?}", visitor_list);
}
