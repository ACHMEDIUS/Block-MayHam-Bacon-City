mod app;
mod plugins;
mod systems;

mod components {
    //! Server-only components
}

mod resources {
    //! Server-only resources
}

fn main() {
    app::run();
}
