struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut result = Vec::new();
    for item in collection {
        if condition.is_match(&item) {
            result.push(item);
        }
    }
    result
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let condition = FilterCondition { value: 5 };

    let filtered = custom_filter(collection, &condition);

    println!("Filtered result: {:?}", filtered);
}
