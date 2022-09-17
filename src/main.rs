use std::{cmp::Ordering, convert::TryInto, io, time::Instant};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    //INIT
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut game_state = GameState {
        num_sites: parse_input!(input_line, i32),
        construction_sites: Vec::new(),
        gold: 0,
        touched_site: -1,
        sites: Vec::new(),
        num_units: 0,
        units: Vec::new(),
        target: None,
    };

    for _ in 0..game_state.num_sites as usize {
        first_loop_init(&mut game_state);
    }

    // LOOP
    loop {
        // INIT TURN
        let stop_watch = Instant::now();
        init_loop(&mut game_state);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // First line: A valid queen action
        // Second line: A set of training instructions

        //eprintln!("{:#?}", game_state);

        // ACTION
        println!("{}", game_state.choose_what_to_build());
        println!("{}", game_state.choose_what_to_train());

        eprintln!("Loop time      : {} ms", stop_watch.elapsed().as_millis());
    }
}

fn init_loop(game_state: &mut GameState) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    game_state.gold = parse_input!(inputs[0], i32);
    game_state.touched_site = parse_input!(inputs[1], i32);
    // -1 if none
    game_state.construction_sites = Vec::new();
    for _ in 0..game_state.num_sites as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let construction_sites = ConstructionSite {
            site_id: parse_input!(inputs[0], i32),
            ignore_1: parse_input!(inputs[1], i32), // used in future leagues
            ignore_2: parse_input!(inputs[2], i32), // used in future leagues
            structure_type: parse_input!(inputs[3], i32).try_into().unwrap(),
            owner: parse_input!(inputs[4], i32).try_into().unwrap(),
            param_1: parse_input!(inputs[5], i32),
            param_2: parse_input!(inputs[6], i32),
        };
        game_state.construction_sites.push(construction_sites)
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    game_state.num_units = parse_input!(input_line, i32);
    game_state.units = Vec::new();
    for _ in 0..game_state.num_units as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let unit = Unit {
            position: Position {
                x: parse_input!(inputs[1], i32),
                y: parse_input!(inputs[2], i32),
            },
            owner: parse_input!(inputs[2], i32).try_into().unwrap(),
            unit_type: parse_input!(inputs[3], i32).try_into().unwrap(), // -1 = QUEEN, 0 = KNIGHT, 1 = ARCHER, 2 = GIANT
            health: parse_input!(inputs[4], i32),
        };

        game_state.units.push(unit);
    }
}

fn first_loop_init(game_state: &mut GameState) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let site = Site {
        site_id: parse_input!(inputs[0], i32),
        position: Position {
            x: parse_input!(inputs[1], i32),
            y: parse_input!(inputs[2], i32),
        },
        radius: parse_input!(inputs[3], i32),
    };
    game_state.sites.push(site);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Site {
    site_id: i32,
    position: Position,
    radius: i32,
}

#[derive(Debug, Clone)]
struct ConstructionSite {
    site_id: i32,
    structure_type: StructureType,
    owner: Owner,
    ignore_1: i32,
    ignore_2: i32,
    param_1: i32,
    param_2: i32,
}

#[derive(Debug, Clone)]
struct GameState {
    num_sites: i32,
    gold: i32,
    touched_site: i32,
    sites: Vec<Site>,
    construction_sites: Vec<ConstructionSite>,
    num_units: i32,
    units: Vec<Unit>,
    target: Option<i32>,
}

#[derive(Debug, Clone)]
struct Unit {
    position: Position,
    owner: Owner,
    unit_type: UnitType,
    health: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum UnitType {
    Queen = -1,
    Knight = 0,
    Bowman = 1,
    Giant = 2,
}

#[derive(Debug, Clone, PartialEq)]
enum StructureType {
    None = -1,
    Tower = 1,
    Barrack = 2,
}

#[derive(Debug, PartialEq, Clone)]
enum Owner {
    None = -1,
    Allied = 0,
    Ennemie = 1,
}

impl From<i32> for Owner {
    fn from(x: i32) -> Self {
        match x {
            -1 => Owner::None,
            0 => Owner::Allied,
            1 => Owner::Ennemie,
            _ => panic!("Unknown Owner"),
        }
    }
}

impl From<i32> for StructureType {
    fn from(x: i32) -> Self {
        match x {
            -1 => StructureType::None,
            1 => StructureType::Tower,
            2 => StructureType::Barrack,
            _ => panic!("Unknown StructureType"),
        }
    }
}

impl From<i32> for UnitType {
    fn from(x: i32) -> Self {
        match x {
            -1 => UnitType::Queen,
            0 => UnitType::Knight,
            1 => UnitType::Bowman,
            2 => UnitType::Giant,
            _ => panic!("Unknown UnitType"),
        }
    }
}

impl Position {
    fn get_distance_from(&self, b: &Position) -> f64 {
        f64::from((b.x - self.x).pow(2) + (b.y - self.y).pow(2)).sqrt()
    }
}

impl GameState {
    fn get_queen(&self) -> &Unit {
        let queen = self
            .units
            .iter()
            .find(|e| e.unit_type == UnitType::Queen && e.owner == Owner::Allied)
            .unwrap();
        queen
    }

    fn get_site_id(&self, id: i32) -> Option<&Site> {
        self.sites.iter().find(|s| s.site_id == id)
    }

    fn get_site_id_by_position(&self, pos: Position) -> Option<&Site> {
        self.sites.iter().find(|s| s.position == pos)
    }

    fn get_sites_id_without_construction(&self) -> Vec<i32> {
        self.construction_sites
            .iter()
            .filter(|c| c.owner == Owner::None)
            .map(|c| c.site_id)
            .collect()
    }

