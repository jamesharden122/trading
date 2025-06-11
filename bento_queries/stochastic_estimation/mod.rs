use crate::listtobin::vectype::VecType;
use chrono::{DateTime, Utc};
use fnv::FnvHashMap;
use nalgebra::base::VecStorage;
use nalgebra::DMatrix;
use std::collections::BTreeMap;
use ndarray::Array1;


#[derive(Debug, PartialEq, Clone)]
pub struct MyMatrix {
    pub colnames: Vec<String>,
    pub data: DMatrix<f64>,
    pub descrips: (f64, f64, f64, f64, f64, f64),
}

impl MyMatrix {
    // Constructor to create a new empty matrix
    pub fn new(rows: usize, cols: usize) -> Self {
        MyMatrix {
            colnames: Vec::new(),
            data: DMatrix::zeros(rows, cols),
            descrips: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn new10x() -> Self {
        MyMatrix {
            colnames: Vec::new(),
            data: DMatrix::zeros(10, 10),
            descrips: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn from(matrix: DMatrix<f64>, colnames: Vec<String>) -> Self {
        MyMatrix {
            colnames,
            data: matrix,
            descrips: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }
    // Example method to add a scalar value to all elements of the matrix
    pub fn from_hashmap(map: FnvHashMap<String, VecType>) -> Self {
        let mut temp_slice: Vec<f64> = Vec::new();
        // Iterate over keys in the map
        for nm in map.keys() {
            // Extend temp_slice with the elements mapped to f64
            if let Some(values) = map.get(nm) {
                temp_slice.extend(
                    values.clone().into_vec_f64(), // Use `as f64` for conversion
                );
            }
        }

        // Calculate dimensions
        let cols = map.keys().len();
        let rows = if let Some(first_key) = map.keys().next() {
            map.get(first_key).unwrap().clone().into_vec_f64().len()
        } else {
            0
        };

        // Build the matrix
        let matrix = DMatrix::from_vec_storage(VecStorage::new(
            nalgebra::Dyn(rows),
            nalgebra::Dyn(cols),
            temp_slice,
        ));

        Self {
            colnames: map.keys().cloned().collect(),
            data: matrix,
            descrips: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn to_fnv_hashmap(&self) -> Option<FnvHashMap<String, VecType>> {
        let mut map = FnvHashMap::default();

        // Ensure colnames and columns match
        assert_eq!(
            self.colnames.len(),
            self.data.ncols(),
            "colnames length does not match matrix column count"
        );

        // Insert each column under its name
        for (i, colname) in self.colnames.iter().enumerate() {
            let column_vec: Vec<f64> = self.data.column(i).iter().copied().collect();
            map.insert(colname.clone(), VecType::F64Vec(column_vec));
        }
        Some(map)
    }

    pub fn add_scalar(&mut self, value: f64) {
        self.data.iter_mut().for_each(|x| *x += value);
    }

    // Method to print the dimensions of the matrix
    pub fn dimensions(&self) {
        println!(
            "Matrix has {} rows and {} columns",
            self.data.nrows(),
            self.data.ncols()
        );
    }

    // Method to get a reference to the internal DMatrix
    pub fn inner_ref(&self) -> &DMatrix<f64> {
        &self.data
    }

    pub fn head(&self, shape: (usize, usize)) {
        let (num_rows, num_cols) = shape;

        // Get a view on the matrix
        let view = self.data.view((0, 0), (num_rows, num_cols));

        // Print the submatrix
        println!("Matrix View:\n{}", view);
    }

    pub fn scale_column(mut self, constant: f64, col_name: String) -> Option<Self> {
        let col = self.colnames.iter().position(|s| *s == col_name);
        //let mut data = self.data.clone();
        self.data
            .column_mut(col?)
            .iter_mut()
            .for_each(|x| *x *= constant);
        //let dt = task::spawn_blocking(move || { data // Return the modified data  }).await;
        //self.data = dt.unwrap();
        Some(self)
    }

    pub fn snapshot(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        let col_means = self.data.row_mean();
        let standard_deviations = self.data.row_variance();
        Some((standard_deviations[4], col_means[4], 0.0, 0.0, 0.0, 0.0))
    }

    pub fn append_retuns(&self, quotient: bool) -> Option<DMatrix<f64>> {
        // Create a new column for storing the returns with an initial NaN value for the first row
        let matrix = &self.data.clone();
        let rows = matrix.shape().0;
        let cols = matrix.shape().1;
        let mut returns_column = vec![0.0; rows];
        // Calculate the first difference and returns for the first column
        for i in 1..rows {
            let previous_value = matrix[(i - 1, 0)];
            let current_value = matrix[(i, 0)];
            let first_diff = current_value - previous_value;

            if previous_value != 0.0 && quotient {
                returns_column[i] = first_diff / previous_value;
            } else if previous_value != 0.0 && !quotient {
                returns_column[i] = current_value / previous_value
            }
        }

        // Create a new matrix with 5 columns to accommodate the original data plus the returns column
        let mut extended_matrix = DMatrix::from_element(rows, cols + 1, 0.0);

        // Copy the original matrix data into the new extended matrix
        for r in 0..rows {
            for c in 0..cols {
                extended_matrix[(r, c)] = matrix[(r, c)];
            }
        }

        // Add the returns column as the fifth column in the new matrix
        for r in 0..rows {
            extended_matrix[(r, cols)] = returns_column[r];
        }
        Some(extended_matrix)
    }

    pub fn vec_retuns(&self, quotient: bool) -> Option<Vec<f64>> {
        // Create a new column for storing the returns with an initial NaN value for the first row
        let matrix = &self.data.clone();
        let rows = matrix.shape().0;
        let cols = matrix.shape().1;
        let mut returns_column = vec![0.0; rows];
        // Calculate the first difference and returns for the first column
        for i in 1..rows {
            let previous_value = matrix[(i - 1, 0)];
            let current_value = matrix[(i, 0)];
            let first_diff = current_value - previous_value;

            if previous_value != 0.0 && quotient == false {
                returns_column[i] = first_diff / previous_value;
            } else if previous_value != 0.0 && quotient == true {
                returns_column[i] = current_value / previous_value
            }
        }
        Some(returns_column)
    }

    pub fn convert_nano_to_datetime(ts_recv: f64) -> Option<DateTime<Utc>> {
        let seconds = (ts_recv / 1_000_000_000.0) as i64;
        let nanoseconds = (ts_recv % 1_000_000_000.0) as u32;

        // Create a NaiveDateTime from ts_recv
        let datetime_recv = DateTime::from_timestamp(seconds, nanoseconds).unwrap();
        println!("Received DateTime: {}", datetime_recv);
        Some(datetime_recv)
    }

    pub fn moments_matrix(self, num_moments: u8) -> Option<DMatrix<f64>> {
        let prc_q = self.vec_retuns(true).unwrap();
        let mut mom_mat = DMatrix::zeros(prc_q.len(), num_moments.into());
        // Raise each element by its column number
        for i in 0..mom_mat.nrows() {
            for j in 0..mom_mat.ncols() {
                mom_mat[(i, j)] = prc_q[i].powf(j as f64);
            }
        }

        Some(mom_mat)
    }

    pub fn calculate_bin_data_with_vwap_and_returns(&self, size_ind: usize, time_ind: usize, trade_value_ind: usize, bin_width_ns: u64) -> Option<Self> {
        let size_col = self.data.column(size_ind);
        let time_col = self.data.column(time_ind);
        let price_col = self.data.column(trade_value_ind);
        println!("completed the column creation");
        // Map bin_id to (total trade_value, total size, sum of price*size for VWAP)
        let mut bin_data: BTreeMap<u64, (f64, f64, f64)> = BTreeMap::new();
        for (&time_ns, (&price, &size)) in time_col.iter().zip(price_col.iter().zip(size_col.iter())) {
            let bin_id = (time_ns as u64) / bin_width_ns;
            let entry = bin_data.entry(bin_id).or_insert((0.0, 0.0, 0.0));
            entry.0 += price;       // total trade_value
            entry.1 += size;              // total size
            entry.2 += price * size;      // sum of price*size for VWAP
        }

        // Sort bins to ensure ordered output
        let mut bin_ids: Vec<u64> = bin_data.keys().cloned().collect();
        bin_ids.sort();

        // Compute VWAPs
        let mut vwaps: Vec<f64> = Vec::new();
        for &bin_id in &bin_ids {
            let (total_value, total_size, weighted_sum) = bin_data[&bin_id];
            let vwap = if total_size > 0.0 { weighted_sum / total_size } else { 0.0 };
            vwaps.push(vwap);
        }

        // Compute log returns using VWAPs
        let mut log_returns: Vec<f64> = vec![0.0]; // First row has no previous VWAP, set to 0
        for w in vwaps.windows(2) {
            let lr = (w[1] / w[0]).ln();
            log_returns.push(lr);
        }
    println!("computed returns");
    // Build the final matrix data
        let mut final_data: Vec<f64> = Vec::new();
        for (i, &bin_id) in bin_ids.iter().enumerate() {
            let (total_value, total_size, weighted_sum) = bin_data[&bin_id];
            let time_ns_floor = bin_id * bin_width_ns;
            let vwap = if total_size > 0.0 { weighted_sum / total_size } else { 0.0 };
            let log_return = log_returns[i];

            final_data.push(total_size);
            final_data.push(total_value);
            final_data.push(bin_id as f64);
            final_data.push(time_ns_floor as f64);
            final_data.push(vwap);
            final_data.push(log_return);
        }

        // Reshape into a matrix: rows = number of bins, 6 columns
        let col_names: Vec<String> = vec![
            String::from("total size"),
            String::from("total value"),
            String::from("bin id"),
            String::from("time ns"),
            String::from("value weighted average price"),
            String::from("Log Returns"),
        ];
        let num_bins = bin_ids.len();
        let data = DMatrix::from_row_slice(num_bins,6,&final_data);
        let arr: Array1<f64> = Array1::from_iter(data.column(5).iter().cloned());
        // Now you can use ndarray functions
        let mean = arr.mean().unwrap();
        let var = arr.var(0.0);
        
       Some( Self {
            colnames: col_names,
            data: data,
            descrips: (mean,var,0.0,0.0,0.0,0.0)
        })
    }
}
