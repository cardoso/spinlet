use spinlets::Spinlet;

pub fn main() {
    let spin = Spinlet::get();
    spin.print_line("---------AFTER BUILD-----------");
    spin.print_line("Doing some stuff after build...");
    spin.print_line("---------AFTER BUILD-----------");
}