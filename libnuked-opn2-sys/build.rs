fn main() {
    println!("Building OPN2 C library");
    cc::Build::new()
        .file("nuked-opn2/ym3438.c")
        .include("nuked-opn2")
        .compile("ym3438");
}
