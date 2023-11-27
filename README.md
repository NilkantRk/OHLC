# Tensorfox Homework Assignment

## Task 1 

Develop a Rust library crate that provides a way of computing rolling OHLC (open-high-low-close) for a stream of numeric 
prices and timestamps for a given time window.

E.g., if the window is 5 minutes, once a new price/timestamp is given, return the rolling 5-minute OHLC over the last 
5 minutes - the earliest price in the time period, the highest/lowest prices and the latest price.

## Task 2 

Use your crate as a dependency to provide something to run which reads JSON data in the given format from a given 
filename (see attached file for example) for multiple symbols, and outputs rolling OHLC for the current symbol that just 
ticked (the window is fixed and provided at startup, same for all symbols).

We will evaluate and grade your solution’s performance and overall quality, including crate structure and following 
rust best practices and formatting guidelines along with any tests or benchmarks you deem necessary.

## Dataset

We provide two files with ticker updates for several symbols. Each line represents a single event with bid/ask price 
and quantity and a timestamp `T` (which should be used to calculate the window). We ask you to calculate rolling OHLC 
for a 5 minutes window. For `data/dataset-a.txt` we additionally provide the expected output. You can use it to test
your solution. We expect you to follow the same output format for `data/dataset-b.txt`. We will test your solution 
using `data/dataset-b.txt`.

## PR

To submit your project solution, we prefer that you create a branch from the main repository in order to work on your solution. 
Once you have completed your solution, please create a pull request (PR) for review. This will allow our team to easily review 
your code changes and if necessary provide feedback. Thank you for your cooperation in following these submission guidelines.
