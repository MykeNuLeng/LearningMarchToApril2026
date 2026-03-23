use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use strum::{IntoEnumIterator, EnumIter};

fn main() {
    let mut pet_rock: Rock = confy::load("rock", None).unwrap();

    if pet_rock.name.is_empty() && pet_rock.interactions.is_empty() {
        pet_rock.name = Input::new()
            .with_prompt("Awww, you have a pet rock!\n What will you name them?")
            .interact_text()
            .unwrap();

        println!("I think {} is a great name", pet_rock.name);
    }

    let actions: Vec<&str> = Interaction::iter().map(|x| x.to_str()).collect();

    loop {
        let idx = Select::new()
            .with_prompt(format!("What will you do to {}", pet_rock.name))
            .items(&actions)
            .interact()
            .unwrap();

        match actions[idx] {
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

#[derive(PartialEq, Debug, Serialize, Deserialize, EnumIter)]
enum Interaction {
    Pet,
    Poke,
    Wave,
    Ignore,
    Admire,
    Pebbles,
}

impl Interaction {
    fn to_str(self) -> &'static str {
        match self {
            Interaction::Pet => "Pet",
            Interaction::Poke => "Poke",
            Interaction::Wave => "Wave",
            Interaction::Ignore => "Ignore",
            Interaction::Admire => "Admire",
            Interaction::Pebbles => "Pebbles",
        }
    }
}

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

        let (mut pokes, mut ignores, mut admires, mut pets, mut waves) = (0, 0, 0, 0, 0);

        for interaction in slice {
            match interaction {
                Interaction::Pet => pets += 1,
                Interaction::Poke => pokes += 1,
                Interaction::Wave => waves += 1,
                Interaction::Ignore => ignores += 1,
                Interaction::Admire => admires += 1,
                _ => (),
            }
        }

        if pokes >= 3 {
            println!("{} is shaking with rage\n", self.name);
        } else if ignores >= 3 {
            println!("You can see the top of {} starting to droop, must {} become a diamond to get any attention?\n", self.name, self.name);
        } else if admires >= 4 {
            println!("You can't be sure, but it looks like {} is blushing?\n", self.name);
        } else if pets >= 2 {
            println!("{} looks oddly content for a rock\n", self.name);
        } else if waves >= 2 {
            println!("{} is starting to think you're making fun of their lack of arms...\n", self.name);
        } else {
            println!("{} is just sitting there letting nothing affect them, not even the steady march of time\n", self.name);
        }
    }
}