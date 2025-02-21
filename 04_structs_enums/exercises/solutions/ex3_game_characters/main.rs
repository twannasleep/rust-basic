use std::io::{self, Write};

// Character types with associated abilities
#[derive(Debug, Clone)]
enum CharacterClass {
    Warrior {
        strength: u32,
        defense: u32,
        weapon_skill: u32,
    },
    Mage {
        intelligence: u32,
        mana: u32,
        spell_power: u32,
    },
    Rogue {
        agility: u32,
        stealth: u32,
        critical_chance: f32,
    },
}

// Item types that characters can use
#[derive(Debug, Clone)]
enum Item {
    Weapon {
        name: String,
        damage: u32,
        durability: u32,
    },
    Armor {
        name: String,
        defense: u32,
        weight: u32,
    },
    Potion {
        name: String,
        healing: u32,
        quantity: u32,
    },
}

// Character status effects
#[derive(Debug, Clone)]
enum StatusEffect {
    Poisoned { damage_per_turn: u32, turns_left: u32 },
    Strengthened { bonus: u32, turns_left: u32 },
    Weakened { penalty: u32, turns_left: u32 },
}

// Main character struct
#[derive(Debug)]
struct Character {
    name: String,
    class: CharacterClass,
    level: u32,
    health: u32,
    max_health: u32,
    experience: u32,
    inventory: Vec<Item>,
    status_effects: Vec<StatusEffect>,
}

impl Character {
    fn new(name: String, class: CharacterClass) -> Self {
        let max_health = match &class {
            CharacterClass::Warrior { .. } => 120,
            CharacterClass::Mage { .. } => 80,
            CharacterClass::Rogue { .. } => 100,
        };

        Character {
            name,
            class,
            level: 1,
            health: max_health,
            max_health,
            experience: 0,
            inventory: Vec::new(),
            status_effects: Vec::new(),
        }
    }

    fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
        if self.health == 0 {
            println!("{} has been defeated!", self.name);
        }
    }

    fn heal(&mut self, amount: u32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    fn use_item(&mut self, index: usize) -> Option<String> {
        if index >= self.inventory.len() {
            return Some("Invalid item index".to_string());
        }

        match &self.inventory[index] {
            Item::Potion { name, healing, .. } => {
                self.heal(*healing);
                self.inventory.remove(index);
                Some(format!("Used {} and healed {} health", name, healing))
            }
            _ => Some("This item cannot be used directly".to_string()),
        }
    }

    fn add_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects.push(effect);
    }

    fn update_status_effects(&mut self) {
        let mut i = 0;
        while i < self.status_effects.len() {
            match &mut self.status_effects[i] {
                StatusEffect::Poisoned { damage_per_turn, turns_left } => {
                    self.take_damage(*damage_per_turn);
                    *turns_left -= 1;
                }
                StatusEffect::Strengthened { turns_left, .. } |
                StatusEffect::Weakened { turns_left, .. } => {
                    *turns_left -= 1;
                }
            }

            if matches!(self.status_effects[i],
                StatusEffect::Poisoned { turns_left: 0, .. } |
                StatusEffect::Strengthened { turns_left: 0, .. } |
                StatusEffect::Weakened { turns_left: 0, .. }) {
                self.status_effects.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn attack(&self, target: &mut Character) {
        let base_damage = match &self.class {
            CharacterClass::Warrior { strength, weapon_skill, .. } => {
                strength + weapon_skill
            }
            CharacterClass::Mage { intelligence, spell_power, .. } => {
                intelligence + spell_power
            }
            CharacterClass::Rogue { agility, critical_chance, .. } => {
                let crit = rand::random::<f32>() < *critical_chance;
                let base = agility;
                if crit { base * 2 } else { base }
            }
        };

        target.take_damage(base_damage);
        println!("{} attacks {} for {} damage!", self.name, target.name, base_damage);
    }

    fn display_status(&self) {
        println!("\nCharacter Status:");
        println!("----------------");
        println!("Name: {}", self.name);
        println!("Class: {:?}", self.class);
        println!("Level: {}", self.level);
        println!("Health: {}/{}", self.health, self.max_health);
        println!("Experience: {}", self.experience);
        
        println!("\nInventory:");
        for (i, item) in self.inventory.iter().enumerate() {
            println!("{}. {:?}", i + 1, item);
        }

        println!("\nStatus Effects:");
        for effect in &self.status_effects {
            println!("- {:?}", effect);
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Game Character System");
    println!("--------------------");

    // Create a character
    println!("\nCreate your character:");
    let name = get_user_input("Enter character name: ");
    
    println!("\nSelect class:");
    println!("1. Warrior");
    println!("2. Mage");
    println!("3. Rogue");

    let class = match get_user_input("Enter choice (1-3): ").as_str() {
        "1" => CharacterClass::Warrior {
            strength: 15,
            defense: 10,
            weapon_skill: 8,
        },
        "2" => CharacterClass::Mage {
            intelligence: 15,
            mana: 100,
            spell_power: 12,
        },
        "3" => CharacterClass::Rogue {
            agility: 12,
            stealth: 10,
            critical_chance: 0.2,
        },
        _ => {
            println!("Invalid choice. Defaulting to Warrior.");
            CharacterClass::Warrior {
                strength: 15,
                defense: 10,
                weapon_skill: 8,
            }
        }
    };

    let mut character = Character::new(name, class);

    // Add some starting items
    character.add_item(Item::Weapon {
        name: "Basic Sword".to_string(),
        damage: 10,
        durability: 100,
    });
    character.add_item(Item::Potion {
        name: "Health Potion".to_string(),
        healing: 50,
        quantity: 3,
    });

    // Main game loop
    loop {
        println!("\nOptions:");
        println!("1. Display status");
        println!("2. Use item");
        println!("3. Add status effect (test)");
        println!("4. Update status effects");
        println!("5. Quit");

        let choice = get_user_input("\nSelect option (1-5): ");

        match choice.as_str() {
            "1" => character.display_status(),
            "2" => {
                if character.inventory.is_empty() {
                    println!("No items in inventory!");
                    continue;
                }

                println!("\nInventory:");
                for (i, item) in character.inventory.iter().enumerate() {
                    println!("{}. {:?}", i + 1, item);
                }

                let index = get_user_input("Enter item number to use: ")
                    .parse::<usize>()
                    .unwrap_or(0)
                    .saturating_sub(1);

                if let Some(message) = character.use_item(index) {
                    println!("{}", message);
                }
            }
            "3" => {
                println!("\nAdd test status effect:");
                println!("1. Poison");
                println!("2. Strengthen");
                println!("3. Weaken");

                let effect = match get_user_input("Select effect (1-3): ").as_str() {
                    "1" => StatusEffect::Poisoned {
                        damage_per_turn: 5,
                        turns_left: 3,
                    },
                    "2" => StatusEffect::Strengthened {
                        bonus: 10,
                        turns_left: 3,
                    },
                    "3" => StatusEffect::Weakened {
                        penalty: 5,
                        turns_left: 2,
                    },
                    _ => {
                        println!("Invalid choice!");
                        continue;
                    }
                };

                character.add_status_effect(effect);
                println!("Status effect added!");
            }
            "4" => {
                character.update_status_effects();
                println!("Status effects updated!");
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select 1-5."),
        }
    }
} 