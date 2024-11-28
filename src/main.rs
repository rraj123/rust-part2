
#[derive(Debug)]
struct Deck{
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self{
        let suits =["Hearts","Diamond","Spades"];
        let values =["Ace","Two","Three"];
    
        let mut cards = vec![];
    
        for suit in suits{
            for value in values {
               let card = format!("{} of {}",value, suit);   
               cards.push(card);  
            }
        }
        
        let deck = Deck{cards};
        return deck;
    }

}


fn main() {
    let deck = Deck::new();
   
    
    println!("Heres your deck {:#?}", deck)

}
