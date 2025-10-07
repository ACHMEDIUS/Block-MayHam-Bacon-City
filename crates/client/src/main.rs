mod app;
mod plugins;
mod states;
mod systems;

mod components {
    //! Client-only components
}

mod resources {
    //! Client-only resources
}

fn main() {
    app::run();
}
