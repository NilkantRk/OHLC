
use ohlc_calculator::RollingOHLC;
// use serde_json::{self, Value};
use serde_json::{self};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};


#[derive(Default, Serialize)]
struct OutputData {
    symbol: String,
    timestamp: u64,
    open: String,
    high: String,
    low: String,
    close: String,
}


fn main() {
    // Update the file paths to point to the correct location
    let input_filename = "../data/dataset-b.txt"; // Change this to your file path
    let output_filename = "../data/output.txt"; // Change this to your desired output file path

    // Read input JSON data from the specified file
    if let Ok(input_file) = File::open(input_filename) {
        let input_reader = io::BufReader::new(input_file);

        // Initialize RollingOHLC with a window size of 5 minutes
        // Here window size can be changed as per requirement, window size takes input in minuits
        let mut ohlc_calculator = RollingOHLC::new(5);

        // Create or truncate the output file
        let output_file = File::create(output_filename).expect("Unable to create output file");
        let mut output_writer = BufWriter::new(output_file);

        // Process each line in the input file
        for line in input_reader.lines() {
            if let Ok(contents) = line {
                // Parse JSON data for each line
                let price_data: serde_json::Value =
                    serde_json::from_str(&contents).unwrap_or_else(|err| {
                        eprintln!("Error parsing JSON: {}", err);
                        serde_json::Value::Object(Default::default())
                    });

                // Extract data and calculate OHLC
                let symbol = price_data["s"].as_str().unwrap_or_default();
                let timestamp = price_data["T"].as_u64().unwrap_or_default();
                let bid = price_data["b"].as_str().unwrap_or_default().parse::<f64>().unwrap_or_default();
                let ask = price_data["a"].as_str().unwrap_or_default().parse::<f64>().unwrap_or_default();
                let ohlc = ohlc_calculator.update(symbol, timestamp, bid, ask);

                // Create OutputData struct
                let output_data = OutputData {
                    symbol: symbol.to_string(),
                    timestamp,
                    open: format!("{:.6}", ohlc.open),
                    high: format!("{:.6}", ohlc.high),
                    low: format!("{:.6}", ohlc.low),
                    close: format!("{:.6}", ohlc.close),
                };

                // Serialize OutputData to JSON and write to the output file
                let output_json =
                    serde_json::to_string(&output_data).expect("Error serializing to JSON");
                writeln!(output_writer, "{}", output_json).expect("Error writing to output file");
            }
        }
        // to check if my output and expected output is same
        // check_if_same();

    } else {
        eprintln!("Error opening file: {}", input_filename);
    }
}







// debug code down
/*
fn read_file_contents(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}

fn parse_json_line(line: &str) -> io::Result<Value> {
    serde_json::from_str(line).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn compare_json_values(json1: &Value, json2: &Value) -> bool {
    let symbol1 = json1["symbol"].as_str();
    let timestamp1 = json1["timestamp"].as_u64();
    let open1 = json1["open"].as_str().and_then(|s| s.parse::<f64>().ok());
    let high1 = json1["high"].as_str().and_then(|s| s.parse::<f64>().ok());
    let low1 = json1["low"].as_str().and_then(|s| s.parse::<f64>().ok());
    let close1 = json1["close"].as_str().and_then(|s| s.parse::<f64>().ok());

    let symbol2 = json2["symbol"].as_str();
    let timestamp2 = json2["timestamp"].as_u64();
    let open2 = json2["open"].as_str().and_then(|s| s.parse::<f64>().ok());
    let high2 = json2["high"].as_str().and_then(|s| s.parse::<f64>().ok());
    let low2 = json2["low"].as_str().and_then(|s| s.parse::<f64>().ok());
    let close2 = json2["close"].as_str().and_then(|s| s.parse::<f64>().ok());

    symbol1 == symbol2
        && timestamp1 == timestamp2
        && open1 == open2
        && high1 == high2
        && low1 == low2
        && close1 == close2
}

fn check_if_same()-> io::Result<()> {
    
    let file1_path = "../data/ohlc-5m-b.txt";
    let file2_path = "../data/outputTest.txt";

    let lines1 = read_file_contents(file1_path)?;
    let lines2 = read_file_contents(file2_path)?;

    // Check if the number of lines is the same
    if lines1.len() != lines2.len() {
        println!("The number of lines in the files is different. {} {}", lines1.len(), lines2.len());
        return Ok(());
    }

    // Compare individual values in each line
    for (line1, line2) in lines1.iter().zip(lines2.iter()) {
        let json1 = parse_json_line(line1)?;
        let json2 = parse_json_line(line2)?;

        // Compare individual values
        if !compare_json_values(&json1, &json2) {
            println!("Lines are different:\n{}\n{}", line1, line2);
        }
    }
    println!("smooth af");
    Ok(())
}
*/
// debug code up
