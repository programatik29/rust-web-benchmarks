use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum MetricsError {
    ParseError
}

pub struct Report {
    framework_name: String,
    max_memory: String,
    metrics: Metrics,
}

const REPORT_HEADER: &str = "| Framework Name | Latency.Avg | Latency.Stdev | Latency.Min | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate | Max. Memory Usage |";
const TABLE_SEPARATOR: &str = "\n|---|---|---|---|---|---|---|---|---|---|\n";

impl Report {
    pub fn new(framework_name: &str,
               max_memory: f64,
               metrics: Metrics) -> Self {
        Self {
            framework_name: framework_name.to_string(),
            metrics,
            max_memory: format!("{:.1}MB", max_memory),
        }
    }

    pub fn generate_from(reports: &Vec<Report>) -> String {
        let mut res = String::new();

        res.push_str(REPORT_HEADER);
        res.push_str(TABLE_SEPARATOR);

        for r in reports {
            let row = format!("|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
                              r.framework_name,
                              r.metrics.latency.avg,
                              r.metrics.latency.std_env,
                              r.metrics.latency.min,
                              r.metrics.latency.max,
                              r.metrics.request.total,
                              r.metrics.request.req_per_sec,
                              r.metrics.transfer.total,
                              r.metrics.transfer.rate,
                              r.max_memory);
            res.push_str(&row);
            res.push('\n');
        }

        res.pop(); // drop last '\n'

        res
    }
}

#[derive(PartialEq, Debug)]
pub struct Metrics {
    latency: Latency,
    request: Request,
    transfer: Transfer,
}

// parse std output from rewrk result
// something like this:
//      Beginning round 1...
//      Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
//        Latencies:
//          Avg      Stdev    Min      Max
//          0.50ms   1.22ms   0.02ms   41.93ms
//        Requests:
//          Total: 30178057 Req/Sec: 1006342.33
//        Transfer:
//          Total: 3.65 GB Transfer Rate: 124.76 MB/Sec
impl FromStr for Metrics {
    type Err = MetricsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<&str>>();

