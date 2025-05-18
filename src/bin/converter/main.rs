fn main() {
    let mut args = std::env::args();
    _ = args.next().unwrap();
    let x: i32 = args.next().unwrap().parse().unwrap();
    let z: i32 = args.next().unwrap().parse().unwrap();

    eprintln!("Coordinates: {x} / Y / {z}");
    let (region_x, region_z) = convert_coordinates_to_region(x, z);
    println!("File: r.{region_x}.{region_z}.mca");
}

fn convert_coordinates_to_chunk(coordinate_x: i32, coordinate_z: i32) -> (i32, i32) {
    let chunk_x = floor_integer_division(coordinate_x, 16);
    let chunk_z = floor_integer_division(coordinate_z, 16);
    (chunk_x, chunk_z)
}

fn floor_integer_division(a: i32, b: i32) -> i32 {
    // Rust naturally floors towards 0, but we need it to floor downwards consistently.
    assert_eq!(3 / 2, 1); // Floors towards 0; 1 is OK.
    assert_eq!(-3 / 2, -1); // Floors towards 0; we need this to be -2, not -1.

    let result = a / b;
    let remainder = a % b;

    if remainder != 0 && result < 0 {
        result - 1
    } else {
        result
    }
}

fn convert_chunk_to_region(chunk_x: i32, chunk_z: i32) -> (i32, i32) {
    let region_x = floor_integer_division(chunk_x, 32);
    let region_z = floor_integer_division(chunk_z, 32);
    (region_x, region_z)
}

fn convert_coordinates_to_region(coordinate_x: i32, coordinate_z: i32) -> (i32, i32) {
    let (chunk_x, chunk_z) = convert_coordinates_to_chunk(coordinate_x, coordinate_z);
    eprintln!("Chunk: X: {chunk_x} / Z: {chunk_z}");
    let (region_x, region_z) = convert_chunk_to_region(chunk_x, chunk_z);
    eprintln!("Region: X: {region_x} / Z: {region_z}");
    (region_x, region_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer_division() {
        assert_eq!(floor_integer_division(3, 2), 1);
    }

    #[test]
    fn test_negative_integer_division() {
        assert_eq!(floor_integer_division(-3, 2), -2);
    }
}
