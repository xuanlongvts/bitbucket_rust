struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}

fn main() {
    let mut ave = AveragedCollection {
        list: vec![1, 2],
        average: 1.5,
    };
    ave.new(3);

    println!("list:     ===> {:?} ", ave.list);
    println!("average:  ===> {} ", ave.average());

    ave.remove();

    println!("list 2:   ===> {:?} ", ave.list);
    println!("average2: ===> {} ", ave.average());
}
