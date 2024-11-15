fn main() {
    // This code is also going to be basic as we will first need to learn String in depth to go in
    // depth with other codes.

    let is_even: bool = true;

    if is_even {
        println!("This is even number.");
    } else if !is_even {
        println!("Not even number.");
    } else {
        println!("I don't know what is it.");
    }

    // For loop with range.
    // Basic for loop

    for _ in 0..10 {
        println!("This is the just to print this statement 10 times.");
    }

    for i in 0..10 {
        println!("Current value of i is : {}", i);
    }

    let sentence: String = String::from("Apoorv");
    let first_word: String = get_first_word(sentence);

    println!("The first word is {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::new();

    //for loop for iterating over iterables here String, but we can also do if for array, heap and
    //other iterables.
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans;
}
