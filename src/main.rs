fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-living",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"];

    let mut index = 0;
    for day in days {
        println!("On the {} day of Christmas", day);
        println!("My true love gave to me");
        let mut counter = index;
        loop {
            if counter == 1 {
                println!("{}, and",gifts[counter]);
            } else {
                println!("{}",gifts[counter]);
            }
            if counter == 0 {
                break;
            }
            counter -= 1
        }
        println!("");
        index += 1
    }
}
