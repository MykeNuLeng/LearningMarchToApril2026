fn main() {
    let mut dwayne = Rock {
        name: String::from("Dwayne"),
        interactions: Vec::new(),
    };

    dwayne.interact(Interaction::Poke);
    dwayne.interact(Interaction::Wave);
    dwayne.interact(Interaction::Poke);
    dwayne.interact(Interaction::Pet);
    dwayne.interact(Interaction::Poke);

    match dwayne.attitude() {
        Attitude::Furious => println!("{} is shaking with rage", dwayne.name),
        Attitude::Blushing => println!("You can't be sure, but it looks like {} is blushing?", dwayne.name),
        Attitude::Stoic => println!("{} is just sitting there letting nothing affect them, not even the steady march of time", dwayne.name),
        Attitude::Upset => println!("You can see the top of {} starting to droop, must a {} become a diamond to get any attention?", dwayne.name, dwayne.name),
        Attitude::Dead => {
            println!("Oh no...");
            drop(dwayne);
        }
    }
}

#[derive(PartialEq)]
enum Interaction {
    Pet,
    Poke,
    Wave,
    Ignore,
    Admire,
    Pebbles,
}

enum Attitude {
    Furious,
    Blushing,
    Stoic,
    Upset,
    Dead,
}

struct Rock {
    name: String,
    interactions: Vec<Interaction>
}

impl Rock {
    fn interact(&mut self, interaction: Interaction) {
        self.interactions.push(interaction)
    }

    fn attitude(&self) -> Attitude {
        let total = self.interactions.len().min(7);
        let slice = &self.interactions[..total];

        if slice.iter().filter(|&x| x == &Interaction::Pebbles).count() >= 1 {
            return Attitude::Dead;
        } else if slice.iter().filter(|&x| x == &Interaction::Poke).count() >= 3 {
            return Attitude::Furious;
        } else if slice.iter().filter(|&x| x == &Interaction::Ignore).count() >= 3 {
            return Attitude::Upset;
        } else if slice.iter().filter(|&x| x == &Interaction::Admire).count() >= 4 {
            return Attitude::Blushing;
        }
        Attitude::Stoic
    }
}