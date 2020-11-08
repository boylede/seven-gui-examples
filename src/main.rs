mod gui;

fn main() {
    gui::one::main().expect("GUI one failed");
    gui::two::main().expect("GUI two failed");
}