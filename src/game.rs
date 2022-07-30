use std::ffi::c_void;
use std::fmt::Display;
use std::os::raw::c_char;

use lazy_static::lazy_static;

#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}
//const RTLD_NEXT: *const c_void = -1isize as *const c_void;
const RTLD_DEFAULT: *const c_void = 0usize as *const c_void;
pub unsafe fn dlsym_current(symbol: &'static str) -> *const c_void {
    let ptr = dlsym(RTLD_DEFAULT, symbol.as_ptr() as *const c_char);
    if ptr.is_null() {
        panic!("Unable to find symbol {}", symbol);
    }
    ptr as *const c_void
}

pub type undefined = ::std::os::raw::c_uchar;
pub type byte = ::std::os::raw::c_uchar;
pub type dwfenc = ::std::os::raw::c_uchar;
pub type dword = ::std::os::raw::c_uint;
pub type longlong = ::std::os::raw::c_longlong;
pub type qword = ::std::os::raw::c_ulong;
pub type uchar = ::std::os::raw::c_uchar;
pub type uint = ::std::os::raw::c_uint;
pub type ulong = ::std::os::raw::c_ulong;
pub type ulonglong = ::std::os::raw::c_ulonglong;
pub type undefined1 = ::std::os::raw::c_uchar;
pub type undefined2 = ::std::os::raw::c_ushort;
pub type undefined4 = ::std::os::raw::c_uint;
pub type undefined5 = ::std::os::raw::c_ulong;
pub type undefined6 = ::std::os::raw::c_ulong;
pub type undefined7 = ::std::os::raw::c_ulong;
pub type undefined8 = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type word = ::std::os::raw::c_ushort;
pub type size_t = ulong;
pub type size_type = size_t;
pub type PPlayer = *mut Player;
pub type PActor = *mut Actor;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:7.0}, y: {:7.0}, z: {:7.0}", self.x, self.y, self.z)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rotation {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //pitch and roll seems to never change, at least for Player
        //it goes from -180 to 180
        write!(f, "{:3.0}", self.yaw + 180.0)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MakeshiftMap {
    pub map_data: [byte; 48usize],
}
impl std::fmt::Debug for MakeshiftMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MakeshiftMap")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MakeshiftSet {
    pub set_data: [byte; 48usize],
}
impl std::fmt::Debug for MakeshiftSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MakeshiftMap")
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Actor {
    pub super_IActor: *mut ::std::os::raw::c_void,
    pub m_refs: size_t,
    pub m_id: u32,
    pub field3_0x14: undefined,
    pub field4_0x15: undefined,
    pub field5_0x16: undefined,
    pub field6_0x17: undefined,
    pub m_target: *mut ::std::os::raw::c_void,
    pub m_timers: *mut ::std::os::raw::c_void,
    pub m_blueprintName: *mut ::std::os::raw::c_void,
    pub m_owner: *mut ::std::os::raw::c_void,
    pub m_health: i32,
    pub field12_0x3c: undefined,
    pub field13_0x3d: undefined,
    pub field14_0x3e: undefined,
    pub field15_0x3f: undefined,
    pub m_states: MakeshiftMap,
    pub m_forwardMovementFraction: f32,
    pub m_strafeMovementFraction: f32,
    pub m_remotePosition: Vector3,
    pub m_remoteVelocity: Vector3,
    pub m_remoteRotation: Rotation,
    pub m_remoteLocationBlendFactor: f32,
    pub m_spawner: *mut ::std::os::raw::c_void,
}

