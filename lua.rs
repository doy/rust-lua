#[link(name = "lua", vers = "0.0.1", author = "doy")];

#[crate_type = "lib"];

pub enum lua_State {}
enum lua_CFunction {}

pub enum Status {
    OK        = 0,
    YIELD     = 1,
    ERRRUN    = 2,
    ERRSYNTAX = 3,
    ERRMEM    = 4,
    ERRGCMM   = 5,
    ERRERR    = 6,
}

pub fn newstate() -> *lua_State {
    lua::luaL_newstate()
}

pub fn close(state: *lua_State) {
    lua::lua_close(state)
}

pub fn openlibs(state: *lua_State) {
    lua::luaL_openlibs(state)
}

pub fn loadstring(state: *lua_State, string: &str) -> Status {
    let status = do str::as_c_str(string) |c_string| {
        lua::luaL_loadstring(state, c_string)
    };
    match status {
        0 => OK,
        3 => ERRSYNTAX,
        4 => ERRMEM,
        5 => ERRGCMM,
        _ => fail ~"Unexpected status in loadstring",
    }
}

pub fn call(state: *lua_State, nargs: int, nresults: int) {
    lua::lua_callk(
        state,
        nargs as libc::c_int,
        nresults as libc::c_int,
        0 as libc::c_int,
        0 as *lua_CFunction
    )
}

extern mod lua {
    fn luaL_newstate() -> *lua_State;
    fn lua_close(state: *lua_State);
    fn luaL_openlibs(state: *lua_State);

    fn luaL_loadstring(
        state: *lua_State,
        string: *libc::c_char
    ) -> libc::c_int;
    fn lua_callk(
        state: *lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: libc::c_int,
        k: *lua_CFunction
    );
}
