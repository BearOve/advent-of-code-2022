// For each pixel in the iterator see if it is visible since the start and update the count_map
fn update_counts_for_row(count_map, it) {
    let ret = 0;
    let val = 0;
    for pix in it {
        let pix_val = pix.as_int();
        if pix_val > val {
            val = pix_val;
            if count_map.pixel(pix.position()).set(1) == 0 {
                ret += 1;
            }
        }
    }
    ret
}

// Update the count map twice for each row or column returned by the iterator. One for
// each direction
fn update_counts(count_map, it) {
    let ret = 0;
    for row in it {
        ret += update_counts_for_row(count_map, row.iter());
        ret += update_counts_for_row(count_map, row.rev_iter());
    }
    ret
}

// Count the number of visible trees in a given direction
fn count_trees(base, it) {
    let ret = 0;
    for tree in it {
        ret += 1;
        if tree.as_int() >= base {
            break;
        }
    }
    ret
}


let forest;
let part1 = 0;
let part2 = 0;

//
// Load the forest map
//

for row in data.blobs() {
    if forest == () {
        forest = dynamic_image(row.len(), 0);
    }
    forest.push_row(row);
}

//
// Get the counts for part1
//

let count_map = dynamic_image(forest.width, forest.height);

part1 += update_counts(count_map, forest.rows());
part1 += update_counts(count_map, forest.cols());

//
// Get the best spot for part2
//

// Iterate the forest but skip the aprin
for spot in forest.pixels(1, 1, -1, -1) {
    let base = spot.as_int();
    let score = 1;
    score *= count_trees(base, spot.right_pixels());
    score *= count_trees(base, spot.left_pixels());
    score *= count_trees(base, spot.up_pixels());
    score *= count_trees(base, spot.down_pixels());
    if score > part2 {
        part2 = score;
        debug([score, spot]);
    }
}

//print(count_map.to_2bit_ascii_art());

return [
    part1.to_string(),
    part2.to_string(),
]