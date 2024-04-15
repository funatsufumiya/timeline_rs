#[cfg(feature="bevy_example")]
mod bevy_example_impls;

#[cfg(feature="bevy_example")]
fn main() {
    bevy_example_impls::easing_tester_impl::main();
}

#[cfg(not(feature="bevy_example"))]
fn main() {
    use log::error;

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    error!("Error: This example requires the `bevy_example` feature to be enabled.");
}
