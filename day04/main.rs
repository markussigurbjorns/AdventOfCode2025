#![no_std]
#![no_main]

use core::ffi::*;
use core::ptr::*;
use core::ptr;
use core::mem::*;
use core::panic::PanicInfo;
use libc::*;
use da::*;


#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    abort();
}

pub unsafe fn read_file(path: *const c_char) -> *mut c_char {
    
    let file = fopen(path, c"rb".as_ptr());
    
    if file.is_null() {
        printf(c"failed to open file\n".as_ptr());
        return null_mut();
    }
    fseek(file, 0, 2);
    let size = ftell(file);
    fseek(file, 0, 0);
   
    let size_usize = (size.saturating_add(1)) as usize;

    let buf = malloc(size_usize) as *mut c_char;
    if buf.is_null() {
        printf(c"malloc failed\n".as_ptr());
        fclose(file);
        return null_mut();
    }
    
    fread(buf as *mut c_void, 1, size as usize, file);
    
    ptr::write(buf.add(size as usize), 0);
    
    fclose(file);
    
    buf
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct FloorPlan {
    pub length: i32,
    pub height: i32,
    pub slots: Array<i8>,
}

pub unsafe fn new_floor_plan() -> FloorPlan {
    FloorPlan {
        length:0,
        height:0,
        slots: zeroed()
    }
}

pub unsafe fn idx(r:i32, c:i32, h:i32) -> i32 {
    r*h+c
}

pub unsafe fn process_part1(buf: *const c_char) -> i32 {
    let mut p = buf;
    let mut rolls: i32 = 0;
    let mut fp: FloorPlan = new_floor_plan();
    let mut l: i32 = 0;
    let mut h: i32 = 0;
    while *p != 0 {
        if *p == b'\n' as i8 {
            fp.length = l;
            h += 1;
        } else {
            da_append(&mut fp.slots, *p);
        }
        if fp.length < 1 {
            l +=1;
        }
        p = p.add(1);
    }
    fp.height = h;

    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..fp.height {
        for c in 0..fp.length {
            let a = idx(r,c,fp.height);
            let slot_a = fp.slots.items.add(a as usize);
            if *slot_a != b'@' as i8 {continue}
            let mut count = 0;
            let mut pick_up = true;
            for (dr,dc) in deltas {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= fp.height || nc < 0 || nc >= fp.length {
                    continue;
                }
                let b = idx(nr, nc, fp.height);
                let slot_b = fp.slots.items.add(b as usize);

                if *slot_a == *slot_b {
                    count +=1;
                    if count == 4 {
                        pick_up = false;
                        break;
                    }
                }
                
            }
            if pick_up {rolls+=1;}
            
        }
    }
    rolls
}


pub unsafe fn process_part2(buf: *const c_char) -> i32 {
    let mut p = buf;
    let mut rolls: i32 = 0;
    let mut fp: FloorPlan = new_floor_plan();
    let mut l: i32 = 0;
    let mut h: i32 = 0;
    while *p != 0 {
        if *p == b'\n' as i8 {
            fp.length = l;
            h += 1;
        } else {
            da_append(&mut fp.slots, *p);
        }
        if fp.length < 1 {
            l +=1;
        }
        p = p.add(1);
    }
    fp.height = h;

    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    loop {
        let mut to_be_removed:Array<usize> = zeroed();
        for r in 0..fp.height {
            for c in 0..fp.length {
                let a = idx(r,c,fp.height);
                let slot_a = fp.slots.items.add(a as usize);
                if *slot_a != b'@' as i8 {continue}
                let mut count = 0;
                let mut pick_up = true;
                for (dr,dc) in deltas {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr < 0 || nr >= fp.height || nc < 0 || nc >= fp.length {
                        continue;
                    }
                    let b = idx(nr, nc, fp.height);
                    let slot_b = fp.slots.items.add(b as usize);

                    if *slot_a == *slot_b {
                        count +=1;
                        if count == 4 {
                            pick_up = false;
                            break;
                        }
                    }
                }
                if pick_up {
                    da_append(&mut to_be_removed, a as usize);
                    rolls += 1;
                }
            }
        }
        if to_be_removed.count == 0 {break;}
        else {
            while to_be_removed.count != 0 {
                let rem = da_pop(&mut to_be_removed).unwrap();
                da_replace(&mut fp.slots, rem, '.' as i8);
            }
        }
    }
    rolls

}

#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
    
    if argc == 1 {
        printf(c"please provide input\n".as_ptr());
        return 0;
    }
    let arg1 = ptr::read(argv.add(1));
    let text = read_file(arg1);

    if !text.is_null() {
        let res1 = process_part1(text);
        printf(c"%d\n".as_ptr(), res1);
        let res2 = process_part2(text);
        printf(c"%d\n".as_ptr(), res2);

    }
    0
}

