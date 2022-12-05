let stacks = [];
let stacks_part1 = [];
let stacks_part2 = [];

let parse_stack_layout = true;

const DEBUG_PRINT = 0;

fn print_stacks(stacks) {
    let max_len = 0;
    for stack in stacks {
        max_len = stack.len().max(max_len);
    }
    for i in range(max_len - 1, -1, -1) {
        let line = "";
        for stack in stacks {
            if i < stack.len() {
                line += stack[i];
                line += " ";
            } else {
                line += "    ";
            }
        }
        print(line);
    }
    let line = " ";
    for i in range(1, stacks.len() + 1) {
        line += i.to_string();
        line += "   ";
    }
    print(line);
    print();
}

for (line, ln) in data.lines() {
    if parse_stack_layout {
        if line == "" {
            parse_stack_layout = false;
            if DEBUG_PRINT != 0 { print_stacks(stacks) }
            stacks_part1.append(stacks);
            stacks_part2.append(stacks);
        } else {
            for (item, stack_i) in line.chunks(4) {
                item.trim();

                if stacks.len() == stack_i {
                    stacks.push([]);
                }
                if item == "" {
                } else if item.starts_with("[") {
                    stacks[stack_i].insert(0, item);
                } else {
                    assert_eq(item.parse_int(), stack_i + 1);
                }
            }
        }
    } else {
        let p = line.split(' ');
        let count = p[1].parse_int();
        let from = p[3].parse_int() - 1;
        let to = p[5].parse_int() - 1;

        let to_move = stacks_part1[from].split(-count);
        to_move.reverse();
        stacks_part1[to].append(to_move);
        if (DEBUG_PRINT & 1) == 1 { print_stacks(stacks_part1) }

        stacks_part2[to].append(stacks_part2[from].split(-count));
        if (DEBUG_PRINT & 2) == 2 { print_stacks(stacks_part2) }
    }
}

let part1 = "";
for stack in stacks_part1 {
    part1 += stack[-1].sub_string(1, 1);
}

let part2 = "";
for stack in stacks_part2 {
    part2 += stack[-1].sub_string(1, 1);
}


return [
    part1,
    part2,
]