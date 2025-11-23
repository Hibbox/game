
mod function;
use function::menu;
use function::show;


/*
fn show(var: &str){
    
    let var_manipulable = String::from(var);
    println!("{}", var_manipulable);

}


fn main() {
    let var = "BIENVENUE SUR LE MENU"; //str
    show(&var);
    println!("Hello, world!");
}
*/



fn main() {
    

    let var = "BIENVENUE SUR LE MENU"; //str
    let var_manipulable = String::from(var);
    show(var_manipulable);
    menu();
}