        if let [_, _, _, _, latency, _, request, _, transfer, ..] = lines[..] {
            Ok(Self {
                latency: latency.parse()?,
                request: request.parse()?,
                transfer: transfer.parse()?,
            })
        } else {
            Err(MetricsError::ParseError)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Latency {
    avg: String,
    std_env: String,
    min: String,
    max: String,
}

// parse for
// 0.50ms   1.22ms   0.02ms   41.93ms
impl FromStr for Latency {
    type Err = MetricsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.trim().split_whitespace()
            .collect::<Vec<&str>>();

        if let [avg, std_env, min, max, ..] = parts[..] {
            Ok(Self {
                avg: avg.to_string(),
                std_env: std_env.to_string(),
                min: min.to_string(),
                max: max.to_string(),
            })
        } else {
            Err(MetricsError::ParseError)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Request {
    total: String,
    req_per_sec: String,
}

// parse for
// Total: 30178057 Req/Sec: 1006342.33
impl FromStr for Request {
    type Err = MetricsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.trim().split_whitespace()
            .collect::<Vec<&str>>();

        // ["Total:", total, "Req/Sec:", rps]
        if let [_, total, _, req_per_sec, ..] = parts[..] {
            Ok(Self {
                total: total.to_string(),
                req_per_sec: req_per_sec.to_string(),
            })
        } else {
            Err(MetricsError::ParseError)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Transfer {
    total: String,
    rate: String,
}

// parse for
// Total: 3.65 GB Transfer Rate: 124.76 MB/Sec
impl FromStr for Transfer {
    type Err = MetricsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.trim().split_whitespace()
            .collect::<Vec<&str>>();

        // ["Total:", total, total_unit, "Transfer", "Rate:", rate, rate_unit]
        if let [_, total, total_unit, _, _, rate, rate_unit, ..] = parts[..] {
            Ok(Self {
                total: format!("{}{}", total, total_unit),
                rate: format!("{}{}", rate, rate_unit),
            })
        } else {
            Err(MetricsError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod report {
        use super::*;

        #[test]
        fn generate() {
            let given = vec![
                Report::new("actix-web", 13.7, r#"
                     Beginning round 1...
                     Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
                       Latencies:
                         Avg      Stdev    Min      Max
                         0.50ms   1.22ms   0.02ms   41.93ms
                       Requests:
                         Total: 30178057 Req/Sec: 1006342.33
                       Transfer:
                         Total: 3.65 GB Transfer Rate: 124.76 MB/Sec
                "#.parse().expect("parse metric fail")),
                Report::new("axum", 12.4, r#"
                     Beginning round 1...
                     Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
                       Latencies:
                         Avg      Stdev    Min      Max
                         0.72ms   0.36ms   0.03ms   17.55ms
                       Requests:
                         Total: 20765149 Req/Sec: 692354.35
                       Transfer:
                         Total: 2.51 GB Transfer Rate: 85.84 MB/Sec
                "#.parse().expect("parse metric fail")),
            ];

            let actual = Report::generate_from(&given);

            let expect = r#"
| Framework Name | Latency.Avg | Latency.Stdev | Latency.Min | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate | Max. Memory Usage |
|---|---|---|---|---|---|---|---|---|---|
|actix-web|0.50ms|1.22ms|0.02ms|41.93ms|30178057|1006342.33|3.65GB|124.76MB/Sec|13.7MB|
|axum|0.72ms|0.36ms|0.03ms|17.55ms|20765149|692354.35|2.51GB|85.84MB/Sec|12.4MB|
"#.trim();

            assert_eq!(actual, expect);
        }
    }

    mod metrics {
        use super::*;

        #[test]
        fn ok() {
            let given = r#"
Beginning round 1...
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    3.56ms   0.97ms   0.11ms   114.54ms
  Requests:
    Total: 4206350 Req/Sec: 140218.97
  Transfer:
    Total: 521.41 MB Transfer Rate: 17.38 MB/Sec

691 Errors: error shutting down connection: Socket is not connected (os error 57)
            "#;
            let actual = given.parse::<Metrics>();

            let expect = Ok(
                Metrics {
                    latency: Latency {
                        avg: "0.50ms".to_string(),
                        std_env: "1.22ms".to_string(),
                        min: "0.02ms".to_string(),
                        max: "41.93ms".to_string(),
                    },
                    request: Request {
                        total: "30178057".to_string(),
                        req_per_sec: "1006342.33".to_string(),
                    },
                    transfer: Transfer {
                        total: "3.65GB".to_string(),
                        rate: "124.76MB/Sec".to_string(),
                    },
                });

            assert_eq!(actual, expect);
        }
    }

    mod latency {
        use super::*;

        #[test]
        fn ok() {
            let given = "0.50ms   1.22ms   0.02ms   41.93ms";
            let actual = given.parse::<Latency>();

            let expect = Ok(Latency {
                avg: "0.50ms".to_string(),
                std_env: "1.22ms".to_string(),
                min: "0.02ms".to_string(),
                max: "41.93ms".to_string(),
            });

            assert_eq!(actual, expect);
        }

        #[test]
        fn err() {
            let given = "0.50ms   1.22ms   0.02ms";
            let actual = given.parse::<Latency>();
            let expect = Err(MetricsError::ParseError);

            assert_eq!(actual, expect);
        }
    }

    mod request {
        use super::*;

        #[test]
        fn ok() {
            let given = "Total: 30178057 Req/Sec: 1006342.33";
            let actual = given.parse::<Request>();

            let expect = Ok(Request {
                total: "30178057".to_string(),
                req_per_sec: "1006342.33".to_string(),
            });

            assert_eq!(actual, expect);
        }

        #[test]
        fn err() {
            let given = "Total: 30178057";
            let actual = given.parse::<Request>();
            let expect = Err(MetricsError::ParseError);

            assert_eq!(actual, expect);
        }
    }

    mod transfer {
        use super::*;

        #[test]
        fn ok() {
            let given = "Total: 3.65 GB Transfer Rate: 124.76 MB/Sec";
            let actual = given.parse::<Transfer>();

            let expect = Ok(Transfer {
                total: "3.65GB".to_string(),
                rate: "124.76MB/Sec".to_string(),
            });

            assert_eq!(actual, expect);
        }

        #[test]
        fn err() {
            let given = "Total: 3.65 GB";
            let actual = given.parse::<Request>();
            let expect = Err(MetricsError::ParseError);

            assert_eq!(actual, expect);
        }
    }
}
