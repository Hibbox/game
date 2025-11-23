use std::collections::HashSet;

#[derive(Debug)]
struct Pendu{
    dict: HashSet<String>,
    solution: String,
    word_hidden: String, // _

}

#[derive(Debug)]
struct Player{
    pv: u8,
    list_letter: HashSet<String>,
}

impl Player{
    pub fn new() -> Self{
    Self {
        pv : 11,
        list_letter : HashSet::new(),
        }   
    }
}

impl Pendu{
    pub fn new() -> Self{
        
        let mut dict = HashSet::new();
        dict.insert("eliot".to_string());
        dict.insert("remy".to_string());
        dict.insert("ronald".to_string());
        dict.insert("pain".to_string()); 

        Self{
            dict : dict,
            solution : String::new(),
            word_hidden : String::new(),
        }

    }

}

pub fn main(){
    println!("hello world");
}