fn req_prefix(data, pfx) {
    let line = data.next_line();
    if line == () {
        return;
    }
    let ret = line.strip_prefix(pfx);
    if ret == () {
        throw "Expected " + pfx.to_debug() + ", but got " + line.to_debug();
    }
    ret
}

fn make_op(s) {
    |old| eval(s)
}

fn parse_monkey(data) {
    if req_prefix(data, "Monkey ") == () {
        return;
    }

    #{
        items: req_prefix(data, "  Starting items: ").split(", ").map(|v| v.parse_int()),
        num_inspected: 0,
        op: make_op(req_prefix(data, "  Operation: new = ")),
        div_by: req_prefix(data, "  Test: divisible by ").parse_int(),
        then: tuple(
            req_prefix(data, "    If true: throw to monkey ").parse_int(),
            req_prefix(data, "    If false: throw to monkey ").parse_int(),
        ),
        last_line: data.next_line(),
    }
}

fn run_part1(monkeys) {
    monkeys.op = |v| v / 3;
    for round in 0..20 {
        monkeys.run_round();
    }
    monkeys.finalize_monkey_business()
}

fn run_part2(monkeys) {
    let div_by = 1;
    for i in 0..monkeys.entries.len() {
        div_by *= monkeys.entries[i].div_by;
    }

    monkeys.op = |v| v % div_by;

    for round in 0..10_000 {
        monkeys.run_round();
    }
    debug(monkeys.counts());

    monkeys.finalize_monkey_business()
}

let monkeys = #{
    entries: [],
    op: (),
    run_round: || {
        for i in 0..this.entries.len() {
            let then = this.entries[i].then;
            let div_by = this.entries[i].div_by;
            this.entries[i].num_inspected += this.entries[i].items.len();
            for item in this.entries[i].items {
                let new_item = this.op(this.entries[i].op(item));
                let j = if (new_item % div_by) == 0 {
                    then[0]
                } else {
                    then[1]
                };
                this.entries[j].items.push(new_item);
            }
            this.entries[i].items = [];
        }
    },
    counts: || {
        this.entries.map(|m| m.num_inspected)
    },
    finalize_monkey_business: || {
        let tmp = this.counts();
        tmp.sort();
        tmp[-2] * tmp[-1]
    }
};
loop {
    let monkey = parse_monkey(data);
    if monkey == () {
        break;
    }
    monkeys.entries.push(monkey);
}



return [
    run_part1(monkeys).to_string(),
    run_part2(monkeys).to_string(),
]