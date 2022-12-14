//
// Load the height map
//
let height_map;
let start = [];
let org_start;
let end;

for row in data.blobs() {
    if height_map == () {
        height_map = dynamic_image(row.len(), 0);
    }
    height_map.push_row(row);
}

// Find the start and end + normalize all the pixels
for pix in height_map.pixels() {
    if pix.as_int() == 83 { // S
        pix.set(97); // 'a'
        org_start = pix;
        start.push(pix);
    } else if pix.as_int() == 69 { // E
        pix.set(122); // 'z'
        end = pix;
    } else if pix.as_int() == 97 { // a
        start.push(pix);
    }

    pix -= 97; // Subtract 'a' to make it range from 0 to 26
}

//
// Initialize the route map by setting every pixel to u32::MAX
//

let route_map = dynamic_image(height_map.width, height_map.height);
for pix in route_map.pixels() {
    pix.set(0xffff_ffff);
}
route_map.pixel(end.pos).set(0);

//
// Flood fill the route map by setting each pixel to the lowest number of steps
// starting at the end and moving towards the start, one step at the time.
//
// Note: Apparantly it is fine to jump down more than one unit of height.
//

let pending = [];
let next_pending = [end];
let steps = 1;

while !next_pending.is_empty() {
    pending = next_pending;
    next_pending = [];

    for cur in pending {
        for pix in cur.ortho_pixels() {
            let route_pix = route_map.pixel(pix.pos);
            if route_pix.as_int() == 0xffff_ffff {
                let d = cur.as_int() - pix.as_int();
                if d <= 1 {
                    route_pix.set(steps);
                    next_pending.push(pix);
                }
            }
        }
    }

    steps += 1;
}

return [
    route_map.pixel(org_start.pos).as_int().to_string(),
    start.map(|s| route_map.pixel(s.pos).as_int()).min().to_string(),
]
