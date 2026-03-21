use dialoguer::Select;

fn main() {
    let mut dwayne = Rock {
        name: String::from("Dwayne"),
        interactions: Vec::new(),
    };

    let labels: Vec<&str> = ACTIONS.iter().map(|label| label.0).collect();

    loop {
        let idx = Select::new()
            .with_prompt(format!("What will you do to {}", dwayne.name))
            .items(&labels)
            .interact()
            .unwrap();

        match &ACTIONS[idx].1 {
            Interaction::Pebbles => {
                println!("Oh no...");
                break;
            }
            Interaction::Pet => dwayne.interact(Interaction::Pet),
            Interaction::Poke => dwayne.interact(Interaction::Poke),
            Interaction::Wave => dwayne.interact(Interaction::Wave),
            Interaction::Ignore => dwayne.interact(Interaction::Ignore),
            Interaction::Admire => dwayne.interact(Interaction::Admire),
        }
        
        dwayne.attitude();
    }
}

#[derive(PartialEq, Debug)]
enum Interaction {
    Pet,
    Poke,
    Wave,
    Ignore,
    Admire,
    Pebbles,
}

const ACTIONS: &[(&str, Interaction)] = &[
    ("Pet", Interaction::Pet),
    ("Poke", Interaction::Poke),
    ("Wave", Interaction::Wave),
    ("Ignore", Interaction::Ignore),
    ("Admire", Interaction::Admire),
    ("Pebbles", Interaction::Pebbles),
];

struct Rock {
    name: String,
    interactions: Vec<Interaction>
}

impl Rock {
    fn interact(&mut self, interaction: Interaction) {
        self.interactions.push(interaction)
    }

    fn attitude(&self) {
        let start = if self.interactions.len() < 7 {0} else {self.interactions.len() - 7};
        let slice = &self.interactions[start..];

        if slice.iter().filter(|&x| x == &Interaction::Poke).count() >= 3 {
            println!("{} is shaking with rage\n", self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Ignore).count() >= 3 {
            println!("You can see the top of {} starting to droop, must {} become a diamond to get any attention?\n", self.name, self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Admire).count() >= 4 {
            println!("You can't be sure, but it looks like {} is blushing?\n", self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Pet).count() >= 2 {
            println!("{} looks oddly content for a rock\n", self.name);
        } else {
            println!("{} is just sitting there letting nothing affect them, not even the steady march of time\n", self.name);
        }
    }
}