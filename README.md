# C-ART 2 volume changes

A command line utility to compute the volume changes between the consecutive treatment phases.

## Usage
```shell
Usage: c_art_2_volume_changes.exe [OPTIONS] --file <FILE>                                                 
                                                                                                          
Options:                                                                                                  
  -f, --file <FILE>        CSV input file [semicolon delimited]                                           
  -r, --results <RESULTS>  JSON file where the results are written to [default: volume_changes_stats.json]
  -h, --help               Print help                                                                     
  -V, --version            Print version 
```

### Input
Input data is a CSV file (values delimited by a semicolon `;`) with the following header items:

* **Patient ID**: unique identifier
* **GTV_phase_1**: volume of the GTV in phase 1
* **GTV_phase_2**: volume of the GTV in phase 2
* **GTV_phase_3**: volume of the GTV in phase 3
* **GTV_N_phase_1**: volume of the GTV_N in phase 1
* **GTV_N_phase_2**: volume of the GTV_N in phase 2
* **GTV_N_phase_3**: volume of the GTV_N in phase 3
* **PTV_DP_phase_1**: volume of the PTV_DP in phase 1
* **PTV_DP_phase_2**: volume of the PTV_DP in phase 2
* **PTV_DP_phase_3**: volume of the PTV_DP in phase 3

### Output
Output data is a JSON file:
```json
[
  {
    "ROI": "GTV",
    "Volume Phase start": 20.23,
    "Phase start": 1,
    "Phase end": 2,
    "average": 4.475,
    "std_dev": 6.207,
    "n": 20
  },
  ...
]
```
* **ROI**: name of the Region Of Interest
* **Volume Phase start**: ROI volume at treatment phase start
* **Phase start**: volume difference is compute between phase start - phase end
* **Phase end**: volume difference is compute between phase start - phase end
* **average**: average volume difference between the phases
* **std_dev**: corrected sample standard deviation: 
  $\sqrt( \frac{1}{N-1} \sum_{i=1}^{N} (x_{i}-\bar{x})^2 )$ 
* **n**: number of values used to compute the average and standard deviation

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.