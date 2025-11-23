use std::collections::HashSet;
use rand::seq::IndexedRandom; // 0.9.0
use std::io;

struct Pendu{
    dict: HashSet<String>,
    solution: String,
    word_hidden: String, // _

}

struct Player<'a>{
    pv: u8,
    list_letter: HashSet<String>,
    game: &'a mut Pendu,
}

impl<'a> Player<'a>{
    pub fn new(pendu : &'a mut Pendu ) -> Self{
    Self {
        pv : 7,
        list_letter : HashSet::new(),
        game : pendu, // pointer vers les attributs du jeu Pendue

        }   
    }

    pub fn set_letter(&mut self){
        let mut stdin = io::stdin();
        let input = &mut String::new();
        stdin.read_line(input);
        let mut first_c = &input.as_str().chars().nth(0).unwrap();
        static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',];
        
        match (ASCII_LOWER.contains(first_c)){ // verifier l'input du 0 au dernier charactere
            true=>{
                self.list_letter.insert(first_c.to_string());

                self.verif_word(first_c);
            },
            false => {

                std::process::exit(1);
            }

        } 


    }

    pub fn verif_word( &mut self, first_c : &char){
         // reference vers le jeu pendu
        
        if self.game.solution.contains(first_c.to_string().as_str()){

            // chercher caractere dans hidden_word
            // iteration dans la l'attribut solution et pour chaque endroit ou la lettre associer est trouver on la met dans hidden word
            for (i, c) in self.game.solution.chars().enumerate() {
                // 
                if( c == *first_c) {
                    self.game.word_hidden.replace_range(i..i+1, c.to_string().as_str()); // on remplace le mot trouver et on s'arrete au caractere suivant 
                }

            }
            if !self.game.word_hidden.contains('_'){
                println!("Vous avez gagner!, le mot cacher etait {}", self.game.solution);
                std::process::exit(1);
            }

        }
        else{

            self.nb_essaie();
        }
        println!("il vous reste {} pv", self.pv);

    }

    pub fn nb_essaie(&mut self){

        let tab : Vec<&str> = vec![
    r#"
       +-------+
       |
       |
       |
       |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |
       |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |       |
       |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |      -|
       |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |      -|-
       |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |      -|-
       |      |
       |
    ==============
    "#,
    r#"
       +-------+
       |       |
       |       O
       |      -|-
       |      | |
       |
    ==============
    "#
    ];

    self.pv -= 1;
    let mut indice : usize = (6 - self.pv).into();
    println!("{}", &tab[indice]);
    if (self.pv == 0){
        println!("Vous etes mort !");
        std::process::exit(1);
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

    pub fn set_solution(&mut self){
        // assignation de mot a l'objet solution
        let mut v: Vec<_> = self.dict.clone().into_iter().collect(); //
        match v.choose(&mut rand::rng()){ // on attend un return Option<&Self::Item> permet d'eviter de pointer sur none et match permet d'etre dans le cas de Some ou None
        // si il y a rien dans le dictionnaire cela permet d'eviter au compilateur de buger
            Some(i) => {
                println!("{}", i);
                self.solution = i.to_string();
            },
            None =>{
                println!("Nothing !");
                std::process::exit(1);
            },
        }
    }

    pub fn set_prop(&mut self){

        let mut new_word = String::new();
        //savoir si il y a un premier caractere dans la solution
        match self.solution.as_str().chars().nth(0){
            Some(i) => {
                new_word.push(i);
            },
            None =>{
                println!("nothings");
                std::process::exit(1);
            }
        }

        for i in 0..self.solution.len()-2{
            new_word.push('_');
        }
        
        match self.solution.as_str().chars().last(){
            Some(i) => {
                new_word.push(i);
            },
            None =>{
                println!("nothings");
                std::process::exit(1);
            }
        }
        self.word_hidden = new_word;

    }

}


pub fn main(){
    
    
    let mut game = Pendu::new();

    game.set_solution();
    game.set_prop();
    println!("{:?}",game);

    {
        let mut player = Player::new(&mut game);
        loop{
            println!("{:#?}",player);
            player.set_letter();
        }     
    }

}