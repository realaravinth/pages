/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::settings::Settings;

// source: https://www.randomlists.com/data/nouns.json
const LEN: usize = 876;
const WORDLIST: [&str; LEN] = [
    "account",
    "achiever",
    "acoustics",
    "act",
    "action",
    "activity",
    "actor",
    "addition",
    "adjustment",
    "advertisement",
    "advice",
    "aftermath",
    "afternoon",
    "afterthought",
    "agreement",
    "air",
    "airplane",
    "airport",
    "alarm",
    "amount",
    "amusement",
    "anger",
    "angle",
    "animal",
    "ants",
    "apparatus",
    "apparel",
    "appliance",
    "approval",
    "arch",
    "argument",
    "arithmetic",
    "arm",
    "army",
    "art",
    "attack",
    "attraction",
    "aunt",
    "authority",
    "babies",
    "baby",
    "back",
    "badge",
    "bag",
    "bait",
    "balance",
    "ball",
    "base",
    "baseball",
    "basin",
    "basket",
    "basketball",
    "bat",
    "bath",
    "battle",
    "bead",
    "bear",
    "bed",
    "bedroom",
    "beds",
    "bee",
    "beef",
    "beginner",
    "behavior",
    "belief",
    "believe",
    "bell",
    "bells",
    "berry",
    "bike",
    "bikes",
    "bird",
    "birds",
    "birth",
    "birthday",
    "bit",
    "bite",
    "blade",
    "blood",
    "blow",
    "board",
    "boat",
    "bomb",
    "bone",
    "book",
    "books",
    "boot",
    "border",
    "bottle",
    "boundary",
    "box",
    "boy",
    "brake",
    "branch",
    "brass",
    "breath",
    "brick",
    "bridge",
    "brother",
    "bubble",
    "bucket",
    "building",
    "bulb",
    "burst",
    "bushes",
    "business",
    "butter",
    "button",
    "cabbage",
    "cable",
    "cactus",
    "cake",
    "cakes",
    "calculator",
    "calendar",
    "camera",
    "camp",
    "can",
    "cannon",
    "canvas",
    "cap",
    "caption",
    "car",
    "card",
    "care",
    "carpenter",
    "carriage",
    "cars",
    "cart",
    "cast",
    "cat",
    "cats",
    "cattle",
    "cause",
    "cave",
    "celery",
    "cellar",
    "cemetery",
    "cent",
    "chalk",
    "chance",
    "change",
    "channel",
    "cheese",
    "cherries",
    "cherry",
    "chess",
    "chicken",
    "chickens",
    "children",
    "chin",
    "church",
    "circle",
    "clam",
    "class",
    "cloth",
    "clover",
    "club",
    "coach",
    "coal",
    "coast",
    "coat",
    "cobweb",
    "coil",
    "collar",
    "color",
    "committee",
    "company",
    "comparison",
    "competition",
    "condition",
    "connection",
    "control",
    "cook",
    "copper",
    "corn",
    "cough",
    "country",
    "cover",
    "cow",
    "cows",
    "crack",
    "cracker",
    "crate",
    "crayon",
    "cream",
    "creator",
    "creature",
    "credit",
    "crib",
    "crime",
    "crook",
    "crow",
    "crowd",
    "crown",
    "cub",
    "cup",
    "current",
    "curtain",
    "curve",
    "cushion",
    "dad",
    "daughter",
    "day",
    "death",
    "debt",
    "decision",
    "deer",
    "degree",
    "design",
    "desire",
    "desk",
    "destruction",
    "detail",
    "development",
    "digestion",
    "dime",
    "dinner",
    "dinosaurs",
    "direction",
    "dirt",
    "discovery",
    "discussion",
    "distance",
    "distribution",
    "division",
    "dock",
    "doctor",
    "dog",
    "dogs",
    "doll",
    "dolls",
    "donkey",
    "door",
    "downtown",
    "drain",
    "drawer",
    "dress",
    "drink",
    "driving",
    "drop",
    "duck",
    "ducks",
    "dust",
    "ear",
    "earth",
    "earthquake",
    "edge",
    "education",
    "effect",
    "egg",
    "eggnog",
    "eggs",
    "elbow",
    "end",
    "engine",
    "error",
    "event",
    "example",
    "exchange",
    "existence",
    "expansion",
    "experience",
    "expert",
    "eye",
    "eyes",
    "face",
    "fact",
    "fairies",
    "fall",
    "fang",
    "farm",
    "fear",
    "feeling",
    "field",
    "finger",
    "fire",
    "fireman",
    "fish",
    "flag",
    "flame",
    "flavor",
    "flesh",
    "flight",
    "flock",
    "floor",
    "flower",
    "flowers",
    "fly",
    "fog",
    "fold",
    "food",
    "foot",
    "force",
    "fork",
    "form",
    "fowl",
    "frame",
    "friction",
    "friend",
    "friends",
    "frog",
    "frogs",
    "front",
    "fruit",
    "fuel",
    "furniture",
    "gate",
    "geese",
    "ghost",
    "giants",
    "giraffe",
    "girl",
    "girls",
    "glass",
    "glove",
    "gold",
    "government",
    "governor",
    "grade",
    "grain",
    "grandfather",
    "grandmother",
    "grape",
    "grass",
    "grip",
    "ground",
    "group",
    "growth",
    "guide",
    "guitar",
    "gun",
    "hair",
    "haircut",
    "hall",
    "hammer",
    "hand",
    "hands",
    "harbor",
    "harmony",
    "hat",
    "hate",
    "head",
    "health",
    "heat",
    "hill",
    "history",
    "hobbies",
    "hole",
    "holiday",
    "home",
    "honey",
    "hook",
    "hope",
    "horn",
    "horse",
    "horses",
    "hose",
    "hospital",
    "hot",
    "hour",
    "house",
    "houses",
    "humor",
    "hydrant",
    "ice",
    "icicle",
    "idea",
    "impulse",
    "income",
    "increase",
    "industry",
    "ink",
    "insect",
    "instrument",
    "insurance",
    "interest",
    "invention",
    "iron",
    "island",
    "jail",
    "jam",
    "jar",
    "jeans",
    "jelly",
    "jellyfish",
    "jewel",
    "join",
    "judge",
    "juice",
    "jump",
    "kettle",
    "key",
    "kick",
    "kiss",
    "kittens",
    "kitty",
    "knee",
    "knife",
    "knot",
    "knowledge",
    "laborer",
    "lace",
    "ladybug",
    "lake",
    "lamp",
    "land",
    "language",
    "laugh",
    "leather",
    "leg",
    "legs",
    "letter",
    "letters",
    "lettuce",
    "level",
    "library",
    "limit",
    "line",
    "linen",
    "lip",
    "liquid",
    "loaf",
    "lock",
    "locket",
    "look",
    "loss",
    "love",
    "low",
    "lumber",
    "lunch",
    "lunchroom",
    "machine",
    "magic",
    "maid",
    "mailbox",
    "man",
    "marble",
    "mark",
    "market",
    "mask",
    "mass",
    "match",
    "meal",
    "measure",
    "meat",
    "meeting",
    "memory",
    "men",
    "metal",
    "mice",
    "middle",
    "milk",
    "mind",
    "mine",
    "minister",
    "mint",
    "minute",
    "mist",
    "mitten",
    "mom",
    "money",
    "monkey",
    "month",
    "moon",
    "morning",
    "mother",
    "motion",
    "mountain",
    "mouth",
    "move",
    "muscle",
    "name",
    "nation",
    "neck",
    "need",
    "needle",
    "nerve",
    "nest",
    "night",
    "noise",
    "north",
    "nose",
    "note",
    "notebook",
    "number",
    "nut",
    "oatmeal",
    "observation",
    "ocean",
    "offer",
    "office",
    "oil",
    "orange",
    "oranges",
    "order",
    "oven",
    "page",
    "pail",
    "pan",
    "pancake",
    "paper",
    "parcel",
    "part",
    "partner",
    "party",
    "passenger",
    "payment",
    "peace",
    "pear",
    "pen",
    "pencil",
    "person",
    "pest",
    "pet",
    "pets",
    "pickle",
    "picture",
    "pie",
    "pies",
    "pig",
    "pigs",
    "pin",
    "pipe",
    "pizzas",
    "place",
    "plane",
    "planes",
    "plant",
    "plantation",
    "plants",
    "plastic",
    "plate",
    "play",
    "playground",
    "pleasure",
    "plot",
    "plough",
    "pocket",
    "point",
    "poison",
    "pollution",
    "popcorn",
    "porter",
    "position",
    "pot",
    "potato",
    "powder",
    "power",
    "price",
    "produce",
    "profit",
    "property",
    "prose",
    "protest",
    "pull",
    "pump",
    "punishment",
    "purpose",
    "push",
    "quarter",
    "quartz",
    "queen",
    "question",
    "quicksand",
    "quiet",
    "quill",
    "quilt",
    "quince",
    "quiver",
    "rabbit",
    "rabbits",
    "rail",
    "railway",
    "rain",
    "rainstorm",
    "rake",
    "range",
    "rat",
    "rate",
    "ray",
    "reaction",
    "reading",
    "reason",
    "receipt",
    "recess",
    "record",
    "regret",
    "relation",
    "religion",
    "representative",
    "request",
    "respect",
    "rest",
    "reward",
    "rhythm",
    "rice",
    "riddle",
    "rifle",
    "ring",
    "rings",
    "river",
    "road",
    "robin",
    "rock",
    "rod",
    "roll",
    "roof",
    "room",
    "root",
    "rose",
    "route",
    "rub",
    "rule",
    "run",
    "sack",
    "sail",
    "salt",
    "sand",
    "scale",
    "scarecrow",
    "scarf",
    "scene",
    "scent",
    "school",
    "science",
    "scissors",
    "screw",
    "sea",
    "seashore",
    "seat",
    "secretary",
    "seed",
    "selection",
    "self",
    "sense",
    "servant",
    "shade",
    "shake",
    "shame",
    "shape",
    "sheep",
    "sheet",
    "shelf",
    "ship",
    "shirt",
    "shock",
    "shoe",
    "shoes",
    "shop",
    "show",
    "side",
    "sidewalk",
    "sign",
    "silk",
    "silver",
    "sink",
    "sister",
    "sisters",
    "size",
    "skate",
    "skin",
    "skirt",
    "sky",
    "slave",
    "sleep",
    "sleet",
    "slip",
    "slope",
    "smash",
    "smell",
    "smile",
    "smoke",
    "snail",
    "snails",
    "snake",
    "snakes",
    "sneeze",
    "snow",
    "soap",
    "society",
    "sock",
    "soda",
    "sofa",
    "son",
    "song",
    "songs",
    "sort",
    "sound",
    "soup",
    "space",
    "spade",
    "spark",
    "spiders",
    "sponge",
    "spoon",
    "spot",
    "spring",
    "spy",
    "square",
    "squirrel",
    "stage",
    "stamp",
    "star",
    "start",
    "statement",
    "station",
    "steam",
    "steel",
    "stem",
    "step",
    "stew",
    "stick",
    "sticks",
    "stitch",
    "stocking",
    "stomach",
    "stone",
    "stop",
    "store",
    "story",
    "stove",
    "stranger",
    "straw",
    "stream",
    "street",
    "stretch",
    "string",
    "structure",
    "substance",
    "sugar",
    "suggestion",
    "suit",
    "summer",
    "sun",
    "support",
    "surprise",
    "sweater",
    "swim",
    "swing",
    "system",
    "table",
    "tail",
    "talk",
    "tank",
    "taste",
    "tax",
    "teaching",
    "team",
    "teeth",
    "temper",
    "tendency",
    "tent",
    "territory",
    "test",
    "texture",
    "theory",
    "thing",
    "things",
    "thought",
    "thread",
    "thrill",
    "throat",
    "throne",
    "thumb",
    "thunder",
    "ticket",
    "tiger",
    "time",
    "tin",
    "title",
    "toad",
    "toe",
    "toes",
    "tomatoes",
    "tongue",
    "tooth",
    "toothbrush",
    "toothpaste",
    "top",
    "touch",
    "town",
    "toy",
    "toys",
    "trade",
    "trail",
    "train",
    "trains",
    "tramp",
    "transport",
    "tray",
    "treatment",
    "tree",
    "trees",
    "trick",
    "trip",
    "trouble",
    "trousers",
    "truck",
    "trucks",
    "tub",
    "turkey",
    "turn",
    "twig",
    "twist",
    "umbrella",
    "uncle",
    "underwear",
    "unit",
    "use",
    "vacation",
    "value",
    "van",
    "vase",
    "vegetable",
    "veil",
    "vein",
    "verse",
    "vessel",
    "vest",
    "view",
    "visitor",
    "voice",
    "volcano",
    "volleyball",
    "voyage",
    "walk",
    "wall",
    "war",
    "wash",
    "waste",
    "watch",
    "water",
    "wave",
    "waves",
    "wax",
    "way",
    "wealth",
    "weather",
    "week",
    "weight",
    "wheel",
    "whip",
    "whistle",
    "wilderness",
    "wind",
    "window",
    "wine",
    "wing",
    "winter",
    "wire",
    "wish",
    "woman",
    "women",
    "wood",
    "wool",
    "word",
    "work",
    "worm",
    "wound",
    "wren",
    "wrench",
    "wrist",
    "writer",
    "writing",
    "yak",
    "yam",
    "yard",
    "yarn",
    "year",
    "yoke",
    "zebra",
    "zephyr",
    "zinc",
    "zipper",
    "zoo",
];

