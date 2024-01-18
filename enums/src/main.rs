
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    VIP(f64, String)
}



fn main() {
    
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::VIP(30.0, "Amy".to_owned())
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price,holder) => {
                println!("Backstage tciket holder: {:?} price: {:?} ", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket Price: {:?} ",price),
            Ticket::VIP(price, holder) => {
                println!("VIP ticket holder: {:?} price: {:?} ", holder, price)
            }
        }
    }
}
