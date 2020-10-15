fn main() {
    let days = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        let suffix = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!(
            "\nOn the {}{} day of Christmas my true love gave to me",
            day + 1,
            suffix
        );

        for item in (0..day + 1).rev() {
            if day > 0 && item == 0 {
                print!("and ")
            }

            println!("{}", days[item])
        }
    }
}