impl Actor {
    pub fn get_position(&self) -> Vector3 {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const Actor) -> Vector3 = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN5Actor11GetPositionEv\x00",
                ))
            };
        }
        unsafe { FUNC(self) }
    }
    pub fn get_velocity(&self) -> Vector3 {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const Actor) -> Vector3 = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN5Actor11GetVelocityEv\x00",
                ))
            };
        }
        unsafe { FUNC(self) }
    }
    pub fn get_rotation(&self) -> Rotation {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const Actor) -> Rotation = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN5Actor11GetRotationEv\x00",
                ))
            };
        }
        unsafe { FUNC(self) }
    }
    pub fn set_position(&self, pos: &Vector3) {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const Actor, *const Vector3) = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN5Actor11SetPositionERK7Vector3\x00",
                ))
            };
        }
        unsafe { FUNC(self, pos) }
    }
    pub fn set_velocity(&self, pos: &Vector3) {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const Actor, *const Vector3) = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN5Actor11SetVelocityERK7Vector3\x00",
                ))
            };
        }
        unsafe { FUNC(self, pos) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Player {
    pub super_Actor: Actor,
    pub super_IPlayer: *mut ::std::os::raw::c_void,
    pub m_characterId: u32,
    pub field3_0xb4: undefined,
    pub field4_0xb5: undefined,
    pub field5_0xb6: undefined,
    pub field6_0xb7: undefined,
    pub m_playerName: *mut ::std::os::raw::c_void,
    pub m_teamName: *mut ::std::os::raw::c_void,
    pub m_avatarIndex: u8,
    pub field10_0xc9: undefined,
    pub field11_0xca: undefined,
    pub field12_0xcb: undefined,
    pub m_colors: [u32; 4usize],
    pub field14_0xdc: undefined,
    pub field15_0xdd: undefined,
    pub field16_0xde: undefined,
    pub field17_0xdf: undefined,
    pub m_inventory: MakeshiftMap,
    pub m_pickups: MakeshiftSet,
    pub m_cooldowns: MakeshiftMap,
    pub m_circuitInputs: MakeshiftMap,
    pub m_circuitOutputs: MakeshiftMap,
    pub m_admin: bool,
    pub m_pvpEnabled: bool,
    pub m_pvpDesired: bool,
    pub field26_0x1d3: undefined,
    pub m_pvpChangeTimer: f32,
    pub m_pvpChangeReportedTimer: i32,
    pub m_changingServerRegion: bool,
    pub field30_0x1dd: undefined,
    pub field31_0x1de: undefined,
    pub field32_0x1df: undefined,
    pub m_currentRegion: *mut ::std::os::raw::c_void,
    pub m_changeRegionDestination: *mut ::std::os::raw::c_void,
    pub m_aiZones: MakeshiftSet,
    pub m_mana: i32,
    pub m_manaRegenTimer: f32,
    pub m_healthRegenCooldown: f32,
    pub m_healthRegenTimer: f32,
    pub m_countdown: i32,
    pub m_remoteLookPosition: Vector3,
    pub m_remoteLookRotation: Rotation,
    pub field43_0x24c: undefined,
    pub field44_0x24d: undefined,
    pub field45_0x24e: undefined,
    pub field46_0x24f: undefined,
    pub m_equipped: [*mut ::std::os::raw::c_void; 10usize],
    pub m_currentSlot: size_t,
    pub m_questStates: MakeshiftMap,
    pub m_currentQuest: *mut ::std::os::raw::c_void,
    pub m_walkingSpeed: f32,
    pub m_jumpSpeed: f32,
    pub m_jumpHoldTime: f32,
    pub field54_0x2ec: undefined,
    pub field55_0x2ed: undefined,
    pub field56_0x2ee: undefined,
    pub field57_0x2ef: undefined,
    pub m_currentNPC: *mut ::std::os::raw::c_void,
    pub m_currentNPCState: *mut ::std::os::raw::c_void,
    pub m_localPlayer: *mut ::std::os::raw::c_void,
    pub m_eventsToSend: *mut ::std::os::raw::c_void,
    pub m_itemsUpdated: bool,
    pub field63_0x311: undefined,
    pub field64_0x312: undefined,
    pub field65_0x313: undefined,
    pub m_itemSyncTimer: f32,
    pub m_chatMessageCounter: u32,
    pub m_chatFloodDecayTimer: f32,
    pub m_lastHitByItem: *mut u8,
    pub m_lastHitItemTimeLeft: f32,
    pub m_circuitStateCooldownTimer: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameServerConnection {
    pub super_ServerConnection: [u8; 256],
    pub m_sock: *const c_void,
    pub m_writeStream: [u8; 32],
    pub m_tickInProgress: bool,
    pub _align1: [u8; 7],
}

impl GameServerConnection {
    pub fn jump(&self, state: bool) {
        lazy_static! {
            static ref FUNC: unsafe extern "C" fn(*const GameServerConnection, bool) = unsafe {
                std::mem::transmute(dlsym_current(
                    "_ZN20GameServerConnection4JumpEb\x00",
                ))
            };
        }
        unsafe { FUNC(self, state) }
    }
}
