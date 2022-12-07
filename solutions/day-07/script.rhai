let fs = #{
    raw_cwd: blob(),
    cwd: [""],
    dirs: #{"": #{size: 0, files: []}},
    enter: |name| {
        this.raw_cwd.push(0x2f);
        this.raw_cwd.append(name);
        this.cwd.push(this.raw_cwd.as_string());
        if !(this.cwd[-1] in this.dirs) {
            this.dirs[this.cwd[-1]] = #{
                size: 0,
                files: [],
            };
        }
    },
    leave: || {
        this.raw_cwd.rstrip_off(0x2f);
        this.cwd.pop();
    },
    set_file: |name, size| {
        for d in this.cwd {
            this.dirs[d].size += size;
        }
        this.dirs[this.cwd[-1]].files.push(#{name: name, size: size});
    },
};


for line in data.blobs() {
    if line.strip_prefix("$ ") {
        if line == "cd /" {
            assert_eq(fs.cwd.len(), 1);
        } else if line == "cd .." {
            fs.leave();
        } else if line == "ls" {
        } else if line.strip_prefix("cd ") {
            fs.enter(line);
        } else {
            assert(false);
        }
    } else if line.strip_prefix("dir ") {
        fs.enter(line);
        fs.leave();
    } else {
        let v = line.as_string().split(' ');
        fs.set_file(v[1], v[0].parse_int());
    }
}

let part1 = 0;

let total_space = 70000000;
let free_space = total_space - fs.dirs[""].size;
let need_space = 30000000 - free_space;
let part2 = total_space;

for dir in fs.dirs.values() {
    if dir.size <= 100000 {
        part1 += dir.size;
    }

    if dir.size < part2 && dir.size >= need_space {
        part2 = dir.size;
    }
}

return [
    part1.to_string(),
    part2.to_string(),
]