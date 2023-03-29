mod r1;
mod r2;

macro_rules! exec {
    ($name:ident) => {
        println!("=> {} => exec", stringify!($name));
        $name::main();
        println!();
        println!("<= {} <= finish", stringify!($name));
    };
}

fn main() {
    println!("https://dtolnay.github.io/rust-quiz/");

    exec!(r1);
    exec!(r2);
}
