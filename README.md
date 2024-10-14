# GraphQL Parsers Benchmarks 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise.

| Case | asyync-graphql-parser | graphql_query (Stellate) | graphql-parser |
|------|------ | ------ | ------|
| directive.graphql | 37.179 | 9.9479 | 39.072 |
| huge.graphql | 7531.799999999999 | 584.98 | 1017.8000000000001 |
| input.graphql | 59.565 | 11.213 | 46.371 |
| multiple-ops.graphql | 73.022 | 12.571 | 54.69 |
| union.graphql | 27.454 | 9.2752 | 18.8 |
| simple-hello.graphql | 10.68 | 8.013 | 10.264 |
| introspection.graphql | 107.64 | 14.451 | 54.08 |
| variables.graphql | 72.711 | 12.502 | 68.246 |
| subscription.graphql | 14.144 | 8.2887 | 12.283 |
| nested-recursive.graphql | 73.913 | 12.539 | 35.926 |
| fragments.graphql | 52.856 | 10.963 | 38.33 |


![Benchmark Bar Chart (Microseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B37.179%2C59.565%2C73.022%2C27.454%2C10.68%2C107.64%2C72.711%2C14.144%2C73.913%2C52.856%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B9.9479%2C11.213%2C12.571%2C9.2752%2C8.013%2C14.451%2C12.502%2C8.2887%2C12.539%2C10.963%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B39.072%2C46.371%2C54.69%2C18.8%2C10.264%2C54.08%2C68.246%2C12.283%2C35.926%2C38.33%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22directive.graphql%22%2C%22input.graphql%22%2C%22multiple-ops.graphql%22%2C%22union.graphql%22%2C%22simple-hello.graphql%22%2C%22introspection.graphql%22%2C%22variables.graphql%22%2C%22subscription.graphql%22%2C%22nested-recursive.graphql%22%2C%22fragments.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Microseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

![Benchmark Bar Chart (Milliseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B7531.799999999999%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B584.98%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B1017.8000000000001%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22huge.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Milliseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

asyync-graphql-parser / huge.graphql
                        time:   [7.5318 ms 7.6220 ms 7.7483 ms]
                        change: [-3.6987% -1.7213% +0.1934%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

asyync-graphql-parser / introspection.graphql
                        time:   [107.64 µs 110.53 µs 114.24 µs]
                        change: [+310.68% +318.73% +327.71%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / union.graphql
                        time:   [27.454 µs 28.165 µs 29.069 µs]
                        change: [+63.964% +67.723% +72.565%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high severe

asyync-graphql-parser / input.graphql
                        time:   [59.565 µs 60.705 µs 62.213 µs]
                        change: [+252.91% +260.78% +268.27%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

asyync-graphql-parser / subscription.graphql
                        time:   [14.144 µs 14.259 µs 14.425 µs]
                        change: [-1.7102% -0.8893% +0.0914%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

asyync-graphql-parser / nested-recursive.graphql
                        time:   [73.913 µs 76.442 µs 79.470 µs]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

asyync-graphql-parser / directive.graphql
                        time:   [37.179 µs 37.695 µs 38.473 µs]
                        change: [+125.08% +128.82% +132.84%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / fragments.graphql
                        time:   [52.856 µs 53.908 µs 55.367 µs]
                        change: [+49.668% +52.401% +55.484%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / simple-hello.graphql
                        time:   [10.680 µs 11.465 µs 12.475 µs]
                        change: [+0.3583% +3.9453% +8.8238%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

asyync-graphql-parser / multiple-ops.graphql
                        time:   [73.022 µs 73.301 µs 73.662 µs]
                        change: [+279.94% +292.57% +303.84%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe

asyync-graphql-parser / variables.graphql
                        time:   [72.711 µs 73.203 µs 73.834 µs]
                        change: [-4.5240% -3.3549% -2.2523%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

graphql_query (Stellate) / huge.graphql
                        time:   [584.98 µs 601.83 µs 624.85 µs]
                        change: [-9.1030% -5.0393% -1.0515%] (p = 0.02 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / introspection.graphql
                        time:   [14.451 µs 14.806 µs 15.305 µs]
                        change: [+50.138% +55.837% +61.194%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / union.graphql
                        time:   [9.2752 µs 9.3251 µs 9.3931 µs]
                        change: [-2.1937% +2.0266% +5.7578%] (p = 0.34 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

graphql_query (Stellate) / input.graphql
                        time:   [11.213 µs 11.668 µs 12.253 µs]
                        change: [+30.750% +36.103% +41.477%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

graphql_query (Stellate) / subscription.graphql
                        time:   [8.2887 µs 8.3713 µs 8.4830 µs]
                        change: [-7.1288% -4.1590% -1.7719%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / nested-recursive.graphql
                        time:   [12.539 µs 13.074 µs 13.720 µs]
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

graphql_query (Stellate) / directive.graphql
                        time:   [9.9479 µs 10.144 µs 10.394 µs]
                        change: [+3.5216% +11.634% +19.834%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

graphql_query (Stellate) / fragments.graphql
                        time:   [10.963 µs 11.248 µs 11.679 µs]
                        change: [+7.5005% +11.039% +15.563%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / simple-hello.graphql
                        time:   [8.0130 µs 8.1812 µs 8.3986 µs]
                        change: [+4.3627% +20.986% +38.811%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

graphql_query (Stellate) / multiple-ops.graphql
                        time:   [12.571 µs 12.827 µs 13.214 µs]
                        change: [+41.696% +45.053% +48.421%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / variables.graphql
                        time:   [12.502 µs 12.765 µs 13.160 µs]
                        change: [-15.677% -8.4136% -1.3236%] (p = 0.03 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

graphql-parser / huge.graphql
                        time:   [1.0178 ms 1.0214 ms 1.0251 ms]
                        change: [-2.5455% -1.2327% -0.4017%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

graphql-parser / introspection.graphql
                        time:   [54.080 µs 54.255 µs 54.473 µs]
                        change: [+221.62% +223.90% +226.21%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

graphql-parser / union.graphql
                        time:   [18.800 µs 18.899 µs 19.020 µs]
                        change: [+41.732% +44.073% +46.130%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

graphql-parser / input.graphql
                        time:   [46.371 µs 46.873 µs 47.741 µs]
                        change: [+196.98% +203.87% +209.11%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

graphql-parser / subscription.graphql
                        time:   [12.283 µs 12.356 µs 12.443 µs]
                        change: [-3.2281% -1.7232% -0.5699%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

graphql-parser / nested-recursive.graphql
                        time:   [35.926 µs 36.114 µs 36.379 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

graphql-parser / directive.graphql
                        time:   [39.072 µs 39.943 µs 41.217 µs]
                        change: [+172.54% +180.44% +189.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

graphql-parser / fragments.graphql
                        time:   [38.330 µs 38.524 µs 38.773 µs]
                        change: [+52.736% +54.976% +57.775%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe

graphql-parser / simple-hello.graphql
                        time:   [10.264 µs 10.451 µs 10.793 µs]
                        change: [-0.2795% +3.3087% +6.3304%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

graphql-parser / multiple-ops.graphql
                        time:   [54.690 µs 55.964 µs 57.545 µs]
                        change: [+275.30% +280.42% +285.48%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

graphql-parser / variables.graphql
                        time:   [68.246 µs 68.793 µs 69.661 µs]
                        change: [+6.2752% +7.1701% +8.1695%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe


```

</details>
