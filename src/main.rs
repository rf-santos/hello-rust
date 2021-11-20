mod phs;
mod vars;

fn main() {
    println!("Hello, world!");
    phs::ph();
    phs::ph_arg();
    phs::ph_mults();
    phs::variable();
    phs::mut_var();
    vars::vars();
    vars::mut_vars();
    vars::multi_vars();
}