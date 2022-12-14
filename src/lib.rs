mod game;
use game::*;

use lazy_static::lazy_static;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};

unsafe fn game_api() -> *const c_void {
    lazy_static! {
        static ref GAME_API: usize =
            unsafe { std::mem::transmute(dlsym_current("Game\x00")) };
    }
    *GAME_API as *const c_void
}
//unsafe fn world() -> *const c_void {
//    lazy_static! {
//        static ref WORLD: usize =
//            unsafe { std::mem::transmute(dlsym_current("GameWorld\x00")) };
//    }
//    *WORLD as *const c_void
//}

unsafe fn get_item_by_name(name: &str) -> *const c_void {
    assert_eq!(name.as_bytes().last(), Some(&b'\x00'));
    lazy_static! {
        static ref FUNC: unsafe extern "C" fn(*const c_void, *const c_char) -> *const c_void = unsafe {
            std::mem::transmute(dlsym_current(
                "_ZN7GameAPI13GetItemByNameEPKc\x00",
            ))
        };
    }
    FUNC(game_api(), name.as_ptr() as *const i8)
}

unsafe fn player_add_item(
    player: *const c_void,
    item: *const c_void,
    count: u32,
    allow_partial: bool,
) -> bool {
    lazy_static! {
        static ref FUNC: unsafe extern "C" fn(*const c_void, *const c_void, u32, bool) -> bool = unsafe {
            std::mem::transmute(dlsym_current(
                "_ZN6Player7AddItemEP5IItemjb\x00",
            ))
        };
    }
    FUNC(player, item, count, allow_partial)
}
unsafe fn player_fast_travel(player: *const Player, from: &str, to: &str) {
    assert_eq!(from.as_bytes().last(), Some(&b'\x00'));
    assert_eq!(to.as_bytes().last(), Some(&b'\x00'));
    lazy_static! {
        static ref FUNC: unsafe extern "C" fn(*const Player, *const u8, *const u8) = unsafe {
            std::mem::transmute(dlsym_current(
                "_ZN6Player10FastTravelEPKcS1_\x00",
            ))
        };
    }
    FUNC(player, from.as_ptr(), to.as_ptr())
}

redhook::hook! {
    unsafe fn _ZN6Player12GetJumpSpeedEv(_x: *const c_void) -> f32 => jump_high {
        1000.0
    }
}

redhook::hook! {
    unsafe fn _ZN6Player15GetWalkingSpeedEv(_x: *const c_void) -> f32 => walk_fast {
        1000.0
    }
}

redhook::hook! {
    unsafe fn _ZN6Player15GetJumpHoldTimeEv(_x: *const c_void) -> f32 => jump_hold {
        1000.0
    }
}

redhook::hook! {
    unsafe fn _ZN6Player7CanJumpEv(_x: *const c_void) -> bool => can_jump {
        true
    }
}
redhook::hook! {
    //make the last egg visible, but not usable
    unsafe fn _ZN14BallmerPeakEgg6CanUseEP7IPlayer(x: *const c_void,y: *const c_void) -> bool => last_egg {
       redhook::real!(_ZN14BallmerPeakEgg6CanUseEP7IPlayer)(x, y);
       true
    }
}

redhook::hook! {
    unsafe fn _ZN11RubicksCube12CanStealItemEP6PlayerP5IItem(
        _x: *const c_void,
        _y: *const c_void,
        _z: *const c_void
    ) -> bool => can_steal {
        true
    }
}

static mut PRINT_POS: AtomicBool = AtomicBool::new(false);
static mut PRINT_VEL: AtomicBool = AtomicBool::new(false);
static mut FLOAT: AtomicBool = AtomicBool::new(false);
static mut FLOAT_HELPER: AtomicBool = AtomicBool::new(false);

//the server think we are falling if didn't sent a jump recently.
//so, if float is enabled, send a jump true with the position packet.
redhook::hook! {
    unsafe fn _ZN20GameServerConnection16MoveAndGetEventsEP6Player(
        game_server: *const GameServerConnection,
        player: *const Player
    ) => client_send_position {
        if FLOAT.load(Ordering::Relaxed) {
            let state = FLOAT_HELPER.get_mut();
            *state = !*state;
            (*game_server).jump(*state);
        }
        redhook::real!(_ZN20GameServerConnection16MoveAndGetEventsEP6Player)(
            game_server,
            player,
        )
    }
}

redhook::hook! {
    unsafe fn _ZN6Player4TickEf(player: *const Player, delta_time: f32) => player_tick {
        if PRINT_POS.load(Ordering::Relaxed) {
            println!(
                "Position {}, Rotation {}",
                 (*player).super_Actor.get_position(),
                 (*player).super_Actor.get_rotation(),
             );
        }
        if PRINT_VEL.load(Ordering::Relaxed) {
            println!(
                "Velocity {}",
                 (*player).super_Actor.get_velocity(),
             );
        }
        if FLOAT.load(Ordering::Relaxed) {
            let mut velocity = (*player).super_Actor.get_velocity();
            velocity.z = 3.0;
            (*player).super_Actor.set_velocity(&velocity);
        }
        redhook::real!(_ZN6Player4TickEf)(player, delta_time)
    }
}

redhook::hook! {
    unsafe fn _ZN11ClientWorld4ChatEP6PlayerRKSs(
        client_world: *const c_void,
        player: *const Player,
        text: *const *const i8
    ) => chat {
        let chat = CStr::from_ptr(*text).to_str().unwrap();
        if chat.starts_with("!hack ") {
            let mut command = chat.split(' ');
            let _hack = command.next().unwrap();
            match command.next() {
                Some("position") => {
                    let pos = PRINT_POS.get_mut();
                    *pos = !*pos;
                },
                Some("velocity") => {
                    let pos = PRINT_VEL.get_mut();
                    *pos = !*pos;
                },
                Some("tp") => {
                    let x: f32 = command.next().unwrap().parse().unwrap();
                    let y: f32 = command.next().unwrap().parse().unwrap();
                    let z: f32 = command.next().unwrap().parse().unwrap();
                    (*player).super_Actor.set_position(&Vector3{x, y, z});
                },
                Some("float") => {
                    let float = FLOAT.get_mut();
                    *float = !*float;
                },
                Some("ft") => {
                    let from = command.next().unwrap().to_owned() + "\x00";
                    let to = command.next().unwrap().to_owned() + "\x00";
                    player_fast_travel(player, &from, &to);
                },
                _ => {
                    panic!("Invalid Hack Command")
                }
            }
        } else {
            //not a command just forward to chat
            redhook::real!(_ZN11ClientWorld4ChatEP6PlayerRKSs)(
                client_world,
                player,
                text
            )
        }
    }
}
