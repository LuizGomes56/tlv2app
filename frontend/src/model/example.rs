use std::rc::Rc;

use yew::UseStateHandle;

use super::realtime::Realtime;

pub fn makeup_example(game_data: UseStateHandle<Option<Rc<Realtime>>>) {
    let json_str = r#"{
        "current_player": {
            "damaging_abilities": {
                "R": "Pop Blossom",
                "Q2": "Blooming Burst",
                "E": "Tangle-Barbs",
                "Q1": "Blooming Burst",
                "C": "Critical Strike",
                "A": "Basic Attack",
                "Q_MAX": "Blooming Burst",
                "W": "Shapesplitter"
            },
            "damaging_items": {
                "3115": "Nashor's Tooth",
                "3124": "Guinsoo's Rageblade",
                "3152": "Hextech Rocketbelt",
                "4633": "Riftmaker",
                "4646": "Stormsurge"
            },
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
                        "Q_MAX": {
                            "minimum_damage": 0.0,
                            "maximum_damage": 990.7605871787094,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "Q1": {
                            "minimum_damage": 491.2144514122856,
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
                        "W": {
                            "minimum_damage": 479.4791082201179,
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
                        "C": {
                            "minimum_damage": 149.40756406644653,
                            "maximum_damage": 0.0,
                            "damage_type": "PHYSICAL_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "Q_MONSTER_BONUS": {
                            "minimum_damage": 79.15100136431332,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
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
                        "A": {
                            "minimum_damage": 85.3757508951123,
                            "maximum_damage": 0.0,
                            "damage_type": "PHYSICAL_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        }
                    },
                    "items": {
                        "3115": {
                            "minimum_damage": 122.33378981308388,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": true
                        },
                        "3124": {
                            "minimum_damage": 24.99505306241473,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": true
                        },
                        "3152": {
                            "minimum_damage": 138.23497518232068,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "4633": {
                            "minimum_damage": 0.0,
                            "maximum_damage": 0.0,
                            "damage_type": "UNKNOWN",
                            "damages_in_area": false,
                            "damages_onhit": false
                        },
                        "4646": {
                            "minimum_damage": 169.85988317985894,
                            "maximum_damage": 0.0,
                            "damage_type": "MAGIC_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        }
                    },
                    "runes": {
                        "8229": {
                            "minimum_damage": 138.57165794192588,
                            "maximum_damage": 0.0,
                            "damage_type": "ADAPTATIVE_DAMAGE",
                            "damages_in_area": false,
                            "damages_onhit": false
                        }
                    },
                    "compared_items": {
                        "4645": {
                            "abilities": {
                                "total": 5276.244545979121,
                                "change": 1101.88956868706,
                                "damages": {
                                    "Q2": {
                                        "minimum_damage": 317.12409111308716,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 67.35102322987524,
                                        "max_dmg_change": 0.0
                                    },
                                    "E": {
                                        "minimum_damage": 690.2672898891825,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 158.33406278618077,
                                        "max_dmg_change": 0.0
                                    },
                                    "R": {
                                        "minimum_damage": 1428.8834103465435,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 311.62319117768106,
                                        "max_dmg_change": 0.0
                                    },
                                    "A": {
                                        "minimum_damage": 87.93702342196568,
                                        "maximum_damage": 0.0,
                                        "damage_type": "PHYSICAL_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 2.5612725268533723,
                                        "max_dmg_change": 0.0
                                    },
                                    "C": {
                                        "minimum_damage": 153.88979098843993,
                                        "maximum_damage": 0.0,
                                        "damage_type": "PHYSICAL_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 4.4822269219934014,
                                        "max_dmg_change": 0.0
                                    },
                                    "Q_MAX": {
                                        "minimum_damage": 0.0,
                                        "maximum_damage": 1258.9747086364023,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 0.0,
                                        "max_dmg_change": 268.21412145769284
                                    },
                                    "W": {
                                        "minimum_damage": 623.985974921781,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 144.5068667016631,
                                        "max_dmg_change": 0.0
                                    },
                                    "Q_MONSTER_BONUS": {
                                        "minimum_damage": 90.45573025149073,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 11.304728887177419,
                                        "max_dmg_change": 0.0
                                    },
                                    "Q1": {
                                        "minimum_damage": 624.726526410228,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 133.51207499794236,
                                        "max_dmg_change": 0.0
                                    }
                                }
                            },
                            "items": {
                                "total": 577.4885230747193,
                                "change": 122.06482183704111,
                                "damages": {
                                    "3115": {
                                        "minimum_damage": 165.14787380216828,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": true,
                                        "min_dmg_change": 42.814083989084395,
                                        "max_dmg_change": 0.0
                                    },
                                    "3124": {
                                        "minimum_damage": 28.564967447839177,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": true,
                                        "min_dmg_change": 3.569914385424447,
                                        "max_dmg_change": 0.0
                                    },
                                    "3152": {
                                        "minimum_damage": 170.64925319858827,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 32.41427801626759,
                                        "max_dmg_change": 0.0
                                    },
                                    "4633": {
                                        "minimum_damage": 0.0,
                                        "maximum_damage": 0.0,
                                        "damage_type": "UNKNOWN",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 0.0,
                                        "max_dmg_change": 0.0
                                    },
                                    "4646": {
                                        "minimum_damage": 213.1264286261236,
                                        "maximum_damage": 0.0,
                                        "damage_type": "MAGIC_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 43.26654544626467,
                                        "max_dmg_change": 0.0
                                    }
                                }
                            },
                            "runes": {
                                "total": 164.69857752599114,
                                "change": 26.126919584065263,
                                "damages": {
                                    "8229": {
                                        "minimum_damage": 164.69857752599114,
                                        "maximum_damage": 0.0,
                                        "damage_type": "ADAPTATIVE_DAMAGE",
                                        "damages_in_area": false,
                                        "damages_onhit": false,
                                        "min_dmg_change": 26.126919584065263,
                                        "max_dmg_change": 0.0
                                    }
                                }
                            }
                        }
                    }
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
                    "health": 110.0,
                    "attack_damage": 0.0,
                    "magic_resist": 0.0,
                    "mana": 0.0
                },
                "current_stats": {
                    "armor": 45.375,
                    "health": 892.65,
                    "attack_damage": 75.195,
                    "magic_resist": 35.02375,
                    "mana": 358.5
                },
                "real_resists": {
                    "magic_resist": 20.02375,
                    "armor": 45.375
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
        "compared_items": {
            "4645": {
                "name": "Shadowflame",
                "gold_cost": 3200,
                "prettified_stats": {
                    "Ability Power": 110.0,
                    "Magic Penetration": 15.0
                }
            }
        },
        "best_item": 4645,
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
        ],
        "enemy_dragon_multipliers": {
            "earth": 1.0,
            "fire": 1.0,
            "chemtech": 1.0
        },
        "ally_dragon_multipliers": {
            "earth": 1.05,
            "fire": 1.03,
            "chemtech": 1.0
        }
    }"#;

    let realtime: Realtime =
        serde_json::from_str(json_str).expect("Falha ao deserializar Realtime");
    game_data.set(Some(Rc::new(realtime)));
}
