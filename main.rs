extern mod lua;

fn main() {
    let state = lua::newstate();
    lua::openlibs(state);
    lua::loadstring(state, os::args()[1]);
    lua::call(state, 0, 0);
    lua::close(state);
}
