#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Ordering};
use std::error::Error;
use log::debug;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    #[serde(rename = "Patient ID")]
    pub patient_id: String,
    #[serde(rename = "GTV_phase_1")]
    pub gtv_phase_i: Option<f64>,
    #[serde(rename = "GTV_phase_2")]
    pub gtv_phase_ii: Option<f64>,
    #[serde(rename = "GTV_phase_3")]
    pub gtv_phase_iii: Option<f64>,
    #[serde(rename = "GTV_N_phase_1")]
    pub gtv_n_phase_i: Option<f64>,
    #[serde(rename = "GTV_N_phase_2")]
    pub gtv_n_phase_ii: Option<f64>,
    #[serde(rename = "GTV_N_phase_3")]
    pub gtv_n_phase_iii: Option<f64>,
    #[serde(rename = "PTV_DP_phase_1")]
    pub ptv_dp_phase_i: Option<f64>,
    #[serde(rename = "PTV_DP_phase_2")]
    pub ptv_dp_phase_ii: Option<f64>,
    #[serde(rename = "PTV_DP_phase_3")]
    pub ptv_dp_phase_iii: Option<f64>,
}

/// Compute the average and standard deviation of the difference between of vectors (v1-v2).
///
/// # Arguments
///
/// * `v1` - vector with optional values
/// * `v2` - vector with optional values
///
/// Both vectors should have the same length, otherwise an error is returned.
/// Returns a tuple of (average, standard deviation, number of actual values used for the
/// calculation).
fn avg_std_dev_from_vectors(
    v1: &Vec<Option<f64>>,
    v2: &Vec<Option<f64>>,
) -> Result<(f64, f64, usize), Box<dyn Error>> {
    let n = v1.len();
    let m = v2.len();
    if n != m {
        return Err(format!(
            "Expected the same number of entries in v1 [{}] and v2 [{}].",
            n, m
        )
            .into());
    }

    let v: Vec<_> = v1.iter()
        .zip(v2.iter())
        .filter_map(|(o1, o2)| {
            if o1.is_some() && o2.is_some() {
                Some(o1.unwrap() - o2.unwrap())
            } else {
                None
            }
        })
        .filter(|x| !x.is_nan())
        .collect();
    let n = v.len() as f64;
    if n != m as f64 {
        debug!("n: {}", n);
        debug!("m: {}", m);
        debug!("v: {:#?}", v);
    }
    let avg = v.iter().sum::<f64>() / n;
    let std_dev = (v.iter()
        .map(|d| f64::powf(d - avg, 2.0))
        .sum::<f64>() / (n - 1.0)
    ).sqrt();
    Ok((avg, std_dev, n as usize))
}

/// Volumes for phase 1, 2 and 3 for a ROI.
#[derive(Clone, Debug, Default)]
pub struct Data {
    pub roi_name: String,
    vol_phase_1: Vec<Option<f64>>,
    vol_phase_2: Vec<Option<f64>>,
    vol_phase_3: Vec<Option<f64>>,
}

