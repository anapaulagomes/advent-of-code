pub fn resulting_frequency(numbers: Vec<&str>) -> i32 {
    let mut sum = 0;

    for number in numbers {
        let change_of = number.parse::<i32>().unwrap();
        sum += change_of;
    }

    sum
}

pub fn first_frequency(numbers: Vec<&str>) -> i32 {
    let mut current_frequency = 0;
    let mut frequency = Vec::new();
    let mut index = 0;

    loop {
        let change_of = numbers[index].parse::<i32>().unwrap();
        
        if frequency.contains(&current_frequency) {
            return current_frequency
        } else {
            frequency.push(current_frequency);
        }
        current_frequency += change_of;
        
        if index + 1 == numbers.len(){
            index = 0;
        } else {
            index += 1;
        }
    }

}

#[cfg(test)]
mod tests_resulting_frequency {
    use super::*;

    #[test]
    fn test_main_example() {
        let numbers = vec!["+1", "-2", "+3", "+1"];
        assert_eq!(resulting_frequency(numbers), 3);
    }

    #[test]
    fn test_positive_numbers_only() {
        let numbers = vec!["+1", "+1", "+1"];
        assert_eq!(resulting_frequency(numbers), 3);
    }

    #[test]
    fn test_random_example() {
        let numbers = vec!["+1", "+1", "-2"];
        assert_eq!(resulting_frequency(numbers), 0);
    }

    #[test]
    fn test_negative_numbers_only() {
        let numbers = vec!["-1", "-2", "-3"];
        assert_eq!(resulting_frequency(numbers), -6);
    }
}

#[cfg(test)]
mod tests_first_frequency {
    use super::*;

    #[test]
    fn test_main_example() {
        let numbers = vec!["+1", "-2", "+3", "+1", "+1", "-2"];
        assert_eq!(first_frequency(numbers), 2);
    }
    
    #[test]
    fn test_check_two_numbers() {
        let numbers = vec!["+1", "-1"];
        assert_eq!(first_frequency(numbers), 0);
    }

    #[test]
    fn test_reaches_ten() {
        let numbers = vec!["+3", "+3", "+4", "-2", "-4"];
        assert_eq!(first_frequency(numbers), 10);
    }

    #[test]
    fn test_reaches_five() {
        let numbers = vec!["-6", "+3", "+8", "+5", "-6"];
        assert_eq!(first_frequency(numbers), 5);
    }

    #[test]
    fn test_reaches_fourteen() {
        let numbers = vec!["+7", "+7", "-2", "-7", "-4"];
        assert_eq!(first_frequency(numbers), 14);
    }
}
