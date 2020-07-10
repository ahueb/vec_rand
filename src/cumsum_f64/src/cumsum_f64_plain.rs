pub fn cumsum_f64_plain(random_vec: &Vec<f64>) -> Vec<f64> {
    let mut cumulative_sum: Vec<f64> = Vec::with_capacity(random_vec.len());
    let mut total_weight = 0f64;
    for w in random_vec {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }
    cumulative_sum
}