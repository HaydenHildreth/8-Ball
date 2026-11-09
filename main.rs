const ANSWERS: [&str; 20] = [
    // Affirmative
    "It is certain.",
    "It is decidely so.",
    "Without a doubt.",
    "Yes, definitely.",
    "You may rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",

    // Non-Commital
    "Reply hazy try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",

    // Negative
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
];

fn main() {
    println!("The 8 Ball is deciding your fate...\n");

    println!("{}", generate_random_number());
}

fn generate_random_number() -> &'static str {
    // 20 possible answers, 10 affirmative, 5 non-commital, 5 negative
    let random_number = rand::random_range(0..ANSWERS.len());
    ANSWERS[random_number]
}
