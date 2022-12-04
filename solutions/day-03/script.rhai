let part1_score = 0;
let part2_score = 0;
let group = [];

fn calc_pri(val) {
    let val = val.to_int();
    switch val {
        97..=122 => val - 96,
        65..=90 => val - 38,
    }
}

for comp1 in data.blobs() {
    group.push(comp1);

    if group.len() == 3 {
        for c in group[0].intersection(group[1]).intersection(group[2]) {
            part2_score += calc_pri(c);
        }
        group = [];
    }

    let comp2 = comp1.split(comp1.len() / 2);

    for c in comp1.intersection(comp2) {
        part1_score += calc_pri(c);
    }
}

return [
    part1_score.to_string(),
    part2_score.to_string(),
]