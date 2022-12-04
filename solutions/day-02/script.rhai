const ROC = 1;
const PAP = 2;
const SCI = 3;

fn get_score_part1(opponent, you) {
    switch opponent {
        "A" => switch you {
            "X" => global::ROC + 3,
            "Y" => global::PAP + 6,
            "Z" => global::SCI + 0,
        },
        "B" => switch you {
            "X" => global::ROC + 0,
            "Y" => global::PAP + 3,
            "Z" => global::SCI + 6,
        },
        "C" => switch you {
            "X" => global::ROC + 6,
            "Y" => global::PAP + 0,
            "Z" => global::SCI + 3,
        },
    }
}

fn get_score_part2(opponent, you) {
    switch opponent {
        "A" => switch you {
            "X" => global::SCI + 0,
            "Y" => global::ROC + 3,
            "Z" => global::PAP + 6,
        },
        "B" => switch you {
            "X" => global::ROC + 0,
            "Y" => global::PAP + 3,
            "Z" => global::SCI + 6,
        },
        "C" => switch you {
            "X" => global::PAP + 0,
            "Y" => global::SCI + 3,
            "Z" => global::ROC + 6,
        },
    }
}

let score_part1 = 0;
let score_part2 = 0;

for line in data.lines() {
    let row = line.split();
    score_part1 += get_score_part1(row[0], row[1]);
    score_part2 += get_score_part2(row[0], row[1]);
}

return [
    score_part1.to_string(),
    score_part2.to_string(),
]