use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};

fn main() {
    let mut pet_rock: Rock = confy::load("rock", None).unwrap();

    if pet_rock.name.is_empty() && pet_rock.interactions.is_empty() {
        pet_rock.name = Input::new()
            .with_prompt("Awww, you have a pet rock!\n What will you name them?")
            .interact_text()
            .unwrap();

        println!("I think {} is a great name", pet_rock.name);
    }

    loop {
        let idx = Select::new()
            .with_prompt(format!("What will you do to {}", pet_rock.name))
            .items(ACTIONS)
            .interact()
            .unwrap();

        match ACTIONS[idx] {
            "Pebbles" => {
                println!("Oh no...");
                pet_rock = Rock::default();
                break;
            }
            "Pet" => pet_rock.interact(Interaction::Pet),
            "Poke" => pet_rock.interact(Interaction::Poke),
            "Wave" => pet_rock.interact(Interaction::Wave),
            "Ignore" => pet_rock.interact(Interaction::Ignore),
            "Admire" => pet_rock.interact(Interaction::Admire),
            _ => (),
        }
        
        pet_rock.attitude();
        confy::store("rock", None, &pet_rock).unwrap();
    }

    confy::store("rock", None, &pet_rock).unwrap();
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
enum Interaction {
    Pet,
    Poke,
    Wave,
    Ignore,
    Admire,
    Pebbles,
}

const ACTIONS: &[&str] = &[
    "Pet",
    "Poke",
    "Wave",
    "Ignore",
    "Admire",
    "Pebbles",
];

#[derive(Serialize, Deserialize, Default, Debug)]
struct Rock {
    name: String,
    interactions: Vec<Interaction>
}

impl Rock {
    fn interact(&mut self, interaction: Interaction) {
        self.interactions.push(interaction)
    }

    fn attitude(&self) {
        let start = self.interactions.len().saturating_sub(7);
        let slice = &self.interactions[start..];

        if slice.iter().filter(|&x| x == &Interaction::Poke).count() >= 3 {
            println!("{} is shaking with rage\n", self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Ignore).count() >= 3 {
            println!("You can see the top of {} starting to droop, must {} become a diamond to get any attention?\n", self.name, self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Admire).count() >= 4 {
            println!("You can't be sure, but it looks like {} is blushing?\n", self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Pet).count() >= 2 {
            println!("{} looks oddly content for a rock\n", self.name);
        } else if slice.iter().filter(|&x| x == &Interaction::Wave).count() >= 2 {
            println!("{} is starting to think you're making fun of their lack of arms...\n", self.name);
        }else {
            println!("{} is just sitting there letting nothing affect them, not even the steady march of time\n", self.name);
        }
    }
}