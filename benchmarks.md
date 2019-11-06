# Benchmarks
Branch is currently tested with `branch 0.1.0` on two machine: an 4-core Acer Aspire, and a single-core Raspberry Pi Zero W. See the appropriate `neofetch`'s for more information.

Each benchmarks should be tested via [`hyperfine`](https://github.com/sharkdp/hyperfine), and should test the following command:
	- branch with 1 thread
	- branch with 2 threads
	- branch with 4 threads
	- branch with 8 threads
	- `tree -a`

Feel free to submit a PR with more benchmarks.

### Acer Aspire

**Neofetch**
```
kiedtl@<REDACTED>
-----------------
OS: Manjaro Linux x86_64
Host: Aspire E5-576 V1.43
Kernel: 4.19.23-1-MANJARO
Shell: bash 5.0.11
Terminal: alacritty
CPU: Intel i3-8130U (4) @ 3.400GHz
Memory: [total] 15916MiB
```

And now, the actual benchmarks:

```
$ hyperfine "branch -t 1" "branch -t 2" "branch -t 4" "branch -t 8" "tree -a"
Benchmark #1: branch -t 1
  Time (mean ± σ):     253.6 ms ±   2.0 ms    [User: 105.9 ms, System: 146.8 ms]
  Range (min … max):   251.0 ms … 257.4 ms    11 runs

Benchmark #2: branch -t 2
  Time (mean ± σ):     237.0 ms ±   2.3 ms    [User: 182.1 ms, System: 176.6 ms]
  Range (min … max):   231.4 ms … 239.4 ms    12 runs

Benchmark #3: branch -t 4
  Time (mean ± σ):     237.8 ms ±   3.0 ms    [User: 180.4 ms, System: 206.3 ms]
  Range (min … max):   232.4 ms … 242.4 ms    12 runs

Benchmark #4: branch -t 8
  Time (mean ± σ):     267.4 ms ±   8.3 ms    [User: 198.2 ms, System: 231.3 ms]
  Range (min … max):   256.5 ms … 286.5 ms    10 runs

Benchmark #5: tree -a
  Time (mean ± σ):     239.3 ms ±   1.5 ms    [User: 128.7 ms, System: 109.7 ms]
  Range (min … max):   235.2 ms … 240.9 ms    12 runs

Summary
  'branch -t 2' ran
    1.00 ± 0.02 times faster than 'branch -t 4'
    1.01 ± 0.01 times faster than 'tree -a'
    1.07 ± 0.01 times faster than 'branch -t 1'
    1.13 ± 0.04 times faster than 'branch -t 8'
```

### Raspberry Pi Zero W

**Neofetch**
```
kiedtl@3dot14 
------------- 
OS: Void Linux armv6l 
Host: Raspberry Pi Zero W Rev 1.1 
Kernel: 4.19.81_1 
Shell: bash 5.0.11 
Terminal: xterm
CPU: BCM2835 (1) @ 1.000GHz 
Memory: [total] 431MiB 
```

The benchmarks (remember, this is a _raspberry pi_, so the times reported here will be _significantly_ slower than on other machines!
