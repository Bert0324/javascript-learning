use std::env;
mod l_1;
mod l_102;
mod l_103;
mod l_107;
mod l_144;
mod l_145;
mod l_297;
mod l_94;
mod l_987;
mod tree_builder;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let module_name = args[1].as_str();
    println!("Start No.{}", module_name);
    match module_name {
        "1" => println!("ret: {:?}", l_1::run()),
        "94" => println!("ret: {:?}", l_94::run()),
        "102" => println!("ret: {:?}", l_102::run()),
        "103" => println!("ret: {:?}", l_103::run()),
        "107" => println!("ret: {:?}", l_107::run()),
        "144" => println!("ret: {:?}", l_144::run()),
        "145" => println!("ret: {:?}", l_145::run()),
        "297" => println!("ret: {:?}", l_297::run()),
        "987" => println!("ret: {:?}", l_987::run()),
        _ => println!("no matched module: {:?}", module_name),
    }
}
