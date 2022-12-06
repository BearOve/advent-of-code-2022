let part1 = [];
let part2 = [];

fn calc_start(line, n) {
    let prev = blob(n);
    for (c, i) in line {
        prev[i % n] = c.to_int();
        if i >= n && prev.unique_count() == n {
            return i + 1;
        }
    }
}

for line in data.blobs() {
    part1.push(calc_start(line, 4));
    part2.push(calc_start(line, 14));
}

return [
    part1.to_string(),
    part2.to_string(),
]