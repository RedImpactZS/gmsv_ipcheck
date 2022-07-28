#![feature(c_unwind)]
#![feature(thread_local)]

use std::net::Ipv4Addr;
use std::str::FromStr;

use cidr_utils::cidr::Ipv4Cidr;
use cidr_utils::utils::Ipv4CidrCombiner;
use gmod::lua::State;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[macro_use] extern crate gmod;

lazy_static! {
static ref GLOBAL_COMBINER: Mutex<Option<Ipv4CidrCombiner>> = Mutex::new(Some(Ipv4CidrCombiner::new()));
}

#[lua_function]
unsafe fn load(lua: State) -> i32 {
    let contents = lua.check_string(1).into_owned();

    if let Some(combiner) = &mut *GLOBAL_COMBINER.lock().unwrap() {
        contents.lines()
            .filter_map(|x| Ipv4Cidr::from_str(x).ok())
            .for_each(|x| combiner.push(x));

        println!("ipcheck.load: Success, loaded {} CIDRs", combiner.len());
    } else {lua.error("ipcheck.load: Can't acquire combiner")}

    0
}

#[lua_function]
unsafe fn clear(lua: State) -> i32 {
    GLOBAL_COMBINER.lock().unwrap().replace(Ipv4CidrCombiner::new());
    0
}

#[lua_function]
unsafe fn contains(lua: State) -> i32 {
    let ip_string = lua.check_string(1);

    match Ipv4Addr::from_str(ip_string.as_ref()) {
        Ok(x) => {
            if let Some(combiner) = &*GLOBAL_COMBINER.lock().unwrap() {
                lua.push_boolean(combiner.contains(x));
            } else {
                lua.error("ipcheck.contains: Can't acquire combiner");
            }
        }
        Err(x) => {
            lua.error("ipcheck.contains: Failed to parse ip from input")
        }
    }


    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.environ.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        };
        ($func:ident, $name:literal) => {
            // _G.environ.$name
            lua.push_function($func);
            lua.set_field(-2, lua_string!($name));
        }
    }

    lua.create_table(0, 2);
    export_lua_function!(load);
    export_lua_function!(clear);
    export_lua_function!(contains);
    lua.set_global(lua_string!("ipcheck"));

    println!("ipcheck.gmod13_open");
    0
}

#[gmod13_close]
fn gmod13_close(lua: State) -> i32 {
    GLOBAL_COMBINER.lock().unwrap().take();

    println!("ipcheck.gmod13_close");
    0
}
