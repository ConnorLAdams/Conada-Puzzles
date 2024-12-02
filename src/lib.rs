use pyo3::{exceptions::PyValueError, prelude::*};
use std::collections::HashMap;
use std::usize;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// A Python class implemented in Rust.
#[pyclass]
struct Planes {
    #[pyo3(get, set)]
    passengers: u64,
    #[pyo3(get, set)]
    seats: u64,
    #[pyo3(get, set)]
    cols: Vec<char>,
}

#[pymethods]
impl Planes {
    #[new]
    #[pyo3(signature = (passengers=100, seats=100, cols="ABCDEF".to_string()))]
    fn new(passengers: Option<u64>, seats: Option<u64>, cols: Option<String>) -> PyResult<Self> {
        let p = Planes{
            passengers: passengers.expect("Expected passengers"),
            seats: seats.expect("Expected seats"), 
            cols: cols.expect("Expected cols for each row on the plane").chars().collect(),
        };
        if p.passengers > p.seats {
            Err(PyValueError::new_err("Can't have more seats than passengers."))
        } else {
            Ok(p)
        }
    }

    fn run_simulation(&self, iterations: u64) -> f64 {
        let mut num_successes: i64 = 0;
        for _ in 0..iterations {
            let seating = self.generate_seating();
            let mut open_seats = Vec::from_iter(seating.values().cloned());
            let mut seat: String = "".to_string();
            for p in 1..=self.passengers {
                if p == 1 || !open_seats.contains(seating.get(&p).unwrap()) {
                    seat = open_seats.choose(&mut thread_rng()).unwrap().to_string();
                    let index = open_seats.iter().position(|x| *x == seat).unwrap();
                    open_seats.remove(index);
                } else {
                    seat = seating.get(&p).unwrap().to_string();
                    let index = open_seats.iter().position(|x| *x == seat).unwrap();
                    open_seats.remove(index);
                }
            }
            if seating.get(&self.passengers).unwrap() == &seat {
                num_successes += 1;
            }
        }
        num_successes as f64 / iterations as f64
    }
    
    fn generate_seating(&self) -> HashMap<u64, String> {
        // Generate available seats
        let mut available_seats = Vec::new();
        for i in 0..self.seats {
            available_seats.push(format!("{}{}", (i+6 / 6), self.cols.get((i % 6) as usize).unwrap()));
        }

        // Randomize the seating
        let mut rng = thread_rng();
        available_seats.shuffle(&mut rng);

        // Assign seating
        let mut seating = HashMap::new();
        for i in 1..=self.passengers {
            seating.insert(i, available_seats.get((i-1) as usize).expect("Seat on a plane").to_string());
        }
        seating
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn puzzles(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Planes>()?;
    Ok(())
}


