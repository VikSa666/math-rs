pub fn parse_matrix(input: &str) -> Vec<Vec<i32>> {
    let mut matrix = vec![];
    let inner = input.trim_start_matches('{').trim_end_matches('}').trim();
    for row_str in inner.split("},{") {
        let row = row_str.split(',').map(|s| s.parse().unwrap()).collect();
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod test {}
