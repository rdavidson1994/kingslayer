use std::env;

use kingslayer::Cli;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let cli = Cli::from_ron_file(&path);

        cli.start();
    } else {
        let cli = Cli::from_ron_str(
            r#"(
curr_room: "Circle Room",
rooms: {
"Closet": Room(
    name: "Closet",
    desc: "This isn't a very large or clean closet.",
    paths: {
        "door": Pathway(
            target: "Circle Room",
            desc: "The door leads back into the room.",
            inspect: "The door is plain and wooden.",
            opening: Some(Open)
        ),
    },
),
"Small Cave": Room(
    name: "Small Cave",
    desc: "The cave is dark and musty.",
    paths: {
        "s": Pathway(
            target: "Circle Room",
            desc: "The opening of the cave leads south back into the circle room.",
            inspect: "The edges of the opening are cracked and rough."
        ),
    },
    allies: [
        Ally(
            hp: 1,
            name: "old man",
            desc: "An old man stoically stands at the back of the cave.",
            inspect: "\"Take the sword, brave one.\""
        )
    ],
    items: [
        Weapon(Weapon(
            name: "iron sword",
            desc: "There is an iron sword on the ground.",
            inspect: "The iron sword is of crude workmanship, but it should be usable.",
            damage: 6   
        )),
    ],
),
"Circle Room": Room(
    name: "Circle Room",
    desc: "You stand in a circular room crafted in stone.",
    paths: {
        "door": Pathway(
            target: "Closet",
            desc: "There is a door on one side.",
            inspect: "The door is plain and wooden.",
            opening: Some(Closed)
        ),
        "n": Pathway(
            target: "Small Cave",
            desc: "There is a mouth of a cave to the north.",
            inspect: "The cave opening glows with a soft flickering light."
        ),
        "s": Pathway(
            target: "Long Hallway",
            desc: "",
            inspect: ""
        ),
        "hallway": Pathway(
            target: "Long Hallway",
            desc: "There is a hallway to the south.",
            inspect: "The hallway seems narrow and foreboding."
        ),
        "e": Pathway(
            target: "Next Room E",
            desc: "There is a pathway to the east.",
            inspect: "The opening is spacious."
        ),
        "w": Pathway(
            target: "Next Room W",
            desc: "There is a pathway to the west.",
            inspect: "The opening is spacious."
        ),
    },
    items: [
        Armor(Armor(
            name: "leather armor",
            desc: "There is a set of leather armor lying in a heap.",
            inspect: "The armor is worn but light and sturdy.",
            ac: 11,
        )),
        Thing(Thing(
            name: "leaf",
            desc: "A leaf lies on the ground.",
            inspect: "It's small, brown, and dry."
        )),
    ]
),
"Next Room W": Room(
    name: "Next Room",
    desc: "You are in the next room over.",
    paths: {
        "e": Pathway(
            target: "Circle Room",
            desc: "There is a pathway to the east.",
            inspect: "The path expands into a larger room."
        ),
    },
),
"Next Room E": Room(
    name: "Next Room",
    desc: "You are in the next room over.",
    paths: {
        "w": Pathway(
            target: "Circle Room",
            desc: "There is a pathway to the west.",
            inspect: "The path expands into a larger room."
        ),
    },
    enemies: [
        Enemy(
            hp: 10,
            xp: 15,
            damage: 4,
            name: "ogre",
            desc: "An ogre stands there menacingly with a club.",
            inspect: "The ogre is holding a long club made of wood. He is wearing nothing but a loincloth and a leather jerkin.",
            is_angry: false,
            loot: [
                Weapon(Weapon(
                    name: "club",
                    desc: "There is a club on the ground.",
                    inspect: "The club is very thick and heavy.",
                    damage: 4
                ))
            ]
        ),
        Enemy(
            hp: 7,
            xp: 10,
            damage: 2,
            name: "goblin",
            desc: "A goblin cowers in a corner holding a knife.",
            inspect: "The goblin is small, grey-green, and skinny. He is brandishing a knife.",
            is_angry: false,
        ),
    ],
),
"Long Hallway": Room(
    name: "Long Hallway",
    desc: "You are in a long, dark hallway.",
    paths: {
        "n": Pathway(
            target: "Circle Room",
            desc: "There is a room to the north.",
            inspect: "The hallway seems narrow and foreboding."
        )
    },
    items: [
        Container(Container(
            name: "large capsule",
            desc: "There is a large capsule here.",
            inspect: "The capsule appears to be able to hold things.",
            opening: Open,
            contents: [
                Thing(Thing(
                    name: "curious object",
                    desc: "There is a curious object here.",
                    inspect: "The object's appearance confuses your weak mind."
                )),
            ]
        )),
    ]
)
}
)"#,
        );

        cli.start();
    }
}
