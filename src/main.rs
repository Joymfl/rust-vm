mod memory;
mod registers;
mod op_codes;
mod cond_flags;

fn main() {
    println!("{}", u16::MAX as usize + 1);
}