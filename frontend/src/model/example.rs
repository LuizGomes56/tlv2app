use std::rc::Rc;

use yew::UseStateHandle;

use super::realtime::Realtime;

pub fn makeup_example(game_data: UseStateHandle<Option<Rc<Realtime>>>) {
    let json_str = r#"{
        "current_player": {
            "damaging_abilities": {
                "C": "Critical Strike",
                "Q2": "Blooming Burst",
                "Q_MAX": "Blooming Burst",
                "R": "Pop Blossom",
                "E": "Tangle-Barbs",
                "A": "Basic Attack",
                "Q1": "Blooming Burst",
                "W": "Shapesplitter"
            },
            "damaging_items": {},
            "damaging_runes": {
                "8229": "Comet"
            },
            "riot_id": "Giga Gnar#BRZL",
            "level": 18,
            "team": "ORDER",
            "position": "",
            "champion_name": "Neeko",
            "champion_id": "Neeko",
            "base_stats": {
                "armor": 109.4,
                "health": 2378.0,
                "attack_damage": 90.5,
                "magic_resist": 52.1,
                "mana": 960.0
            },
            "bonus_stats": {
                "armor": 5.47000274658204,
                "health": 830.000244140625,
                "attack_damage": 33.61499786376952,
                "magic_resist": 2.6049980163574205,
                "mana": 25.00006103515625
            },
            "current_stats": {
                "ability_power": 659.1480102539062,
                "armor": 114.87000274658205,
                "armor_penetration_flat": 0.0,
                "armor_penetration_percent": 1.0,
                "attack_damage": 124.11499786376952,
                "attack_range": 550.0,
                "attack_speed": 1.5931501388549805,
                "crit_chance": 0.0,
                "crit_damage": 175.0,
                "current_health": 3208.000244140625,
                "magic_penetration_flat": 15.0,
                "magic_penetration_percent": 1.0,
                "magic_resist": 54.70499801635742,
                "max_health": 3208.000244140625,
                "max_mana": 985.0000610351562,
                "current_mana": 985.0000610351562
            }
        },
        "enemies": [
            {
                "champion_id": "Chogath",
                "champion_name": "Cho'Gath",
                "riot_id": "Chogath#BOT",
                "team": "CHAOS",
                "level": 3,
                "position": "MIDDLE",
                "damages": {
                    "abilities": {
                        "Q1": {
                            "minimum_damage": 491.2144514122856,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "W": {
                            "minimum_damage": 479.4791082201179,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "E": {
                            "minimum_damage": 531.9332271030017,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "A": {
                            "minimum_damage": 85.3757508951123,
                            "maximum_damage": 0.0,
                            "damage_type": "PHYSICAL_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "Q2": {
                            "minimum_damage": 249.77306788321192,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "R": {
                            "minimum_damage": 1117.2602191688625,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "Q_MAX": {
                            "minimum_damage": 0.0,
                            "maximum_damage": 990.7605871787094,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "C": {
                            "minimum_damage": 149.40756406644653,
                            "maximum_damage": 0.0,
                            "damage_type": "PHYSICAL_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        }
                    },
                    "items": {},
                    "runes": {
                        "8229": {
                            "minimum_damage": 138.57165794192588,
                            "maximum_damage": 0.0,
                            "damage_type": "ADAPTATIVE_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        }
                    },
                    "compared_items": {}
                },
                "base_stats": {
                    "armor": 45.375,
                    "health": 782.65,
                    "attack_damage": 75.195,
                    "magic_resist": 35.02375,
                    "mana": 358.5
                },
                "bonus_stats": {
                    "armor": 0.0,
                    "health": 0.0,
                    "attack_damage": 0.0,
                    "magic_resist": 0.0,
                    "mana": 0.0
                },
                "current_stats": {
                    "armor": 45.375,
                    "health": 782.65,
                    "attack_damage": 75.195,
                    "magic_resist": 35.02375,
                    "mana": 358.5
                }
            }
        ],
        "game_information": {
            "game_time": 366.6326293945313,
            "map_number": 11
        },
        "recommended_items": [
            4645,
            3165,
            3137,
            6653,
            3102,
            6655
        ],
        "compared_items": {},
        "scoreboard": [
            {
                "assists": 0,
                "creep_score": 0,
                "deaths": 0,
                "kills": 1,
                "riot_id": "Giga Gnar#BRZL",
                "champion_id": "Neeko",
                "champion_name": "Neeko",
                "team": "ORDER",
                "position": ""
            },
            {
                "assists": 0,
                "creep_score": 10,
                "deaths": 0,
                "kills": 0,
                "riot_id": "Chogath#BOT",
                "champion_id": "Chogath",
                "champion_name": "Cho'Gath",
                "team": "CHAOS",
                "position": "MIDDLE"
            }
        ]
    }"#;

    let realtime: Realtime =
        serde_json::from_str(json_str).expect("Falha ao deserializar Realtime");
    game_data.set(Some(Rc::new(realtime)));
}
