// This is a program that prints lyrics for the song Twelve Days of Christmas.

static DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

static GIFTS: [&str; 12] = [
    " partridge in a pear tree",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

static LEFT: &str = "On the ";
static RIGHT: &str = " day of Christmas,";
static SECOND: &str = "my true love gave to me";
static V1: &str = "A";
static V2: &str = "And a";

/// This function generates the lyrics for the song "The Twelve Days of Christmas" for a given day.
/// ```
/// let day = 0;
/// let lyrics = p22::song::stanza(day);
/// assert_eq!(lyrics, "On the first day of Christmas,\nmy true love gave to me\nA partridge in a pear tree.\n");
/// let day = 4;
/// let lyrics = p22::song::stanza(day);
/// assert_eq!(lyrics, "On the fifth day of Christmas,\nmy true love gave to me\nFive golden rings,\nFour calling birds,\nThree French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree.\n");
/// let day = 11;
/// let lyrics = p22::song::stanza(day);
/// assert_eq!(lyrics, "On the twelfth day of Christmas,\nmy true love gave to me\nTwelve drummers drumming,\nEleven pipers piping,\nTen lords a-leaping,\nNine ladies dancing,\nEight maids a-milking,\nSeven swans a-swimming,\nSix geese a-laying,\nFive golden rings,\nFour calling birds,\nThree French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree!\n");
/// ```
pub fn stanza(day: usize) -> String {
    let mut result = format!("{}{}{}", LEFT, DAYS[day], RIGHT);
    result.push_str(&format!("\n{}\n", SECOND));
    for i in (0..=day).rev() {
        if i == 0 && day != 0 {
            result.push_str(V2);
        } else if i == 0 {
            result.push_str(V1);
        }
        result.push_str(GIFTS[i]);
        if i == 0 && day == 11 {
            result.push('!');
        } else if i == 0 {
            result.push('.');
        }
        result.push('\n');
    }
    result
}

/// This function prints the lyrics for all 12 days of Christmas.
pub fn lyrics() {
    for day in 0..12 {
        println!("{}", stanza(day));
    }
}