impl Data {
    /// Create a new ROI with a name.
    ///
    /// # Arguments
    ///
    /// * `name` - ROI name
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            roi_name: name.as_ref().to_string(),
            vol_phase_1: Default::default(),
            vol_phase_2: Default::default(),
            vol_phase_3: Default::default(),
        }
    }
    /// Add volumes per phase.
    ///
    /// Volumes are discarded if one of the input arguments is None or it's value is a NaN.
    ///
    /// # Arguments
    ///
    /// * `v1` - volume phase I
    /// * `v2` - volume phase II
    /// * `v3` - volume phase III
    pub fn add_vol(&mut self, v1: Option<f64>, v2: Option<f64>, v3: Option<f64>) {
        let v1 = v1.unwrap_or(f64::NAN);
        let v2 = v2.unwrap_or(f64::NAN);
        let v3 = v3.unwrap_or(f64::NAN);
        if v1.is_nan() || v2.is_nan() || v3.is_nan() {
            return;
        }
        self.vol_phase_1.push(Some(v1) );
        self.vol_phase_2.push(Some(v2) );
        self.vol_phase_3.push(Some(v3) );
    }

    /// Clear all the volumes in the different phases.
    pub fn clear(&mut self) {
        self.vol_phase_1.clear();
        self.vol_phase_2.clear();
        self.vol_phase_3.clear();
    }

    pub fn phase_1_to_2_stat(&self) -> Result<Stat, Box<dyn Error>> {
        let avg_vol = self.vol_phase_1.iter().map(|x| x.unwrap_or(f64::NAN)).sum::<f64>() / self.vol_phase_1.len() as f64;
        let (avg, std_dev, n) = avg_std_dev_from_vectors(&self.vol_phase_1, &self.vol_phase_2)?;
        Ok(Stat {
            roi_name: self.roi_name.clone(),
            avg_vol_phase_start: avg_vol,
            phase_start: 1,
            phase_end: 2,
            avg,
            std_dev,
            n,
        })
    }
    pub fn phase_2_to_3_stat(&self) -> Result<Stat, Box<dyn Error>> {
        let avg_vol = self.vol_phase_2.iter().map(|x| x.unwrap_or(f64::NAN)).sum::<f64>() / self.vol_phase_1.len() as f64;
        let (avg, std_dev, n) = avg_std_dev_from_vectors(&self.vol_phase_2, &self.vol_phase_3)?;
        Ok(Stat {
            roi_name: self.roi_name.clone(),
            avg_vol_phase_start: avg_vol,
            phase_start: 2,
            phase_end: 3,
            avg,
            std_dev,
            n,
        })
    }
}

pub fn read_csv(filename: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_path(filename)?;
    let mut v = vec![];
    for result in rdr.deserialize() {
        let record = result?;
        v.push(record);
    }
    Ok(v)
}

pub fn records_to_data(records: &Vec<Record>) -> Vec<Data> {
    let mut gtv = Data::new("GTV");
    let mut gtv_n = Data::new("GTV_N");
    let mut ptv_dp = Data::new("PTV_DP");
    for record in records {
        gtv.add_vol(
            record.gtv_phase_i,
            record.gtv_phase_ii,
            record.gtv_phase_iii,
        );
        gtv_n.add_vol(
            record.gtv_n_phase_i,
            record.gtv_n_phase_ii,
            record.gtv_n_phase_iii,
        );
        ptv_dp.add_vol(
            record.ptv_dp_phase_i,
            record.ptv_dp_phase_ii,
            record.ptv_dp_phase_iii,
        );
    }
    vec![gtv, gtv_n, ptv_dp]
}

/// Stores statical data (average and standard deviation) per ROI and phase change.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Stat {
    /// Name of the ROI
    #[serde(rename = "ROI")]
    pub roi_name: String,
    /// Average volume at the phase when the initial data was acquired.
    #[serde(rename = "Volume Phase start")]
    pub avg_vol_phase_start: f64,
    /// Phase at which the initial data was acquired.
    #[serde(rename = "Phase start")]
    pub phase_start: i32,
    /// Phase at which the last data was acquired.
    #[serde(rename = "Phase end")]
    pub phase_end: i32,
    /// Average volume difference
    #[serde(rename = "average")]
    pub avg: f64,
    /// Standard deviation of the volume differences
    #[serde(rename = "std_dev")]
    pub std_dev: f64,
    /// Number of data points from which the data was computed.
    #[serde(rename = "n")]
    pub n: usize,
}

impl PartialEq for Stat {
    fn eq(&self, other: &Self) -> bool {
        self.roi_name == other.roi_name
            && self.phase_start == other.phase_start
            && self.phase_end == other.phase_end
    }
}

impl Eq for Stat {}

impl Ord for Stat {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.roi_name.cmp(&other.roi_name);
        if o != Ordering::Equal {
            return o;
        }
        let o = self.phase_start.cmp(&other.phase_start);
        if o != Ordering::Equal {
            return o;
        }
        self.phase_end.cmp(&other.phase_end)
    }
}

impl PartialOrd for Stat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn add_data(stats: &mut Vec<Stat>, data: &Data) -> Result<(), Box<dyn Error>> {
    stats.push(data.phase_1_to_2_stat()?);
    stats.push(data.phase_2_to_3_stat()?);
    Ok(())
}

pub fn dataset_to_stats(dataset: &Vec<Data>) -> Result<Vec<Stat>, Box<dyn Error>> {
    let mut v = vec![];
    for data in dataset {
        add_data(&mut v, data)?;
    }
    v.sort();
    Ok(v)
}
