use std::io;

use lib::Student;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (keys_length, keys) = get_input_from_user()?;

    // adding students and their pattern of answers
    let students = vec![
        Student::new(String::from("keyvoon"), String::from("331122"), &keys),
        Student::new(String::from("nezam"), String::from("123"), &keys),
        Student::new(String::from("shir farhad"), String::from("2123"), &keys),
    ];

    // find the highest score
    let mut max_score: u8 = 0;
    for student in students.iter() {
        if student.score > max_score {
            max_score = student.score;
        }
    }
    println!("{}", max_score);

    // sort and print students with highest score
    students
        .iter()
        .filter(|student| student.score == max_score)
        .for_each(|student| println!("{}", student.name));

    Ok(())
}

fn get_input_from_user() -> Result<(u8, String), Box<dyn std::error::Error>> {
    // get input from user
    let keys_length: u8;
    let mut keys = String::new();

    let mut keys_length_temp = String::new();
    io::stdin().read_line(&mut keys_length_temp)?;
    keys_length = keys_length_temp
        .trim()
        .parse()
        .expect("Error happened here");
    io::stdin().read_line(&mut keys)?;

    Ok((keys_length, keys))
}

mod lib {
    pub struct Student {
        pub name: String,
        pattern: String,
        pub score: u8,
    }

    impl Student {
        pub fn new(name: String, pattern: String, keys: &String) -> Self {
            let mut new_student = Self {
                name: name,
                pattern: pattern,
                score: 0,
            };

            new_student.score(keys);
            new_student
        }

        fn score(self: &mut Self, keys: &String) {
            // create answers from pattern
            let answers = self.pattern.repeat((keys.len() / self.pattern.len()) + 1);

            let mut answers_iter = answers.chars();

            for key in keys.chars() {
                if key == answers_iter.next().unwrap() {
                    self.score += 1;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn same_answers() {
            let keys = String::from("123123");
            let student = Student::new(String::from("Ali"), String::from("1"), &keys);
            assert_eq!(student.score, 2);
        }

        #[test]
        fn quere_example_1() {
            let keys = String::from("111323311123111");
            let keyvoon = Student::new(String::from("keyvoon"), String::from("331122"), &keys);

            let nezam = Student::new(String::from("nezam"), String::from("123"), &keys);

            let shir_farhad =
                Student::new(String::from("shir farhad"), String::from("2123"), &keys);

            assert_eq!(nezam.score, 7);
            assert_eq!(shir_farhad.score, 7);
            assert_eq!(keyvoon.score, 7);
        }
    }
}
