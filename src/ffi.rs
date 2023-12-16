#[allow(unused)]
pub(crate) mod internal {
    use crate::binary_layout::prelude::*;

    define_layout!(snapshot, LittleEndian, {
        frame: u32,
        resolution: [u8; 4],
        input: snapshot_input::NestedView,
        data: [u8], // open ended byte array, matches until the end of the packet
    });

    define_layout!(snapshot_input, LittleEndian, {
        gamepad_p1: [u8; 10],
        gamepad_p2: [u8; 10],
        gamepad_p3: [u8; 10],
        gamepad_p4: [u8; 10],
    });

    // Allocate a big buffer for reading/writing snapshot data
    pub static mut SNAPSHOT_DATA: [u8; 10240] = [0; 10240];
    pub static mut SNAPSHOT_DATA_SIZE: usize = 0;

    pub fn write_snapshot(data: &[u8]) {
        unsafe {
            let size = data.len();
            assert!(
                size < SNAPSHOT_DATA.len(),
                "Snapshot is too large (max size 10240 bytes)"
            );
            SNAPSHOT_DATA[0..size].copy_from_slice(data);
            SNAPSHOT_DATA_SIZE = size;
        }
        std::println!("Frame {:?}", read_snapshot_frame());
        let res = read_snapshot_resolution();
        let x = res & 0xffff;
        let y = res >> 16;
        std::println!("Resolution {:?}", [x, y]);
    }

    pub fn read_snapshot() -> &'static [u8] {
        unsafe { &SNAPSHOT_DATA[..SNAPSHOT_DATA_SIZE] }
    }

    pub fn read_snapshot_mut() -> &'static mut [u8] {
        unsafe { &mut SNAPSHOT_DATA[..SNAPSHOT_DATA_SIZE] }
    }

    pub fn read_snapshot_frame() -> u32 {
        unsafe {
            let snapshot = read_snapshot();
            let view = snapshot::View::new(snapshot);
            view.frame().read()
        }
    }

    pub fn read_snapshot_resolution() -> u32 {
        unsafe {
            let snapshot = read_snapshot();
            let view = snapshot::View::new(snapshot);
            let res = view.resolution();
            u32::from_le_bytes(*view.resolution())
        }
    }

    pub fn read_snapshot_gamepad(i: usize) -> [u8; 10] {
        unsafe {
            let snapshot = read_snapshot();
            let view = snapshot::View::new(snapshot);
            match i {
                0 => *view.input().gamepad_p1(),
                1 => *view.input().gamepad_p2(),
                2 => *view.input().gamepad_p3(),
                3 => *view.input().gamepad_p4(),
                n => unreachable!("Snapshot gamepad out-of-range"),
            }
        }
    }

    pub fn write_snapshot_state(data: &[u8]) -> usize {
        unsafe {
            let snapshot = read_snapshot_mut();
            let mut view = snapshot::View::new(snapshot);
            view.data_mut()[..data.len()].copy_from_slice(data);
            SNAPSHOT_DATA.len() - SNAPSHOT_DATA_SIZE
        }
    }

    pub fn read_snapshot_state() -> Vec<u8> {
        unsafe {
            let snapshot = read_snapshot();
            let view = snapshot::View::new(snapshot);
            // On first frame, no snapshot state should exist
            if view.frame().read() == 0 {
                return vec![];
            }
            view.data().to_vec()
        }
    }
}

#[allow(unused)]
pub mod sys {
    #[cfg(not(target_family = "wasm"))]
    pub fn rand() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn rand() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn rand() -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn rand() -> u32;
            }
            rand()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn log(ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn log(ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn log(ptr: *const u8, len: u32) {
        // #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn log(ptr: *const u8, len: u32);
            }
            log(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn resolution() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn resolution() -> u32 {
        super::internal::read_snapshot_resolution()
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn resolution() -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn resolution() -> u32;
            }
            resolution()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        -1
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        let mut state = super::internal::read_snapshot_state();
        unsafe { std::ptr::copy(ptr, state.as_mut_ptr(), len as usize) };
        super::internal::write_snapshot_state(&state) as i32
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn save(ptr: *const u8, len: u32) -> i32;
            }
            save(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        return -1;
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        let state = super::internal::read_snapshot_state();
        unsafe {
            std::ptr::copy(state.as_ptr(), ptr, state.len());
            *len = state.len() as u32;
        };
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn load(ptr: *mut u8, len: *mut u32) -> i32;
            }
            load(ptr, len)
        }
    }
}

