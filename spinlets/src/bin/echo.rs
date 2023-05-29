use spinlets::Spinlet;

pub fn main() {
    let spin = Spinlet::get();
    spin.print_line("---------BEFORE BUILD-----------");
    spin.print_line("Your build will run with the following arguments:");
    for arg in std::env::args() {
        spin.print(" - ");
        spin.print_line(&arg);
    }
    spin.print_line("---------AFTER BUILD-----------");
}