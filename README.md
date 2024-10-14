# GraphQL Parsers Benchmarks 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise.

| Case | asyync-graphql-parser | graphql_query (Stellate) | graphql-parser |
|------|------ | ------ | ------|
| fragments.graphql | 34.817 | 9.8989 | 25.168 |
| input.graphql | 16.577 | 8.4025 | 15.091 |
| recursive-types.graphql | 18.548 | 8.7571 | 14.474 |
| union.graphql | 16.501 | 8.9255 | 13.045 |
| variables.graphql | 74.467 | 12.915 | 63.403 |
| directive.graphql | 16.147 | 8.6043 | 14.056 |
| introspection.graphql | 25.853 | 9.1898 | 16.639 |
| nested.graphql | 22.204 | 8.8473 | 16.221 |
| huge.graphql | 7686.3 | 615.74 | 1020 |
| simple-hello.graphql | 10.333 | 8.0284 | 9.7934 |
| multiple-ops.graphql | 17.991 | 8.6985 | 14.5 |
| interfaces.graphql | 13.879 | 8.3523 | 12.202 |
| invalid-query.graphql | 13.883 | 8.5462 | 11.871 |
| subscription.graphql | 14.291 | 8.4949 | 12.418 |


![Benchmark Bar Chart (Microseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B34.817%2C16.577%2C18.548%2C16.501%2C74.467%2C16.147%2C25.853%2C22.204%2C10.333%2C17.991%2C13.879%2C13.883%2C14.291%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B9.8989%2C8.4025%2C8.7571%2C8.9255%2C12.915%2C8.6043%2C9.1898%2C8.8473%2C8.0284%2C8.6985%2C8.3523%2C8.5462%2C8.4949%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B25.168%2C15.091%2C14.474%2C13.045%2C63.403%2C14.056%2C16.639%2C16.221%2C9.7934%2C14.5%2C12.202%2C11.871%2C12.418%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22fragments.graphql%22%2C%22input.graphql%22%2C%22recursive-types.graphql%22%2C%22union.graphql%22%2C%22variables.graphql%22%2C%22directive.graphql%22%2C%22introspection.graphql%22%2C%22nested.graphql%22%2C%22simple-hello.graphql%22%2C%22multiple-ops.graphql%22%2C%22interfaces.graphql%22%2C%22invalid-query.graphql%22%2C%22subscription.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Microseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

![Benchmark Bar Chart (Milliseconds)](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22backgroundColor%22%3A%22rgba%28255%2C%2099%2C%20132%2C%200.6%29%22%2C%22data%22%3A%5B7686.3%5D%2C%22label%22%3A%22asyync-graphql-parser%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2854%2C%20162%2C%20235%2C%200.6%29%22%2C%22data%22%3A%5B615.74%5D%2C%22label%22%3A%22graphql_query%20%28Stellate%29%22%7D%2C%7B%22backgroundColor%22%3A%22rgba%2875%2C%20192%2C%20192%2C%200.6%29%22%2C%22data%22%3A%5B1020.0%5D%2C%22label%22%3A%22graphql-parser%22%7D%5D%2C%22labels%22%3A%5B%22huge.graphql%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Benchmark%20Performance%20%28Milliseconds%29%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

asyync-graphql-parser / interfaces.graphql
                        time:   [13.879 µs 14.103 µs 14.406 µs]
                        change: [-2.3684% -0.1645% +2.1842%] (p = 0.90 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

asyync-graphql-parser / huge.graphql
                        time:   [7.6863 ms 7.7555 ms 7.8769 ms]
                        change: [-0.3492% +0.7171% +2.3055%] (p = 0.42 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

asyync-graphql-parser / introspection.graphql
                        time:   [25.853 µs 26.234 µs 26.751 µs]
                        change: [-3.1298% -1.8210% -0.3438%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

asyync-graphql-parser / union.graphql
                        time:   [16.501 µs 16.773 µs 17.145 µs]
                        change: [-7.5771% -4.2325% -1.2280%] (p = 0.01 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

asyync-graphql-parser / input.graphql
                        time:   [16.577 µs 16.908 µs 17.347 µs]
                        change: [-7.5027% -4.6387% -1.9605%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / nested.graphql
                        time:   [22.204 µs 22.404 µs 22.653 µs]
                        change: [-5.8673% -4.0848% -2.5622%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

asyync-graphql-parser / subscription.graphql
                        time:   [14.291 µs 14.373 µs 14.479 µs]
                        change: [-0.3088% +0.2318% +0.8546%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / directive.graphql
                        time:   [16.147 µs 16.392 µs 16.718 µs]
                        change: [+2.3316% +3.6898% +5.3573%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

asyync-graphql-parser / fragments.graphql
                        time:   [34.817 µs 35.102 µs 35.444 µs]
                        change: [+1.0949% +1.9400% +2.9019%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

asyync-graphql-parser / recursive-types.graphql
                        time:   [18.548 µs 18.745 µs 19.013 µs]
                        change: [-1.3998% -0.4630% +0.7242%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

asyync-graphql-parser / simple-hello.graphql
                        time:   [10.333 µs 10.481 µs 10.670 µs]
                        change: [-2.4762% -1.4705% -0.2534%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

asyync-graphql-parser / multiple-ops.graphql
                        time:   [17.991 µs 18.397 µs 18.935 µs]
                        change: [-0.9868% +2.2428% +5.8871%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe

asyync-graphql-parser / variables.graphql
                        time:   [74.467 µs 75.468 µs 76.736 µs]
                        change: [+1.8791% +2.7429% +3.7616%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  7 (7.00%) high severe

asyync-graphql-parser / invalid-query.graphql
                        time:   [13.883 µs 14.305 µs 15.028 µs]
                        change: [-0.8225% +0.6575% +2.6116%] (p = 0.56 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / interfaces.graphql
                        time:   [8.3523 µs 8.4860 µs 8.6625 µs]
                        change: [-0.7924% +0.3146% +1.5743%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

graphql_query (Stellate) / huge.graphql
                        time:   [615.74 µs 638.37 µs 664.36 µs]
                        change: [-4.3651% +0.4908% +5.2352%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

graphql_query (Stellate) / introspection.graphql
                        time:   [9.1898 µs 9.3274 µs 9.5049 µs]
                        change: [-0.9474% +1.8617% +5.8764%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe

graphql_query (Stellate) / union.graphql
                        time:   [8.9255 µs 9.3281 µs 9.8034 µs]
                        change: [+2.6404% +6.5999% +11.047%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe

graphql_query (Stellate) / input.graphql
                        time:   [8.4025 µs 8.4784 µs 8.5799 µs]
                        change: [-1.4681% +0.3424% +3.2106%] (p = 0.83 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

graphql_query (Stellate) / nested.graphql
                        time:   [8.8473 µs 9.0177 µs 9.2424 µs]
                        change: [-5.1577% -3.3016% -1.3855%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / subscription.graphql
                        time:   [8.4949 µs 8.6923 µs 8.9572 µs]
                        change: [+0.8389% +3.2506% +6.6796%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

graphql_query (Stellate) / directive.graphql
                        time:   [8.6043 µs 8.8629 µs 9.1796 µs]
                        change: [+1.6972% +8.1668% +15.491%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) high mild
  14 (14.00%) high severe

graphql_query (Stellate) / fragments.graphql
                        time:   [9.8989 µs 10.217 µs 10.652 µs]
                        change: [-2.4212% +0.4440% +3.3638%] (p = 0.78 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / recursive-types.graphql
                        time:   [8.7571 µs 8.9581 µs 9.2277 µs]
                        change: [-1.8402% +0.6743% +3.0162%] (p = 0.62 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high severe

graphql_query (Stellate) / simple-hello.graphql
                        time:   [8.0284 µs 8.1138 µs 8.2324 µs]
                        change: [-16.867% -9.8867% -3.5085%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

graphql_query (Stellate) / multiple-ops.graphql
                        time:   [8.6985 µs 8.8353 µs 9.0096 µs]
                        change: [-6.7326% -2.6035% +0.7274%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

graphql_query (Stellate) / variables.graphql
                        time:   [12.915 µs 13.488 µs 14.256 µs]
                        change: [-1.5631% +6.0436% +14.624%] (p = 0.16 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) high mild
  15 (15.00%) high severe

graphql_query (Stellate) / invalid-query.graphql
                        time:   [8.5462 µs 9.1042 µs 9.8399 µs]
                        change: [-12.016% -2.1245% +6.7320%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

graphql-parser / interfaces.graphql
                        time:   [12.202 µs 12.660 µs 13.273 µs]
                        change: [+3.1912% +7.6464% +13.537%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

graphql-parser / huge.graphql
                        time:   [1.0200 ms 1.0394 ms 1.0760 ms]
                        change: [-0.4710% +0.4251% +1.7504%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

graphql-parser / introspection.graphql
                        time:   [16.639 µs 16.762 µs 16.909 µs]
                        change: [-5.6925% -2.7448% -0.8606%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

graphql-parser / union.graphql
                        time:   [13.045 µs 13.276 µs 13.581 µs]
                        change: [-1.1033% +0.6618% +2.4757%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

graphql-parser / input.graphql
                        time:   [15.091 µs 15.402 µs 15.821 µs]
                        change: [-4.6376% -1.3365% +2.0842%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

graphql-parser / nested.graphql
                        time:   [16.221 µs 16.455 µs 16.771 µs]
                        change: [-0.3050% +1.0988% +2.6443%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe

graphql-parser / subscription.graphql
                        time:   [12.418 µs 12.601 µs 12.928 µs]
                        change: [+1.5386% +2.7441% +4.1318%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

graphql-parser / directive.graphql
                        time:   [14.056 µs 14.413 µs 15.078 µs]
                        change: [-1.0626% +0.8386% +3.1683%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

graphql-parser / fragments.graphql
                        time:   [25.168 µs 25.395 µs 25.727 µs]
                        change: [-7.0562% -4.6344% -2.3885%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

graphql-parser / recursive-types.graphql
                        time:   [14.474 µs 14.595 µs 14.748 µs]
                        change: [-0.2970% +1.0184% +2.1169%] (p = 0.11 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

graphql-parser / simple-hello.graphql
                        time:   [9.7934 µs 9.8977 µs 10.029 µs]
                        change: [-2.2404% +0.5924% +3.9437%] (p = 0.75 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

graphql-parser / multiple-ops.graphql
                        time:   [14.500 µs 14.578 µs 14.675 µs]
                        change: [-7.8587% -4.0586% -1.3354%] (p = 0.01 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

graphql-parser / variables.graphql
                        time:   [63.403 µs 63.693 µs 64.069 µs]
                        change: [-3.8025% -2.2875% -0.9440%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

graphql-parser / invalid-query.graphql
                        time:   [11.871 µs 12.086 µs 12.403 µs]
                        change: [-2.5707% -0.9521% +0.7783%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe


```

</details>