#[allow(unused)]
pub mod input {
    #[cfg(not(target_family = "wasm"))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {
        let gamepad = super::internal::read_snapshot_gamepad(player as usize);
        unsafe { std::ptr::copy(gamepad.as_ptr(), out_ptr, gamepad.len()) };
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/input")]
            extern "C" {
                fn gamepad(player: u32, out_ptr: *mut u8);
            }
            return gamepad(player, out_ptr);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/input")]
            extern "C" {
                fn mouse(player: u32, out_ptr: *mut u8);
            }
            mouse(player, out_ptr)
        }
    }
}

#[allow(unused)]
pub mod canvas {
    #[cfg(not(target_family = "wasm"))]
    pub fn clear(fill: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn clear(fill: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn clear(fill: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn clear(fill: u32);
            }
            clear(fill)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_camera(x: i32, y: i32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn set_camera(x: i32, y: i32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn set_camera(x: i32, y: i32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn set_camera(x: i32, y: i32);
            }
            set_camera(x, y)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_camera() -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_camera() -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_camera() -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_camera() -> i32;
            }
            get_camera()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn quad(
        xy: i32,
        wh: u32,
        fill: u32,
        rotation_deg: i32,
        rotation_origin: i32,
        border_radius: u32,
        border_size: u32,
        border_color: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn quad(
        xy: i32,
        wh: u32,
        fill: u32,
        rotation_deg: i32,
        rotation_origin: i32, // xy
        border_radius: u32, // wh
        border_size: u32,
        border_color: u32, // trbl
    ) {
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn quad(
        xy: i32,
        wh: u32,
        fill: u32,
        rotation_deg: i32,
        rotation_origin: i32,
        border_radius: u32,
        border_size: u32,
        border_color: u32,
    ) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn quad(
                    xy: i32,
                    wh: u32,
                    fill: u32,
                    rotation_deg: i32,
                    rotation_origin: i32,
                    border_radius: u32,
                    border_size: u32,
                    border_color: u32,
                );
            }
            quad(
                xy,
                wh,
                fill,
                rotation_deg,
                rotation_origin,
                border_radius,
                border_size,
                border_color,
            )
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn circfill(x: i32, y: i32, d: u32, fill: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn circfill(x: i32, y: i32, d: u32, fill: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn circfill(x: i32, y: i32, d: u32, fill: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn circfill(x: i32, y: i32, d: u32, fill: u32);
            }
            circfill(x, y, d, fill)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn rectfill(x: i32, y: i32, w: u32, h: u32, fill: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn rectfill(x: i32, y: i32, w: u32, h: u32, fill: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn rectfill(x: i32, y: i32, w: u32, h: u32, fill: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn rectfill(x: i32, y: i32, w: u32, h: u32, fill: u32);
            }
            rectfill(x, y, w, h, fill)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn subsprite(x: i32, y: i32, w: u32, h: u32, sx: u32, sy: u32, sw: u32, sh: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn subsprite(x: i32, y: i32, w: u32, h: u32, sx: u32, sy: u32, sw: u32, sh: u32) {}
    #[rustfmt::skip]
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn subsprite(x: i32, y: i32, w: u32, h: u32, sx: u32, sy: u32, sw: u32, sh: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn subsprite(x: i32, y: i32, w: u32, h: u32, sx: u32, sy: u32, sw: u32, sh: u32);
            }
            subsprite(x, y, w, h, sx, sy, sw, sh)
        }
    }

    // #[cfg(not(target_family = "wasm"))]
    // pub fn sprite_by_key(ptr: *const u8, len: u32, x: i32, y: i32, fps: u32) {}
    // #[cfg(all(target_family = "wasm", feature = "no-host"))]
    // pub fn sprite_by_key(ptr: *const u8, len: u32, x: i32, y: i32, fps: u32) {}
    // #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn sprite_by_key(ptr: *const u8, len: u32, x: i32, y: i32, fps: u32, deg: i32) {
        #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn sprite_by_key(ptr: *const u8, len: u32, x: i32, y: i32, fps: u32, deg: i32);
            }
            return sprite_by_key(ptr, len, x, y, fps, deg);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn text(x: i32, y: i32, font: u8, color: u32, ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn text(x: i32, y: i32, font: u8, color: u32, ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn text(x: i32, y: i32, font: u8, color: u32, ptr: *const u8, len: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn text(x: i32, y: i32, font: u8, color: u32, ptr: *const u8, len: u32);
            }
            text(x, y, font, color, ptr, len)
        }
    }
}
