# [Something] Benchmarks 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise.

| Case | async-graphql-parser | graphql_query (Stellate) | graphql-parser |
|------|------ | ------ | ------|

| nested.graphql | 0 | 9.1897 | 16.123 |
| subscription.graphql | 0 | 8.3642 | 12.153 |
| interfaces.graphql | 0 | 8.3554 | 11.886 |
| directive.graphql | 0 | 8.5099 | 13.859 |
| recursive-types.graphql | 0 | 8.8064 | 14.3 |
| multiple-ops.graphql | 0 | 8.7984 | 14.716 |
| simple-hello.graphql | 0 | 8.1142 | 9.8752 |
| invalid-query.graphql | 0 | 8.4075 | 12.002 |
| fragments.graphql | 0 | 9.8934 | 25.512 |
| input.graphql | 0 | 8.507 | 15.247 |
| introspection.graphql | 0 | 9.1634 | 16.894 |
| huge.graphql | 0 | 607 | 1021.3000000000001 |
| variables.graphql | 0 | 12.951 | 64.252 |
| union.graphql | 0 | 8.5301 | 12.945 |


![Benchmark Bar Chart (Microseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B23.057%2C14.291%2C13.891%2C15.675%2C18.748%2C18.108%2C10.469%2C13.774%2C34.347%2C17.042%2C26.486%2C73.072%2C16.837%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B9.1897%2C8.3642%2C8.3554%2C8.5099%2C8.8064%2C8.7984%2C8.1142%2C8.4075%2C9.8934%2C8.507%2C9.1634%2C12.951%2C8.5301%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B16.123%2C12.153%2C11.886%2C13.859%2C14.3%2C14.716%2C9.8752%2C12.002%2C25.512%2C15.247%2C16.894%2C64.252%2C12.945%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22nested.graphql%22%2C%22subscription.graphql%22%2C%22interfaces.graphql%22%2C%22directive.graphql%22%2C%22recursive-types.graphql%22%2C%22multiple-ops.graphql%22%2C%22simple-hello.graphql%22%2C%22invalid-query.graphql%22%2C%22fragments.graphql%22%2C%22input.graphql%22%2C%22introspection.graphql%22%2C%22variables.graphql%22%2C%22union.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Microseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

![Benchmark Bar Chart (Milliseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B7674.299999999999%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B607.0%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B1021.3000000000001%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22huge.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Milliseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

asyync-graphql-parser / interfaces.graphql
                        time:   [13.891 µs 14.038 µs 14.299 µs]
                        change: [-3.8389% -1.5315% +0.9169%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

asyync-graphql-parser / huge.graphql
                        time:   [7.6743 ms 7.7003 ms 7.7294 ms]
                        change: [-47.897% -26.351% -3.1131%] (p = 0.10 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

asyync-graphql-parser / introspection.graphql
                        time:   [26.486 µs 26.696 µs 26.945 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

asyync-graphql-parser / union.graphql
                        time:   [16.837 µs 17.293 µs 17.889 µs]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

asyync-graphql-parser / input.graphql
                        time:   [17.042 µs 17.592 µs 18.290 µs]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

asyync-graphql-parser / nested.graphql
                        time:   [23.057 µs 23.318 µs 23.620 µs]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / subscription.graphql
                        time:   [14.291 µs 14.357 µs 14.441 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

asyync-graphql-parser / directive.graphql
                        time:   [15.675 µs 15.747 µs 15.825 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

asyync-graphql-parser / fragments.graphql
                        time:   [34.347 µs 34.483 µs 34.631 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

asyync-graphql-parser / recursive-types.graphql
                        time:   [18.748 µs 18.827 µs 18.910 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

asyync-graphql-parser / simple-hello.graphql
                        time:   [10.469 µs 10.531 µs 10.606 µs]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  8 (8.00%) high severe

asyync-graphql-parser / multiple-ops.graphql
                        time:   [18.108 µs 18.207 µs 18.322 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

asyync-graphql-parser / variables.graphql
                        time:   [73.072 µs 73.288 µs 73.531 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

asyync-graphql-parser / invalid-query.graphql
                        time:   [13.774 µs 13.826 µs 13.888 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

graphql_query (Stellate) / interfaces.graphql
                        time:   [8.3554 µs 8.3973 µs 8.4470 µs]
                        change: [+0.1141% +0.6853% +1.2599%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

graphql_query (Stellate) / huge.graphql
                        time:   [607.00 µs 621.62 µs 636.40 µs]
                        change: [-3.3379% +1.8029% +6.9637%] (p = 0.51 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

graphql_query (Stellate) / introspection.graphql
                        time:   [9.1634 µs 9.2414 µs 9.3423 µs]
                        change: [-5.5906% -3.1307% -0.6615%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

graphql_query (Stellate) / union.graphql
                        time:   [8.5301 µs 8.6233 µs 8.7460 µs]
                        change: [-5.9144% -2.9463% -0.4328%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / input.graphql
                        time:   [8.5070 µs 8.5913 µs 8.6921 µs]
                        change: [-2.2073% -0.4314% +1.1293%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

graphql_query (Stellate) / nested.graphql
                        time:   [9.1897 µs 9.4091 µs 9.6834 µs]
                        change: [-4.7124% -1.0879% +2.0869%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

graphql_query (Stellate) / subscription.graphql
                        time:   [8.3642 µs 8.4275 µs 8.5046 µs]
                        change: [-1.2485% -0.5123% +0.1926%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / directive.graphql
                        time:   [8.5099 µs 8.5888 µs 8.6914 µs]
                        change: [-7.8936% -3.0864% +0.6367%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  8 (8.00%) high severe

graphql_query (Stellate) / fragments.graphql
                        time:   [9.8934 µs 10.021 µs 10.173 µs]
                        change: [-1.6221% +0.6841% +3.1826%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

graphql_query (Stellate) / recursive-types.graphql
                        time:   [8.8064 µs 8.9187 µs 9.0519 µs]
                        change: [-0.7486% +0.8643% +2.9303%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

graphql_query (Stellate) / simple-hello.graphql
                        time:   [8.1142 µs 8.2422 µs 8.4055 µs]
                        change: [+2.8061% +10.093% +20.465%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

graphql_query (Stellate) / multiple-ops.graphql
                        time:   [8.7984 µs 9.1504 µs 9.8324 µs]
                        change: [-3.3047% +0.0782% +4.7082%] (p = 0.97 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

graphql_query (Stellate) / variables.graphql
                        time:   [12.951 µs 13.364 µs 13.903 µs]
                        change: [-11.978% -5.9666% +0.3049%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  12 (12.00%) high severe

graphql_query (Stellate) / invalid-query.graphql
                        time:   [8.4075 µs 8.5129 µs 8.6517 µs]
                        change: [+0.3836% +6.7560% +16.315%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

graphql-parser / interfaces.graphql
                        time:   [11.886 µs 11.955 µs 12.037 µs]
                        change: [-0.9008% +0.2105% +1.0988%] (p = 0.71 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

graphql-parser / huge.graphql
                        time:   [1.0213 ms 1.0247 ms 1.0283 ms]
                        change: [+1.2226% +1.6815% +2.0853%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

graphql-parser / introspection.graphql
                        time:   [16.894 µs 17.410 µs 18.341 µs]
                        change: [+4461749% +4542777% +4683249%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

graphql-parser / union.graphql
                        time:   [12.945 µs 13.209 µs 13.553 µs]
                        change: [+3409106% +3444856% +3491676%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

graphql-parser / input.graphql
                        time:   [15.247 µs 15.762 µs 16.424 µs]
                        change: [+4008017% +4106297% +4241054%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

graphql-parser / nested.graphql
                        time:   [16.123 µs 16.308 µs 16.599 µs]
                        change: [+4232766% +4265797% +4305109%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

graphql-parser / subscription.graphql
                        time:   [12.153 µs 12.233 µs 12.346 µs]
                        change: [+3209062% +3227428% +3249589%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

graphql-parser / directive.graphql
                        time:   [13.859 µs 13.996 µs 14.182 µs]
                        change: [+3662923% +3707371% +3757337%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

graphql-parser / fragments.graphql
                        time:   [25.512 µs 26.097 µs 26.827 µs]
                        change: [+6854436% +7024456% +7194105%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

graphql-parser / recursive-types.graphql
                        time:   [14.300 µs 14.432 µs 14.603 µs]
                        change: [+3796220% +3829529% +3873168%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

graphql-parser / simple-hello.graphql
                        time:   [9.8752 µs 10.019 µs 10.217 µs]
                        change: [+2595127% +2626645% +2667219%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

graphql-parser / multiple-ops.graphql
                        time:   [14.716 µs 14.947 µs 15.271 µs]
                        change: [+3838579% +3956972% +4137664%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

graphql-parser / variables.graphql
                        time:   [64.252 µs 65.676 µs 67.517 µs]
                        change: [+17017440% +17232704% +17457459%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

graphql-parser / invalid-query.graphql
                        time:   [12.002 µs 12.179 µs 12.395 µs]
                        change: [+3171939% +3200489% +3241259%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe


```

</details>
