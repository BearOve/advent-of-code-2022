let screen = #{
    cycles: 0,
    part1: 0,
    next_part1: 20,
    x: 1,
    screen: dynamic_image(40, 6),
    run_cycle: || {
        let pix = this.screen.pixel(this.cycles % 240);
        let sprite = range(this.x - 1, this.x + 2);

        pix.set(pix.x in sprite);

        this.cycles += 1;
        if this.cycles == this.next_part1 {
            this.part1 += this.cycles * this.x;
            this.next_part1 += 40;
        }
    }
};

for line in data.lines().split_once(' ') {
    let c = switch line[0] {
        "noop" => screen.run_cycle(),
        "addx" => {
            screen.run_cycle();
            screen.run_cycle();
            screen.x += line[1].parse_int();
        }
        _ => todo(),
    };
}

return [
    screen.part1.to_string(),
    screen.screen.to_ascii_art(".#"),
]