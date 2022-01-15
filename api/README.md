## Dev
```
source _scripts\dev.sh
```

# Perfroamnce command
## DO - PG
```
plow "https://todopad.in/todo?pending=true" -c 300 -n 20000  -H Authorization:"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NDIzMDc5MzIsImlzcyI6InRvZG9wYWQiLCJ1c2VyaWQiOjEsImVtYWlsIjoibWFkaGFuZ2FuZXNoQGdtYWlsLmNvbSJ9.T4Jwr4sYfX_Xf4D8O30zwPtUeONlmFpF8N4k9ofhNjw" 

Benchmarking https://todopad.in/todo?pending=true with 20000 request(s) using 300 connection(s).
@ Real-time charts is listening on http://[::]:18888

Summary:
  Elapsed   2m1.431s
  Count        20000
    2xx        19743
    5xx          257
  RPS        164.701
  Reads    0.119MB/s
  Writes   0.048MB/s

Statistics    Min       Mean      StdDev        Max     
  Latency   53.781ms  1.616701s  4.567882s  1m40.787751s
  RPS         0.49     168.02      60.09        224     

Latency Percentile:
  P50          P75        P90        P95        P99        P99.9       P99.99   
  79.229ms  1.918702s  3.584584s  6.905618s  28.959037s  31.70761s  1m40.782787s

Latency Histogram:
  502.218ms     16527  82.64%
  2.908503s      2073  10.37%
  6.520232s       761   3.81%
  11.238484s      286   1.43%
  17.195234s      111   0.56%
  19.852681s       28   0.14%
  31.712587s      204   1.02%
  1m29.452101s     10   0.05%


```


## DO - Sqlite
```
plow "https://gokarma.in/api/todo?pending=true" -c 300 -n 20000  -H Authorization:"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NDIzMDc1MzUsImlzcyI6InRvZG9wYWQiLCJ1c2VyaWQiOjEsImVtYWlsIjoidGVzdEB0ZXN0LmNvbSIsIm5hbWUiOiJ0ZXN0In0.ijZNLd6xMJcx7khZo8AMTnSBmibCVXEgShefW2fmaWI"

Benchmarking https://gokarma.in/api/todo?pending=true with 20000 request(s) using 300 connection(s).
@ Real-time charts is listening on http://[::]:18888

Summary:
  Elapsed    28.338s
  Count        20000
    2xx        20000
  RPS        705.749
  Reads    0.429MB/s
  Writes   0.214MB/s

Statistics   Min      Mean      StdDev       Max   
  Latency   5.34ms  423.054ms  338.233ms  3.358065s
  RPS       327.75   701.53      115.7     905.98  

Latency Percentile:
  P50           P75        P90        P95       P99      P99.9     P99.99  
  265.044ms  602.817ms  708.102ms  809.729ms  2.10436s  3.22351s  3.352244s

Latency Histogram:
  235.321ms  8068  40.34%
  335.083ms  6320  31.60%
  595.078ms  2614  13.07%
  720.454ms  1973   9.87%
  1.02349s    623   3.12%
  1.623215s   167   0.84%
  2.290774s   214   1.07%
  3.278695s    21   0.11%
```


## Local - on PG
```
madhanganesh@IN-MAC-163 api % plow "http://localhost:8080/todo?pending=true" -c 1000 -n 20000 -H Authorization:"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NDIzMTQwMDUsImlzcyI6InRvZG9wYWQiLCJ1c2VyaWQiOjI5NywiZW1haWwiOiJ0ZXN0QHRlc3QuY29tIn0.xC6bKHHN2-ffQRzLLOyY3ctTrUkzBd42mErTF2Q0Y70"

Benchmarking http://localhost:8080/todo?pending=true with 20000 request(s) using 1000 connection(s).
@ Real-time charts is listening on http://[::]:18888

Summary:
  Elapsed    18.301s
  Count        20000
    2xx        19948
    5xx           52
  RPS       1092.818
  Reads    0.662MB/s
  Writes   0.287MB/s

Statistics   Min      Mean      StdDev       Max    
  Latency   2.56ms  682.445ms  2.399781s  16.870234s
  RPS        1.99    1110.81    745.27     1893.22  

Latency Percentile:
  P50         P75        P90        P95        P99        P99.9      P99.99  
  9.322ms  517.578ms  828.848ms  2.719769s  15.800438s  16.84418s  16.866076s

Latency Histogram:
  229.565ms   18968  94.84%
  3.177405s     467   2.34%
  6.612745s      15   0.08%
  7.644078s      76   0.38%
  9.156615s       7   0.04%
  10.367468s     34   0.17%
  14.516662s    173   0.87%
  16.163649s    260   1.30%
```

## Local on - SQLite
```
madhanganesh@IN-MAC-163 api % plow "http://localhost:8080/todo?pending=true" -c 1000 -n 20000 -H Authorization:"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NDIzMDc1MzUsImlzcyI6InRvZG9wYWQiLCJ1c2VyaWQiOjEsImVtYWlsIjoidGVzdEB0ZXN0LmNvbSIsIm5hbWUiOiJ0ZXN0In0.ijZNLd6xMJcx7khZo8AMTnSBmibCVXEgShefW2fmaWI"

Benchmarking http://localhost:8080/todo?pending=true with 20000 request(s) using 1000 connection(s).
@ Real-time charts is listening on http://[::]:18888

Summary:
  Elapsed     3.032s
  Count        20000
    2xx        20000
  RPS       6595.716
  Reads    2.290MB/s
  Writes   1.830MB/s

Statistics    Min      Mean      StdDev       Max   
  Latency    182µs   149.262ms  155.536ms  1.177312s
  RPS       4234.23   6541.43    2106.51    8362.13 

Latency Percentile:
  P50           P75        P90        P95        P99       P99.9     P99.99  
  103.069ms  210.013ms  353.179ms  485.577ms  705.589ms  911.046ms  970.038ms

Latency Histogram:
  41.803ms   9067  45.34%
  155.126ms  6632  33.16%
  264.645ms  2491  12.46%
  389.152ms   857   4.29%
  531.912ms   474   2.37%
  666.468ms   361   1.81%
  763.439ms   106   0.53%
  919.86ms     12   0.06%
```
