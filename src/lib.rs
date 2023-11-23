pub fn collect_lines(path: &str) -> std::io::Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(&str::to_string)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
}
