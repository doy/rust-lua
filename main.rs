extern mod lua;

fn main() {
    let state = lua::newstate();
    state.openlibs();
    state.loadstring(os::args()[1]);
    state.call(0, 0);
    state.close();
}
