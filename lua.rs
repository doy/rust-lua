#[link(name = "lua", vers = "0.0.1", author = "doy")];

#[crate_type = "lib"];

pub struct State {
    priv state: *lua_State,
}

impl State {
    fn close(self) {
        close(self)
    }

    fn openlibs(self) {
        openlibs(self)
    }

    fn loadstring(self, string: &str) -> Status {
        loadstring(self, string)
    }

    fn call(self, nargs: int, nresults: int) {
        call(self, nargs, nresults)
    }
}

pub enum Status {
    OK        = 0,
    YIELD     = 1,
    ERRRUN    = 2,
    ERRSYNTAX = 3,
    ERRMEM    = 4,
    ERRGCMM   = 5,
    ERRERR    = 6,
}

pub fn newstate() -> State {
    State { state: lua::luaL_newstate() }
}

pub fn close(state: State) {
    lua::lua_close(state.state)
}

pub fn openlibs(state: State) {
    lua::luaL_openlibs(state.state)
}

pub fn loadstring(state: State, string: &str) -> Status {
    let status = do str::as_c_str(string) |c_string| {
        lua::luaL_loadstring(state.state, c_string)
    };
    match status {
        0 => OK,
        3 => ERRSYNTAX,
        4 => ERRMEM,
        5 => ERRGCMM,
        _ => fail ~"Unexpected status in loadstring",
    }
}

pub fn call(state: State, nargs: int, nresults: int) {
    lua::lua_callk(
        state.state,
        nargs as libc::c_int,
        nresults as libc::c_int,
        0 as libc::c_int,
        0 as *lua_CFunction
    )
}

enum lua_State {}
enum lua_CFunction {}

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
