use std::io;
mod pendu;
mod morpion;


pub fn show(var: String){
    
    println!("{}", var);

}

pub fn menu(){

    let mut stdin = io::stdin();
    let input = &mut String::new();

    println!("choisier une option: \n\t1.option:\tPendu\n\t2.option:\tMorpion");
    
    stdin.read_line(input);
    println!("{}", input);

    match &input.as_str()[0..input.as_str().len() -1]  {
        "1" => {
            println!("Pendu");
            pendu::main();
        },
        "2" => {
            println!("Morpion");
            morpion::main();
        },
        _ => {  
            println!("exit");
            std::process::exit(1);
        }
    }

}