    fn get_sites_pos_without_construction(&self) -> Vec<Position> {
        self.get_sites_id_without_construction()
            .iter()
            .filter(|i| self.get_site_id(**i) != None)
            .map(|i| self.get_site_id(*i).expect("impossible").position)
            .collect()
    }

    fn get_barrack_bowman_id(&self) -> Vec<i32> {
        self.construction_sites
            .iter()
            .filter(|c| {
                c.structure_type == StructureType::Barrack
                    && c.owner == Owner::Allied
                    && c.param_2 == 1
            })
            .map(|c| c.site_id)
            .collect()
    }

    fn get_barrack_knight_id(&self) -> Vec<i32> {
        self.construction_sites
            .iter()
            .filter(|c| {
                c.structure_type == StructureType::Barrack
                    && c.owner == Owner::Allied
                    && c.param_2 == 0
            })
            .map(|c| c.site_id)
            .collect()
    }

    fn get_barrack_id(&self) -> Vec<i32> {
        self.construction_sites
            .iter()
            .filter(|c| c.structure_type == StructureType::Barrack && c.owner == Owner::Allied)
            .map(|c| c.site_id)
            .collect()
    }

    fn get_tower_id(&self) -> Vec<i32> {
        self.construction_sites
            .iter()
            .filter(|c| c.structure_type == StructureType::Tower && c.owner == Owner::Allied)
            .map(|c| c.site_id)
            .collect()
    }

    fn choose_what_to_build(&mut self) -> String {
        let mut building = "TOWER";

        if self.get_barrack_bowman_id().len() < 1 {
            building = "BARRACKS-ARCHER";
        } else if self.get_barrack_knight_id().len() < 1 {
            building = "BARRACKS-KNIGHT";
        } else if self.get_tower_id().len() < 1 {
            building = "TOWER";
        } else if self.get_barrack_bowman_id().len() < self.get_barrack_knight_id().len()
            && self.get_barrack_id().len() < 4
        {
            building = "BARRACKS-ARCHER";
        } else if self.get_barrack_bowman_id().len() >= self.get_barrack_knight_id().len()
            && self.get_barrack_id().len() < 4
        {
            building = "BARRACKS-KNIGHT";
        }

        format!("BUILD {} {}", self.update_target(), building)
    }

    fn choose_what_to_train(&self) -> String {
        let ids = self.get_barrack_id();

        if ids.len() == 0 {
            return String::from("TRAIN");
        }

        let mut to_train = String::from("");
        for id in ids {
            to_train = format!("{} {}", to_train, id);
        }
        format!("TRAIN {}", to_train.trim_end().trim_start())
    }

    fn update_target(&mut self) -> i32 {
        let target_is_null_or_target_builded = self.target == None
            || !self
                .get_sites_id_without_construction()
                .contains(&self.target.unwrap());
        if target_is_null_or_target_builded {
            let pos = self.get_sites_pos_without_construction();

            let target = find_nearest(self.get_queen().position, pos);
            self.target = match target {
                Some(pos) => match self.get_site_id_by_position(pos) {
                    Some(site) => Some(site.site_id),
                    _ => Some(0),
                },
                _ => Some(0),
            }
        }

        self.target.unwrap()
    }
}

#[test]
fn test_get_distance_from() {
    let origin = Position { x: 0, y: 0 };
    let x10 = Position { x: 10, y: 0 };
    assert_eq!(10.0, origin.get_distance_from(&x10));
    assert_eq!(10.0, x10.get_distance_from(&origin));

    let y10 = Position { x: 0, y: 10 };
    assert_eq!(10.0, origin.get_distance_from(&y10));
    assert_eq!(10.0, y10.get_distance_from(&origin));

    let x10y10 = Position { x: 10, y: 10 };
    assert_eq!(14.142135623730951, origin.get_distance_from(&x10y10));
    assert_eq!(14.142135623730951, x10y10.get_distance_from(&origin));
}

#[test]
fn test_find_nearest() {
    let queen_pos = Position { x: 50, y: 50 };
    let nearest = Position { x: 55, y: 55 };

    let sites_pos = vec![nearest];
    let result = find_nearest(queen_pos, sites_pos);
    assert_eq!(nearest, result.unwrap());

    let sites_pos = vec![nearest, Position { x: 1000, y: 1000 }];
    let result = find_nearest(queen_pos, sites_pos);
    assert_eq!(nearest, result.unwrap());

    let sites_pos = vec![
        Position { x: 1000, y: 1000 },
        nearest,
        Position { x: 1000, y: 1000 },
    ];
    let result = find_nearest(queen_pos, sites_pos);
    assert_eq!(nearest, result.unwrap());

    let sites_pos = vec![
        Position { x: 54, y: 1000 },
        nearest,
        Position { x: 1000, y: 54 },
    ];
    let result = find_nearest(queen_pos, sites_pos);
    assert_eq!(nearest, result.unwrap());

    let sites_pos = vec![Position { x: 0, y: 0 }, nearest, Position { x: 50, y: 40 }];
    let result = find_nearest(queen_pos, sites_pos);
    assert_eq!(nearest, result.unwrap());
}

fn find_nearest(from: Position, mut others: Vec<Position>) -> Option<Position> {
    if others.len() == 0 {
        return None;
    }

    others.sort_by(|a, b| {
        let a = a.get_distance_from(&from);
        let b = b.get_distance_from(&from);

        if a > b {
            Ordering::Greater
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    match others.first() {
        Some(s) => Some(s.to_owned()),
        _ => None,
    }
}
