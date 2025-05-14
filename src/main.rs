use arrayvec::ArrayString;
use rand::random_range;

type WordleString = ArrayString<5>;

const DICTIONARY: &'static str = include_str!("../dictionary.txt");

/// The hints that wordle replies with
#[derive(Clone, Copy, Debug)]
enum WordleGuess {
    Invalid,
    Misplaced,
    Correct,
}


/// We either know for certain what a character is, or we don't know.
#[derive(Clone, Copy, Debug)]
enum WordleCharMatcher {
    Known(u8),
    Unknown
}

/// Store the known status of each character, then store up to 5 unique characters into the string
/// buffer
struct WordleMatcher ( [WordleCharMatcher; 5], ArrayString<5>, ArrayString<21> );
impl WordleMatcher {
    fn new() -> Self {
        Self ( [WordleCharMatcher::Unknown; 5], ArrayString::<5>::new(), ArrayString::<21>::new() )
    }
    fn add(&mut self, c: char) {
        if !self.1.contains(c) {
            self.1.push(c)
        }
    }
    fn remove(&mut self, c: char) {
        if !self.2.contains(c) {
            self.2.push(c)
        }
    }
    fn validate(&self, word: &WordleString) -> bool {
        for (i, c) in word.chars().enumerate() {
            let char_illegal = self.2.contains(c);
            // TODO: fix char::from(b)
            let char_known = if let WordleCharMatcher::Known(b) = self.0[i] { c == char::from(b) } else { true };
            if char_illegal || !char_known {
                return false;
            }
        }
        true
    }
}

/// The game is just guessing a hidden string
struct WordleGame {
    word: WordleString,
}

impl WordleGame {
    /// Self explanitory
    fn from_word(word: WordleString) -> Self {
        WordleGame { word }
    }

    /// Use the `rand` crate to randomly select a word from the provided dictionary. Create a game
    /// from the selected word.
    fn from_random(dictionary: &[WordleString]) -> Self {
        let random_index = random_range(0..dictionary.len());
        WordleGame::from_word(dictionary[random_index])
    }

    /// The user has guessed a word, compare the guess to the answer and return a hint for each
    /// character
    fn guess(&self, guess: WordleString) -> [WordleGuess; 5] {
        let mut result = [ WordleGuess::Invalid; 5 ];
        let real_word = &self.word.as_bytes();
        let guess = guess.as_bytes();
        for i in 0..5 {
            let is_correct = real_word[i] == guess[i];
            let contains_letter = real_word.contains(&guess[i]);
            result[i] = if is_correct { WordleGuess::Correct } else if contains_letter { WordleGuess::Misplaced } else { WordleGuess::Invalid };
        }
        result
    }
}

/// This does not necessarily need to be a struct, but is for the sake of potentially storing extra
/// data later
struct WordleSolver {}
impl WordleSolver {
    // fn new() -> Self { Self {} }
    
    fn update_pattern(mut pattern: WordleMatcher, hint: [WordleGuess; 5], guess: WordleString) -> WordleMatcher {
        let guess = guess.as_bytes();
        for i in 0..5 {
            match (pattern.0[i], hint[i]) {
                (WordleCharMatcher::Known(_), WordleGuess::Correct) => {}, // do nothing
                (WordleCharMatcher::Unknown, WordleGuess::Correct) => {
                    pattern.0[i] = WordleCharMatcher::Known(guess[i]);
                    pattern.add(char::from(guess[i]));
                },
                (_, WordleGuess::Misplaced) => {
                    // TODO: add guess[i] to the known characters
                    pattern.add(char::from(guess[i]));
                },
                (_, WordleGuess::Invalid) => {
                    // TODO: Somehow represent invalid characters
                    pattern.remove(char::from(guess[i]));
                },
            }
        }
        pattern
    }

    fn solve(game: WordleGame, dictionary: Vec<WordleString>) -> (WordleString, usize) {
        const MAX_GUESSES: usize = 15000;
        let mut word = WordleString::from("crane").unwrap();
        let mut filter_list = dictionary.clone();
        let mut pattern = WordleMatcher::new();
        println!("list starting at {}", filter_list.len());
        
        // We start at 1 guess
        for i in 1..MAX_GUESSES {
            let hint = game.guess(word);
            if matches![hint, [WordleGuess::Correct, WordleGuess::Correct, WordleGuess::Correct, WordleGuess::Correct, WordleGuess::Correct]] {
                return (word, i);
            }
            pattern = WordleSolver::update_pattern(pattern, hint, word);
            filter_list.retain(|w| !w.eq(&word) && pattern.validate(w));
            word = filter_list.first().unwrap().clone();
            println!("list reduced to {}", filter_list.len());
        }

        (word, MAX_GUESSES)
    }
}

fn main() {
    let dictionary: Vec<WordleString> = DICTIONARY.lines().into_iter().map(|line| WordleString::from(line)).flatten().collect();
    let game = WordleGame::from_random(&dictionary);
    let (word, guesses) = WordleSolver::solve(game, dictionary);

    println!("Found {} in {} tries!", word, guesses);
}
