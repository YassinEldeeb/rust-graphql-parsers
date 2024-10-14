use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use urlencoding::encode;

fn main() {
    let rust_output = get_command_output("cargo", vec!["bench"], None);

    // cooldown
    thread::sleep(time::Duration::from_secs(10));

    let benchmarks = extract_benchmarks(&rust_output);
    let (micro_chart_url, milli_chart_url) = generate_quickchart_url(&benchmarks);
    let readme_content = generate_readme(
        &benchmarks,
        &micro_chart_url,
        &milli_chart_url,
        &rust_output,
    );
    write_to_file("README.md", &readme_content).expect("Failed to write README.md");
}

fn get_command_output(command: &str, args: Vec<&str>, dir: Option<&str>) -> String {
    let mut command = Command::new(command);
    command.args(args);
    if let Some(directory) = dir {
        command.current_dir(directory);
    }

    let output = command.output().expect("Failed to execute command").stdout;
    String::from_utf8_lossy(&output).into_owned()
}

// A map with the structure: {case_name: {tool_name: time}}
type Benchmarks = HashMap<String, HashMap<String, f64>>;

fn extract_benchmarks(rust_output: &str) -> Benchmarks {
    let mut benchmarks = Benchmarks::new();

    let re = Regex::new(r"([^\s]+(?: \([^\)]+\))?)\s+/\s+(\S+)\s+time:\s+\[(\d+\.\d+)\s*(µs|ms)\s+\d+\.\d+\s*(µs|ms)\s+\d+\.\d+\s*(µs|ms)\]").unwrap();

    for captures in re.captures_iter(rust_output) {
        let tool_name = captures.get(1).unwrap().as_str().to_string();
        let case_name = captures.get(2).unwrap().as_str().to_string();
        let time_str = captures.get(3).unwrap().as_str(); // Numeric time value
        let time_unit = captures.get(4).unwrap().as_str(); // Time unit (µs or ms)

        match time_str.parse::<f64>() {
            Ok(time) => {
                // Convert milliseconds to microseconds if needed
                let time_in_us = if time_unit == "ms" {
                    time * 1000.0
                } else {
                    time
                };

                benchmarks
                    .entry(case_name)
                    .or_insert_with(HashMap::new)
                    .insert(tool_name, time_in_us);
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse time '{}' for tool '{}', case '{}': {}",
                    time_str, tool_name, case_name, e
                );
            }
        }
    }

    benchmarks
}

fn generate_readme(
    benchmarks: &Benchmarks,
    microsecond_chart_url: &str,
    millisecond_chart_url: &str,
    rust_output: &str,
) -> String {
    let tool_names = vec![
        "asyync-graphql-parser",
        "graphql_query (Stellate)",
        "graphql-parser",
    ];

    let header_row = format!(
        "| Case | {} |\n|------|{}|\n",
        tool_names.join(" | "),
        tool_names
            .iter()
            .map(|_| "------")
            .collect::<Vec<&str>>()
            .join(" | ")
    );

    let mut table_rows = String::new();
    for (case_name, times) in benchmarks {
        let row = format!(
            "| {} | {} |\n",
            case_name,
            tool_names
                .iter()
                .map(|tool| format!("{}", times.get(*tool).unwrap_or(&0.0)))
                .collect::<Vec<String>>()
                .join(" | ")
        );
        table_rows.push_str(&row);
    }

    format!(
        "# GraphQL Parsers Benchmarks \n\n\
        > Note: benchmarks are ran within GitHub CI which might introduce a bit of noise.\n\n\
        {}{}\n\n\
        ![Benchmark Bar Chart (Microseconds)]({})\n\n\
        ![Benchmark Bar Chart (Milliseconds)]({})\n\n\
        <details><summary>Click to expand logs</summary>\n\n\
        Rust Benchmark Output:\n\n\
        ```shell\n\
        {}\n\
        ```\n\n\
        </details>\n",
        header_row, table_rows, microsecond_chart_url, millisecond_chart_url, rust_output
    )
}
fn write_to_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

fn generate_quickchart_url(benchmarks: &Benchmarks) -> (String, String) {
    let tool_names = vec![
        "asyync-graphql-parser",
        "graphql_query (Stellate)",
        "graphql-parser",
    ];

    let mut microsecond_datasets = Vec::new();
    let mut millisecond_datasets = Vec::new();
    let mut microsecond_labels = Vec::new();
    let mut millisecond_labels = Vec::new();

    for (case_name, times) in benchmarks {
        // Separate based on time ranges
        let is_microseconds = times.values().all(|&v| v < 1000.0); // Values under 1ms
        if is_microseconds {
            microsecond_labels.push(case_name.clone());
        } else {
            millisecond_labels.push(case_name.clone());
        }

        for (i, tool) in tool_names.iter().enumerate() {
            if is_microseconds {
                if microsecond_datasets.len() <= i {
                    microsecond_datasets.push(json!({
                        "label": tool,
                        "backgroundColor": match i {
                            0 => "rgba(255, 99, 132, 0.6)",
                            1 => "rgba(54, 162, 235, 0.6)",
                            2 => "rgba(75, 192, 192, 0.6)",
                            _ => "rgba(153, 102, 255, 0.6)",
                        },
                        "data": Vec::<serde_json::Value>::new(),
                    }));
                }
                microsecond_datasets[i]["data"]
                    .as_array_mut()
                    .unwrap()
                    .push((*times.get(*tool).unwrap_or(&0.0)).into());
            } else {
                if millisecond_datasets.len() <= i {
                    millisecond_datasets.push(json!({
                        "label": tool,
                        "backgroundColor": match i {
                            0 => "rgba(255, 99, 132, 0.6)",
                            1 => "rgba(54, 162, 235, 0.6)",
                            2 => "rgba(75, 192, 192, 0.6)",
                            _ => "rgba(153, 102, 255, 0.6)",
                        },
                        "data": Vec::<serde_json::Value>::new(),
                    }));
                }
                millisecond_datasets[i]["data"]
                    .as_array_mut()
                    .unwrap()
                    .push((*times.get(*tool).unwrap_or(&0.0)).into());
            }
        }
    }

    // Generate chart for microsecond values
    let microsecond_chart_data = json!({
        "type": "bar",
        "data": {
            "labels": microsecond_labels,
            "datasets": microsecond_datasets
        },
        "options": {
            "title": {
                "display": true,
                "text": "Benchmark Performance (Microseconds)"
            },
            "scales": {
                "yAxes": [{
                    "ticks": {
                        "beginAtZero": true
                    }
                }]
            }
        }
    });

    // Generate chart for millisecond values
    let millisecond_chart_data = json!({
        "type": "bar",
        "data": {
            "labels": millisecond_labels,
            "datasets": millisecond_datasets
        },
        "options": {
            "title": {
                "display": true,
                "text": "Benchmark Performance (Milliseconds)"
            },
            "scales": {
                "yAxes": [{
                    "ticks": {
                        "beginAtZero": true
                    }
                }]
            }
        }
    });

    let microsecond_chart_data_str = microsecond_chart_data.to_string();
    let millisecond_chart_data_str = millisecond_chart_data.to_string();

    let microsecond_chart_url = format!(
        "https://quickchart.io/chart?bkg=white&c={}",
        encode(&microsecond_chart_data_str)
    );

    let millisecond_chart_url = format!(
        "https://quickchart.io/chart?bkg=white&c={}",
        encode(&millisecond_chart_data_str)
    );

    (microsecond_chart_url, millisecond_chart_url)
}