pub mod libc {
    use core::ffi::*;

    #[repr(C)]
    pub struct FILE {
        _private: [u8; 0],
    }


    extern "C" {
        pub fn abort() -> !;
        pub fn printf(format: *const c_char, ...) -> c_int;
        pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        pub fn fclose(file: *mut FILE) -> c_int;
        pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
        pub fn fread(ptr: *mut c_void, size: usize, nobj: usize, stream: *mut FILE) -> usize;
        pub fn ftell(stream: *mut FILE) -> c_long;
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn free(p: *mut c_void);
    }

    pub unsafe fn realloc_items<T>(ptr: *mut T, count: usize) -> *mut T {
        extern "C" {
            #[link_name = "realloc"]
            fn realloc_raw(ptr: *mut c_void, size: usize) -> *mut c_void;
        }
        realloc_raw(ptr as *mut c_void, size_of::<T>()*count) as *mut T
    }
}


pub mod da {
    use crate::libc;
    use core::ptr;
    use core::ffi::*;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Array<T> {
        pub items: *mut T,
        pub count: usize,
        pub capacity: usize,
    }

    pub unsafe fn da_append<T>(da: *mut Array<T>, item: T) {
        let da_ref = &mut *da;

        if da_ref.count >= da_ref.capacity {
            if da_ref.capacity == 0 {
                da_ref.capacity = 256;
            } else {
                da_ref.capacity *= 2;
            }
            da_ref.items = libc::realloc_items(da_ref.items, da_ref.capacity);
        }

        *da_ref.items.add(da_ref.count) = item;
        da_ref.count += 1;
    }

    pub unsafe fn da_pop<T>(da: *mut Array<T>) -> Option<T> {
        let da_ref = &mut *da;

        if da_ref.count == 0 {
            return None;
        }

        da_ref.count -= 1;
        let ptr = da_ref.items.add(da_ref.count);

        Some(ptr::read(ptr))
    }

    pub unsafe fn da_remove<T>(da: *mut Array<T>, index: usize) -> Option<T> {
        let da_ref = &mut *da;

        if index >= da_ref.count {
            return None;
        }

        let ptr = da_ref.items.add(index);
        let removed = ptr::read(ptr);

        let num_to_move = da_ref.count - index - 1;
        if num_to_move > 0 {
            ptr::copy(ptr.add(1), ptr, num_to_move);
        }

        da_ref.count -= 1;

        Some(removed)
    }

    pub unsafe fn da_replace<T>(da: *mut Array<T>, index: usize, item: T) -> Option<T> {
        let da_ref = &mut *da;

        if index >= da_ref.count {
            return None;
        }

        let ptr = da_ref.items.add(index);

        let old = ptr::read(ptr);
        ptr::write(ptr, item);

        Some(old)
    }

    pub unsafe fn da_destroy<T>(da: *mut Array<T>) {
        let da_ref = &mut *da;

        for i in 0..da_ref.count {
            ptr::drop_in_place(da_ref.items.add(i));
        }

        libc::free(da_ref.items as *mut c_void);
        da_ref.items = ptr::null_mut();
        da_ref.count = 0;
        da_ref.capacity = 0;
    }
}
