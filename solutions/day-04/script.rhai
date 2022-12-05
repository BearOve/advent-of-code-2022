let part1_count = 0;
let part2_count = 0;

fn parse_range(a) {
    a[0].parse_int()..=a[1].parse_int()
}

for row in data.lines().split(',').split('-') {
    let a = parse_range(row[0]);
    let b = parse_range(row[1]);

    if a.start in b && a.end in b {
        part1_count += 1;
    } else if b.start in a && b.end in a {
        part1_count += 1;
    }

    if a.start in b || a.end in b || b.start in a || b.end in a {
        part2_count += 1;
    }
}

return [
    part1_count.to_string(),
    part2_count.to_string(),
]