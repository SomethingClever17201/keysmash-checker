use std::collections::HashMap;
use std::error::Error;
use std::fs;


//what is a keysmash?
//if >= 50% of the message isn't a word. keysmash
//word is defined by out.csv
fn main() -> Result<(), Box<dyn Error>> {

    let input : String = "antidisestablishmentarianism unfaiiiiiiiir".to_string();
    let mut valid_words : HashMap<String,Vec<String>>  = HashMap::new();
    let messages_bulk:  String = fs::read_to_string("out.csv")?;

    //get all words by splitting on new line
    let messages = messages_bulk.split("\n");

    //get first character, construct hashmap of "first_letter-length->Vector of words that start"
    for message in messages {
        let content = message.split(",").next().unwrap();

        let Some(first_character) = content.chars().next() else {continue; };
        let valid_key = format!("{}-{}",first_character.to_string(),content.chars().count());

        if !valid_words.contains_key(&valid_key) {
            let value : String = valid_key.clone();
            valid_words.insert(value, Vec::<String>::new());
        }
        let word_list : &mut Vec<String> = valid_words.get_mut(&valid_key).unwrap();

        let content_string = String::from(content);
        if !word_list.contains(&content_string){
            word_list.push(content_string);
        }
        // println!("{} {}", valid_key,word_list.join(","));
    }

    let words_in_message = input.split(" ");
    let word_count = words_in_message.clone().count();

    let mut keysmash = 0;
    for word in words_in_message {
        let Some(first_character) = word.chars().next() else {return Ok(());};

        let key = format!("{}-{}",first_character.to_string(), word.chars().count());

        if valid_words.contains_key(&key) {
            let word_list : &mut Vec<String> = valid_words.get_mut(&key).unwrap();
            if word_list.contains(&word.to_string()) {
                continue;
            }
        }
        keysmash += 1;
    }
    if keysmash >= (word_count / 2){
        println!{"You've keysmashed, cutie"};
    }
    else { 
        println!{"No Keysmash"};
    }

    Ok(())
}

