let elves = int_array([0]);

for line in data.lines() {
    if !line.is_empty() {
        elves[-1] += line.parse_int();
    } else {
        elves.push(0);
    }
}

debug("All elves: " + elves);

let top_elf = elves.max();
let top3_elves = elves.sort().extract(-3);

debug("Top elf: " + top_elf);
debug("Top 3 elves: " + top3_elves);

return [
    top_elf.to_string(),
    top3_elves.sum().to_string(),
]
