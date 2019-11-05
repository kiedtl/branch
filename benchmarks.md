# Benchmarks
on an 4-core Arch Linux, see this neofetch
```
█████████████████  ████████   kiedtl@298739487
██████████████████  ████████   ----------------
██████████████████  ████████   OS: Manjaro Linux x86_64
██████████████████  ████████   Host: Aspire E5-576 V1.43
████████            ████████   Kernel: 4.19.23-1-MANJARO
████████  ████████  ████████   Uptime: 23 mins
████████  ████████  ████████   Packages: 1150 (pacman)
████████  ████████  ████████   Shell: bash 5.0.11
████████  ████████  ████████   WM: i3
████████  ████████  ████████   Theme: Breath [GTK2/3]
████████  ████████  ████████   Icons: maia [GTK2/3]
████████  ████████  ████████   Terminal: alacritty
████████  ████████  ████████   CPU: Intel i3-8130U (4) @ 3.400GHz
████████  ████████  ████████   GPU: Intel UHD Graphics 620
                               Memory: 444MiB / 15916MiB
                               Disk (/): 13G / 28G (50%)
```
benchmarks:
```
[kiedtl@298739487 linux-master]$ hyperfine "$BRANCH -t 1" "$BRANCH -t 2" "$BRANCH -t 4" "$BRANCH -t 8" "tree -a"
Benchmark #1: /home/kiedtl/repos/branch/target/release/branch -t 1
  Time (mean ± σ):     253.6 ms ±   2.0 ms    [User: 105.9 ms, System: 146.8 ms]
  Range (min … max):   251.0 ms … 257.4 ms    11 runs

Benchmark #2: /home/kiedtl/repos/branch/target/release/branch -t 2
  Time (mean ± σ):     237.0 ms ±   2.3 ms    [User: 182.1 ms, System: 176.6 ms]
  Range (min … max):   231.4 ms … 239.4 ms    12 runs

Benchmark #3: /home/kiedtl/repos/branch/target/release/branch -t 4
  Time (mean ± σ):     237.8 ms ±   3.0 ms    [User: 180.4 ms, System: 206.3 ms]
  Range (min … max):   232.4 ms … 242.4 ms    12 runs

Benchmark #4: /home/kiedtl/repos/branch/target/release/branch -t 8
  Time (mean ± σ):     267.4 ms ±   8.3 ms    [User: 198.2 ms, System: 231.3 ms]
  Range (min … max):   256.5 ms … 286.5 ms    10 runs

Benchmark #5: tree -a
  Time (mean ± σ):     239.3 ms ±   1.5 ms    [User: 128.7 ms, System: 109.7 ms]
  Range (min … max):   235.2 ms … 240.9 ms    12 runs

Summary
  '/home/kiedtl/repos/branch/target/release/branch -t 2' ran
    1.00 ± 0.02 times faster than '/home/kiedtl/repos/branch/target/release/branch -t 4'
    1.01 ± 0.01 times faster than 'tree -a'
    1.07 ± 0.01 times faster than '/home/kiedtl/repos/branch/target/release/branch -t 1'
    1.13 ± 0.04 times faster than '/home/kiedtl/repos/branch/target/release/branch -t 8'
```

