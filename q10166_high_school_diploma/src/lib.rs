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

        let shir_farhad = Student::new(String::from("shir farhad"), String::from("2123"), &keys);

        assert_eq!(nezam.score, 7);
        assert_eq!(shir_farhad.score, 7);
        assert_eq!(keyvoon.score, 7);
    }
}
