macro_rules! item_types {
    (
        $(
            $variant:ident = $id:expr => $name:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ItemType {
            $(
                $variant = $id,
            )*
        }

        impl ItemType {
            pub fn from_id(id: &u16) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> u16 {
                match self {
                    $(
                        Self::$variant => $id,
                    )*
                }
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }
        }
    };
}

item_types! {
    Plug = 1 => "Plug",
    Food = 2 => "Food",
    Trophy = 3 => "Trophy",
    Siege = 4 => "Siege",
    Potion = 5 => "Potion",
    StyleBook = 6 => "Style Book",
    Tool = 7 => "Tool",
    Ingredient = 8 => "Ingredient",
    Drink = 10 => "Drink",
    Costume = 11 => "Costume",
    Disguise = 12 => "Disguise",
    Lure = 14 => "Lure",
    Container = 16 => "Container",

    Head = 17 => "Head/None",
    Amulet = 18 => "Necklace",
    Chest = 19 => "Chest",
    Shoulders = 20 => "Shoulders",
    Waist = 21 => "Waist",
    Legs = 22 => "Legs",
    Feet = 23 => "Feet",
    Ring = 24 => "Ring",
    Hand = 25 => "Hand",

    LightHead = 26 => "Hat",
    LightNeck = 27 => "Neck/Light",
    LightChest = 28 => "Robe",
    LightShoulders = 29 => "Epaulets",
    LightWaist = 30 => "Sash",
    LightLegs = 31 => "Breeches",
    LightFeet = 32 => "Shoes",
    LightRing = 33 => "Ring/Light",
    LightHands = 34 => "Gloves",

    MediumHead = 35 => "Helmet",
    MediumNeck = 36 => "Neck/Medium",
    MediumChest = 37 => "Jack",
    MediumShoulders = 38 => "Arm Cops",
    MediumWaist = 39 => "Belt",
    MediumLegs = 40 => "Guards",
    MediumFeet = 41 => "Boots",
    MediumRing = 42 => "Ring/Medium",
    MediumHands = 43 => "Bracers",

    HeavyHelm = 44 => "Helm",
    HeavyNeck = 45 => "Neck/Heavy",
    HeavyChestplate = 46 => "Cuirass",
    HeavyShoulders = 47 => "Pauldron",
    HeavyWaist = 48 => "Girdle",
    HeavyLegs = 49 => "Greaves",
    HeavyFeet = 50 => "Sabatons",
    HeavyRing = 51 => "Ring/Heavy",
    HeavyHands = 52 => "Gauntlets",

    OneHandedAxe = 53 => "Axe",
    OneHandedMace = 56 => "Mace",
    OneHandedSword = 59 => "Sword",
    OneHandedDagger = 62 => "Dagger",
    Shield = 65 => "Shield",
    RuneOffHand = 66 => "Rune/Off Hand",

    TwoHandedSword = 67 => "Greatsword",
    TwoHandedAxe = 68 => "Battle Axe",
    TwoHandedMaul = 69 => "Maul",
    Bow = 70 => "Bow",
    RestorationStaff = 71 => "Restoration Staff",
    InfernoStaff = 72 => "Inferno Staff",
    IceStaff = 73 => "Ice Staff",
    LightningStaff = 74 => "Lightning Staff",

    Jerkin = 75 => "Jerkin",
    Bread = 76 => "Bread",
    Meat = 77 => "Meat",
    Stew = 78 => "Stew",
    Wine = 80 => "Wine",
    Spirits = 81 => "Spirits",
    Beer = 82 => "Beer",

    HealthPotion = 84 => "Health Potion",
    HmsPotion = 86 => "HMS Potion",
    MagickaPotion = 87 => "Magicka Potion",
    StaminaPotion = 89 => "Stamina Potion",

    SoulGem = 112 => "Soul Gem",
    RepairKit = 113 => "Repair Kit",
    Rune = 114 => "Rune",
    SpecialFurnishings = 115 => "Special Furnishings",
    WeaponGlyph = 116 => "Weapon Glyph",
    ArmorGlyph = 117 => "Armor Glyph",
    JewelryGlyph = 118 => "Jewelry Glyph",
    Scroll = 120 => "Scroll",
    Reagent = 121 => "Reagent",
    AlchemyBase = 122 => "Alchemy Base",
    Collectible = 123 => "Collectible",

    Ore = 124 => "Ore",
    Ingot = 125 => "Ingot",
    Wood = 126 => "Wood",
    Plank = 127 => "Plank",
    Fiber = 128 => "Fiber",
    Cloth = 129 => "Cloth",
    Temper = 130 => "Temper",
    Rosin = 131 => "Rosin",
    Wax = 132 => "Wax",
    StyleMaterial = 133 => "Style Material",
    TraitStoneArmour = 134 => "Armour Trait Stone",
    TraitStoneWeapon = 135 => "Weapon Trait Stone",

    Trash = 153 => "Trash",
    CyrodiilRepairKit = 155 => "Repair Kit",
    NonCombatPet = 156 => "Non-Combat Pet",
    GuildTabard = 159 => "Guild Tabard",

    AspectRune = 160 => "Rune",
    EssenceRune = 161 => "Rune",
    PotencyRune = 162 => "Rune",

    StyleMaterialFragment = 165 => "Style Material Fragment",
    Fish = 166 => "Fish",
    Treasure = 167 => "Treasure",
    TreasureRepairKit = 168 => "Repair Kit",
    CrownStoreItem = 173 => "Crown Store Item",

    HealthPotion2 = 175 => "Health Potion",
    MagickaPotion2 = 177 => "Magicka Potion",
    MagickaPotion3 = 178 => "Magicka Potion",
    StaminaPotion2 = 179 => "Stamina Potion",

    HotPotion = 203 => "Hot Potion",
    HotPoison = 204 => "Hot Poison",
    DotPotion = 205 => "DoT Potion",
    DotPoison = 206 => "DoT Poison",
    ProtectionPotion = 207 => "Protection Potion",
    ProtectionPoison = 208 => "Protection Poison",
    VulnerabilityPotion = 209 => "Vulnerability Potion",
    VulnerabilityPoison = 210 => "Vulnerability Poison",
    VitalityPotion = 211 => "Vitality Potion",
    VitalityPoison = 212 => "Vitality Poison",
    DefilePotion = 213 => "Defile Potion",
    DefilePoison = 214 => "Defile Poison",

    TreasureMap = 215 => "Treasure Map",
    SurveyReport = 216 => "Survey Report",
    KeyFragment = 217 => "Key Fragment",
    MuseumPiece = 218 => "Museum Piece",
    RecipeFragment = 219 => "Recipe Fragment",

    MaterialUpgrader = 221 => "Material Upgrader",
    Key = 222 => "Key",
    MonsterTrophy = 223 => "Monster Trophy",
    RareFish = 224 => "Rare Fish",
    ReagentHerb = 225 => "Reagent (Herb)",
    ReagentAnimalPart = 227 => "Reagent (Animal Part)",

    RareIngredient = 238 => "Ingredient (Rare)",
    CollectibleDyeStamp = 240 => "Collectible Dye Stamp",
    Furniture = 242 => "Furniture",
    Toy = 244 => "Toy",

    Regulus = 245 => "Regulus",
    Bast = 246 => "Bast",
    CleanPelt = 247 => "Clean Pelt",
    MundaneRune = 248 => "Mundane Rune",
    AlchemicalResin = 249 => "Alchemical Resin",
    DecorativeWax = 250 => "Decorative Wax",
    Heartwood = 251 => "Heartwood",

    Diagram = 252 => "Diagram",
    Blueprint = 253 => "Blueprint",
    Pattern = 254 => "Pattern",
    Formula = 255 => "Formula",
    Praxis = 256 => "Praxis",
    Design = 257 => "Design",
    Writ = 259 => "Writ",
    TargetDummy = 260 => "Target Dummy",
    RuneboxFragment = 261 => "Runebox Fragment",
    EventContainer = 262 => "Event Container",

    Dust = 263 => "Dust",
    Ounce = 264 => "Ounce",
    Plating = 265 => "Plating",
    PlatingDust = 266 => "Plating Dust",
    TraitDust = 267 => "Trait Dust",
    TraitStoneJewellery = 268 => "Jewellery Trait Stone",
    Orpiment = 269 => "Orpiment",
    Sketch = 271 => "Sketch",
    KeepRecallStone = 274 => "Keep Recall Stone",
    CurrencyContainer = 276 => "Currency Container",
    HolidayWrit = 277 => "Holiday Writ",

    HeroPotion = 279 => "Hero Potion",
    TreasureUnique = 280 => "Treasure",
    TimidityPoison = 281 => "Timidity Poison",
    TimidityPotion = 282 => "Timidity Potion",
    DamageTargetPoison = 283 => "DT Poison",
    GroupRepair = 284 => "Group Repair",
    TreasureAntiquityLead = 285 => "Treasure Antiquity Lead",
    SaturaliaFish = 286 => "Saturalia Fish",
    StylePage = 287 => "Style Page",

    CollectibleFragment = 289 => "Collectible Fragment",
    Grimoire = 290 => "Grimoire",
    FocusScript = 291 => "Focus Script",
    SignatureScript = 292 => "Signature Script",
    AffixScript = 293 => "Affix Script",
    Ink = 294 => "Ink",
    Lockpick = 295 => "Lockpick",
    Unidentified = 298 => "Unidentified",
    SkillScroll = 301 => "Skill Scroll",

    Engram = 314 => "Engram",
    HeavyHelmDuplicate = 326 => "Helm",
}