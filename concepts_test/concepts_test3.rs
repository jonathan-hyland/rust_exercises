// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    twelve_days()
}

fn twelve_days() {
    let mut counter = 12;
    let mut phrase = String::new();
    let mut day = 1;
    let mut ending = String::new();

    while counter > 0 {
        match day {
            1 => { phrase = "a patridge in a pear tree.".to_string() },
            2 => { phrase = "two turtle doves and ".to_string() + &phrase},
            3 => { phrase = "three French hens, ".to_string() + &phrase},
            4 => { phrase = "four calling birds, ".to_string() + &phrase},
            5 => { phrase = "five golden rings, ".to_string() + &phrase},
            6 => { phrase = "six geese a-laying, ".to_string() + &phrase},
            7 => { phrase = "seven swans a-swimming, ".to_string() + &phrase},
            8 => { phrase = "eight maids a-milking, ".to_string() + &phrase},
            9 => { phrase = "nine ladies dancing, ".to_string() + &phrase},
            10 => { phrase = "ten lords a-leaping, ".to_string() + &phrase},
            11 => { phrase = "eleven pipers piping, ".to_string() + &phrase},
            12 => { phrase = "tweleve drummers drumming, ".to_string() + &phrase},
            _ => println!("OVER!"),
        }

        if day == 1 {
            ending = "st".to_string();
        } else if day == 2 {
            ending = "nd".to_string();
        } else if day == 3 {
            ending = "rd".to_string();
        } else {
            ending = "th".to_string();
        }

        println!("On the {}{} day of Christmas, my true love gave to me {}", day, ending, phrase);
        day += 1;
        counter -= 1;
    } 
}