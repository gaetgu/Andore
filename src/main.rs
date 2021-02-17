use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

/*
    I feel it my civic duty to inform you that you do NOT want
    to dig deeper into this code. It was slapped together really
    quickly, and yes, this is the only file (??? lines, !!!)

    I am currently working to split it up into different files,
    so if you are here to contribute, consider leaving a star, 
    watch the repo, and wait for the next commit!
*/

////////////////////////////////////////////////////////////////
/// Constants (Think of them as settings, seems more refined) //
////////////////////////////////////////////////////////////////
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

const LIMIT_FPS: i32 = 20;


////////////////////////////////////////////////////////////////
/// The renderer ///////////////////////////////////////////////
////////////////////////////////////////////////////////////////
struct Tcod {
    root: Root,
    con: Offscreen,
}


////////////////////////////////////////////////////////////////
/// An object. Can be an npc, player, etc. /////////////////////
////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // Move the object
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    // Set color and draw object
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}


////////////////////////////////////////////////////////////////
/// A tile. Think walls and floors. ////////////////////////////
////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}


////////////////////////////////////////////////////////////////
/// Create and render the map (a 2d array of tiles) ////////////
////////////////////////////////////////////////////////////////
type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

fn make_map() -> Map {
    // Fill the map with unblocked tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

// Just a quick test


////////////////////////////////////////////////////////////////
/// Handle them keys! //////////////////////////////////////////
////////////////////////////////////////////////////////////////
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Control
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,

        // Movement
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        // Default
        _ => {}
    }

    false
}


////////////////////////////////////////////////////////////////
/// The main function. This really contains way too much. //////
////////////////////////////////////////////////////////////////
fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Andore")
        .init();
    
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    let mut objects = [player, npc];

    let game = Game {
        map: make_map(),
    };

    while !tcod.root.window_closed() {
        tcod.con.clear();

        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();
        tcod.root.wait_for_keypress(true);

        // Check for and handle keys
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}