use std::env;
use rand::seq::SliceRandom;
use rand::thread_rng;
// ill add more quotes later dw
fn main() {
    let quotes = [
        "",
        "Software is like sex! It’s better when it is free. -Linus Torvalds",
        "The oozing Crest of corruption... HADO NUMBER 90, KUROHITSUGI! -Sosuke Aizen",
        "You asked me why I'm doing all this? ... So Naruto, how would you confront this hatred in order to create peace? I want to know what your answer is. -Pain",
        "Everybody makes a wrong turn once in a while. -Ash Ketchum",
        "I always try to do my best and I know I am getting better!!! -Ash Ketchum",
        "This world is rotten. -Light Yagami",
        "Humans are so interesting... -Ryuk",
        "I am justice. -Kira",
        "pika pika -Pikachu",
        "I use Arch btw -probably you",
        "Yo btw 90% of gamblers who quit were 3 games away from jackpot",
        "Maximum effort! -Deadpool",
        "There are 206 bones in the human body. 207 if I’m watching Gossip Girl. -Deadpool",
        "Will you take the road to light, or the road to darkness? -DiZ",
        "No one would miss me. -Roxas",
        "It looks like my summer vacation is over. -Roxas",
        "May your heart be the guiding key.",
        "No matter where we are, our hearts will bring us back together.",
        "Microsoft fucking sucks ass",
        "My Bluetooth driver made my GPU stop working. -ry4c",
        "Stupid people can believe in anything, so you can believe in yourself! -Joseph Joestar",
    ];

    let args: Vec<String> = env::args().collect();

    
    if args.len() == 2 && args[1] == "--quote" {
        let mut rng = thread_rng();
        if let Some(quote) = quotes.choose(&mut rng) {
            println!("{}", quote);
        }
    } else {
        eprintln!("Usage: tell --quote");
    }
}
