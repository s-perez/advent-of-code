
pub struct Reading {
    pub value: Vec<i32>
}

pub struct PowerConsumption {
    pub gamma_rate: u32,
    pub epsilon_rate: u32,
    pub value: u32
}

impl Reading {

    pub fn get_power_consumption(&self) -> PowerConsumption {
        let gamma_rate = self.value.iter()
            .map(Reading::get_gamma_digit)
            .fold(0, |acc, b| acc * 2 + b);

        let epsilon_rate = self.value.iter()
            .map(Reading::get_epsilon_digit)
            .fold(0, |acc, b| acc * 2 + b);

        PowerConsumption{
            gamma_rate: gamma_rate,
            epsilon_rate: epsilon_rate,
            value: gamma_rate * epsilon_rate
        }
    }

    pub fn from_str(line: String) -> Reading {
        Reading{value: line
            .chars()
            .map(map_boolean)
            .collect::<Vec<i32>>()}
    }

    pub fn reducer(acc: Reading, element: Reading) -> Reading {
        let acc_value = acc.value;
        let element_value = element.value;
        Reading{value: acc_value
            .into_iter()
            .zip(element_value.into_iter())
            .map(|(x, y)| x + y)
            .collect::<Vec<i32>>()}
    }

    fn get_gamma_digit(digit: &i32) -> u32 {
        if digit > &0 {
            1
        } else {
            0
        }
    }

    fn get_epsilon_digit(digit: &i32) -> u32 {
        if digit < &0 {
            1
        } else {
            0
        }
    }
}

fn map_boolean(c: char) -> i32 {
    match c {
        '1' => 1,
        '0' => -1,
        _ => 0
    }
}
