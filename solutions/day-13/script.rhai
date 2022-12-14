fn compare_pair(left, right) {
    let ltyp = type_of(left);
    let rtyp = type_of(right);
    let array = ltyp == "array" || rtyp == "array";
    if array && ltyp == "i64" {
        left = [left];
    } else if array && rtyp == "i64" {
        right = [right];
    }

    if array {
        for i in 0..left.len() {
            if i < right.len() {
                let ret = compare_pair(left[i], right[i]);
                if ret != 0 {
                    return ret;
                }
            } else {
                return left.len() - right.len();
            }
        }

        left.len() - right.len()
    } else if ltyp == "i64" {
        left - right
    } else {
        todo()
    }
}

let pair = [];
let pairs = [];
let packets = [];

for line in data.lines() {
    if pair.len() < 2 {
        assert(line.to_chars().all(|c| "[],0123456789".contains(c))); // Make sure eval is safe
        pair.push(eval(line));
        packets.push(pair[-1]);

        if pair.len() == 2 {
            pairs.push(pair);
        }
    } else {
        assert_eq(line, "");
        pair.clear();
    }
}

let part1 = 0;

for (pair, i) in pairs {
    if compare_pair(pair[0], pair[1]) < 0 {
        part1 += i + 1;
    }
}

packets.push([[2]]);
packets.push([[6]]);
packets.sort(|a, b| compare_pair(a, b));

let part2 = (packets.index_of([[2]]) + 1) * (packets.index_of([[6]]) + 1);

return [
    part1.to_string(),
    part2.to_string(),
]