
pub enum DeJucarie {
    Egld(u32),
    Achievement(String),
    Usdc
}

pub fn print_juc(j: &DeJucarie) {
    match j {
        DeJucarie::Egld(x) => println!("EGLD: {x}"),
        DeJucarie::Achievement(congrats) => println!("Achievemnt unlocked: {congrats}"),
        DeJucarie::Usdc => println!("Usdc"),
    }
}
