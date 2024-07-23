use rand::Rng;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Write};

const L: usize = 20; // Size of the lattice

// Function to generate a percolation lattice with given probability p
fn generate_percolation_lattice(p: f64) -> Vec<Vec<Vec<bool>>> {
    let mut rng = rand::thread_rng();
    let mut lattice = vec![vec![vec![false; L]; L]; L];

    for x in 0..L {
        for y in 0..L {
            for z in 0..L {
                lattice[x][y][z] = rng.gen::<f64>() < p;
            }
        }
    }

    lattice
}

// Function to write the lattice data to a VTK file
fn write_vtk(lattice: &Vec<Vec<Vec<bool>>>, filename: &str, occupied: bool) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "# vtk DataFile Version 3.0")?;
    writeln!(file, "3D percolation lattice")?;
    writeln!(file, "ASCII")?;
    writeln!(file, "DATASET UNSTRUCTURED_GRID")?;

    let mut points = vec![];
    let mut cells = vec![];

    let mut point_index = 0;
    for x in 0..L {
        for y in 0..L {
            for z in 0..L {
                if lattice[x][y][z] == occupied {
                    // Define the 8 vertices of the cube
                    let base_point = point_index * 8;
                    points.push([x as f32, y as f32, z as f32]);
                    points.push([x as f32 + 1.0, y as f32, z as f32]);
                    points.push([x as f32 + 1.0, y as f32 + 1.0, z as f32]);
                    points.push([x as f32, y as f32 + 1.0, z as f32]);
                    points.push([x as f32, y as f32, z as f32 + 1.0]);
                    points.push([x as f32 + 1.0, y as f32, z as f32 + 1.0]);
                    points.push([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0]);
                    points.push([x as f32, y as f32 + 1.0, z as f32 + 1.0]);

                    // Define the connectivity of the cube
                    cells.push(vec![
                        8,
                        base_point + 0,
                        base_point + 1,
                        base_point + 2,
                        base_point + 3,
                        base_point + 4,
                        base_point + 5,
                        base_point + 6,
                        base_point + 7,
                    ]);

                    point_index += 1;
                }
            }
        }
    }

    writeln!(file, "POINTS {} float", points.len())?;
    for point in &points {
        writeln!(file, "{} {} {}", point[0], point[1], point[2])?;
    }

    writeln!(file, "CELLS {} {}", cells.len(), cells.len() * 9)?;
    for cell in &cells {
        for &val in cell {
            write!(file, "{} ", val)?;
        }
        writeln!(file)?;
    }

    writeln!(file, "CELL_TYPES {}", cells.len())?;
    for _ in &cells {
        writeln!(file, "12")?; // VTK_HEXAHEDRON type
    }

    Ok(())
}

// Directions for moving in the 3D lattice
const DIRECTIONS: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

// Function to check if a coordinate is within bounds
fn is_in_bounds(x: isize, y: isize, z: isize) -> bool {
    x >= 0 && x < L as isize && y >= 0 && y < L as isize && z >= 0 && z < L as isize
}

// Function to find the largest connected component using BFS
fn find_largest_connected_component(lattice: &Vec<Vec<Vec<bool>>>, occupied: bool) -> Vec<(usize, usize, usize)> {
    let mut visited = vec![vec![vec![false; L]; L]; L];
    let mut largest_component = vec![];

    for x in 0..L {
        for y in 0..L {
            for z in 0..L {
                if lattice[x][y][z] == occupied && !visited[x][y][z] {
                    let mut component = vec![];
                    let mut queue = VecDeque::new();
                    queue.push_back((x, y, z));
                    visited[x][y][z] = true;

                    while let Some((cx, cy, cz)) = queue.pop_front() {
                        component.push((cx, cy, cz));

                        for &(dx, dy, dz) in &DIRECTIONS {
                            let nx = cx as isize + dx;
                            let ny = cy as isize + dy;
                            let nz = cz as isize + dz;

                            if is_in_bounds(nx, ny, nz) {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                let nz = nz as usize;

                                if lattice[nx][ny][nz] == occupied && !visited[nx][ny][nz] {
                                    queue.push_back((nx, ny, nz));
                                    visited[nx][ny][nz] = true;
                                }
                            }
                        }
                    }

                    if component.len() > largest_component.len() {
                        largest_component = component;
                    }
                }
            }
        }
    }

    largest_component
}

// Function to write the largest connected component to a VTK file
fn write_component_vtk(component: &Vec<(usize, usize, usize)>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "# vtk DataFile Version 3.0")?;
    writeln!(file, "Largest connected component")?;
    writeln!(file, "ASCII")?;
    writeln!(file, "DATASET UNSTRUCTURED_GRID")?;

    let mut points = vec![];
    let mut cells = vec![];

    let mut point_index = 0;
    for &(x, y, z) in component {
        // Define the 8 vertices of the cube
        let base_point = point_index * 8;
        points.push([x as f32, y as f32, z as f32]);
        points.push([x as f32 + 1.0, y as f32, z as f32]);
        points.push([x as f32 + 1.0, y as f32 + 1.0, z as f32]);
        points.push([x as f32, y as f32 + 1.0, z as f32]);
        points.push([x as f32, y as f32, z as f32 + 1.0]);
        points.push([x as f32 + 1.0, y as f32, z as f32 + 1.0]);
        points.push([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0]);
        points.push([x as f32, y as f32 + 1.0, z as f32 + 1.0]);

        // Define the connectivity of the cube
        cells.push(vec![
            8,
            base_point + 0,
            base_point + 1,
            base_point + 2,
            base_point + 3,
            base_point + 4,
            base_point + 5,
            base_point + 6,
            base_point + 7,
        ]);

        point_index += 1;
    }

    writeln!(file, "POINTS {} float", points.len())?;
    for point in &points {
        writeln!(file, "{} {} {}", point[0], point[1], point[2])?;
    }

    writeln!(file, "CELLS {} {}", cells.len(), cells.len() * 9)?;
    for cell in &cells {
        for &val in cell {
            write!(file, "{} ", val)?;
        }
        writeln!(file)?;
    }

    writeln!(file, "CELL_TYPES {}", cells.len())?;
    for _ in &cells {
        writeln!(file, "12")?; // VTK_HEXAHEDRON type
    }

    Ok(())
}

fn main() {
    let p = 0.15; // Probability of a cube being occupied
    let lattice = generate_percolation_lattice(p);

    // Write the original lattice VTK files
    if let Err(e) = write_vtk(&lattice, "occupied_cells.vtk", true) {
        eprintln!("Error writing occupied cells VTK file: {}", e);
    }
    if let Err(e) = write_vtk(&lattice, "empty_cells.vtk", false) {
        eprintln!("Error writing empty cells VTK file: {}", e);
    }

    // Find the largest connected component for occupied cells
    let largest_component = find_largest_connected_component(&lattice, true);
    if let Err(e) = write_component_vtk(&largest_component, "largest_occupied_component.vtk") {
        eprintln!("Error writing largest occupied component VTK file: {}", e);
    }

    // Find the largest connected component for empty cells
    let largest_component = find_largest_connected_component(&lattice, false);
    if let Err(e) = write_component_vtk(&largest_component, "largest_empty_component.vtk") {
        eprintln!("Error writing largest empty component VTK file: {}", e);
    }
}

