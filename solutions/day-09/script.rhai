fn Rope(knots) {
    let ret = #{
        head: tuple(0, 0),
        tail: [],
        part1: fixed_set(tuple(0, 0)),
        part2: fixed_set(tuple(0, 0)),
        move: |count, pos| {
            for round in 0..count {
                this.move_ones(round, pos);
            }
        },
        move_ones: |round, pos| {
            this.head += pos;

            let cur = this.head;
            for i in 0..this.tail.len() {
                let d = cur - this.tail[i];
                if d[0].abs() > 1 {
                    this.tail[i] += tuple(d[0], d[1]).sign()
                } else if d[1].abs() > 1 {
                    this.tail[i] += tuple(d[0], d[1]).sign()
                } else {
                    break;
                }
                cur = this.tail[i];
            }

            this.part1.insert(this.tail[0]);
            this.part2.insert(this.tail[-1]);
        },
    };
    ret.tail.pad(knots - 1, tuple(0, 0));
    ret
}

let rope = Rope(10);

for line in data.lines().split_once(' ') {
    let count = line[1].parse_int();
    let pos = switch line[0] {
        "L" => tuple(-1, 0),
        "R" => tuple(1, 0),
        "U" => tuple(0, -1),
        "D" => tuple(0, 1),
        _ => todo(line),
    };

    rope.move(count, pos);
}

return [
    rope.part1.len().to_string(),
    rope.part2.len().to_string(),
]