struct ID<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
}

impl<'a> ID<'a> {
    fn hostname(&self, base_domain: &str) -> String {
        format!(
            "{}-{}-{}.{}",
            self.first, self.second, self.third, base_domain
        )
    }
}

fn get_random_id() -> ID<'static> {
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    let mut rng: ThreadRng = thread_rng();
    let first: usize = rng.gen_range(0..LEN);
    let mut second: usize;
    let mut third: usize;

    loop {
        second = rng.gen_range(0..LEN);
        if second != first {
            break;
        }
    }

    loop {
        third = rng.gen_range(0..LEN);
        if third != first && second != third {
            break;
        }
    }

    let first = WORDLIST[first];
    let second = WORDLIST[second];
    let third = WORDLIST[third];
    ID {
        first,
        second,
        third,
    }
}

pub fn get_random_subdomain(s: &Settings) -> String {
    let id = get_random_id();
    id.hostname(&s.page.base_domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdomains() {
        // test random ID
        let id = get_random_id();
        assert_ne!(id.first, id.second);
        assert_ne!(id.first, id.third);
        assert_ne!(id.third, id.second);

        // test ID::hostname
        let delimiter = "foobar21312";
        assert_eq!(
            id.hostname(delimiter),
            format!("{}-{}-{}.{delimiter}", id.first, id.second, id.third,)
        );
    }
}
