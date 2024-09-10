pub struct AveragedCollection {
    list: Vec<i32>, // вектор чисел
    average: f64,   // среднеарифметическое чисел вектора
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
