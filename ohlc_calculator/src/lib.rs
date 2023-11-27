
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug)]
pub struct RollingOHLC {
    window_size: usize,
    data: HashMap<String, VecDeque<(u64, f64, f64)>>,
    data_for_max: HashMap<String, VecDeque<(u64, f64, f64)>>,
    data_for_min: HashMap<String, VecDeque<(u64, f64, f64)>>,
}

// data stores maintains a deque for current window logs with valid timestamp (ex within 5 min) for each symbol
// data_for_max maintains a deque for instant retrival of high (~O(1)) for each symbol, after updation front of the queue will always have high value
// data_for_min maintains a deque for instant retrival of low (~O(1)) for each symbol, after updation front of the queue will always have low value

impl RollingOHLC {
    pub fn new(window_size: usize) -> Self {
        RollingOHLC {
            window_size,
            data: HashMap::new(),
            data_for_max: HashMap::new(),
            data_for_min: HashMap::new(),
        }
    }

    pub fn update(&mut self, symbol: &str, timestamp: u64, bid: f64, ask: f64) -> OHLC {
        let window_data = self.data.entry(symbol.to_string()).or_insert(VecDeque::new());
        window_data.push_back((timestamp, bid, ask));

        let max_dq = self.data_for_max.entry(symbol.to_string()).or_insert(VecDeque::new());
        let min_dq = self.data_for_min.entry(symbol.to_string()).or_insert(VecDeque::new());

        let cutoff_timestamp = timestamp - (self.window_size as u64 * 60 * 1000);

        // Remove outdated entry from all 3 dq
        Self::remove_outdated_entry(cutoff_timestamp, window_data);
        Self::remove_outdated_entry(cutoff_timestamp, max_dq);
        Self::remove_outdated_entry(cutoff_timestamp, min_dq);

        // add entry to max_dq
        while let Some(back_element) = max_dq.back() {
            if ((back_element.1 + back_element.2)/2.0) <= ((bid + ask)/2.0) {
                max_dq.pop_back();
            } else {
                break; 
            }
        }
        max_dq.push_back((timestamp, bid, ask));

        // add entry to min_dq
        while let Some(back_element) = min_dq.back() {
            if ((back_element.1 + back_element.2)/2.0) >= ((bid + ask)/2.0) {
                min_dq.pop_back();
            } else {
                break; 
            }
        }
        min_dq.push_back((timestamp, bid, ask));

        // Calculate OHLC
        let ohlc = Self::calculate_ohlc(window_data, max_dq, min_dq); // Use Self:: to refer to the struct's own method
        
        return ohlc;
    }

    fn remove_outdated_entry(cutoff_timestamp: u64, dq:&mut VecDeque<(u64, f64, f64)>){
        while let Some(front_element) = dq.front() {
            if front_element.0 < cutoff_timestamp {
                dq.pop_front();
            } else {
                break; // Stop the loop if the front element is not older than cutoff_timestamp
            }
        }
    }

    fn calculate_ohlc(window_data:&VecDeque<(u64, f64, f64)>, max_dq:&VecDeque<(u64, f64, f64)>, min_dq:&VecDeque<(u64, f64, f64)>) -> OHLC {
        if let (Some(first_data), Some(max_data), Some(min_data), Some(last_data)) 
        = (window_data.front().cloned(), max_dq.front().cloned(), min_dq.front().cloned(), window_data.back().cloned()) {
            let ohlc = OHLC {
                open: (first_data.1 + first_data.2) / 2.0,
                high: (max_data.1 + max_data.2) / 2.0,
                low: (min_data.1 + min_data.2) / 2.0,
                close: (last_data.1 + last_data.2) / 2.0,
            };

           return ohlc;
        } else {
            OHLC {
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
            }
        }
    }
}
