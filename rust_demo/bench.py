#!/usr/bin/env python3
import argparse
import os
import time
import statistics as stats
import sys

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

# Global clean style
plt.style.use('seaborn-v0_8-whitegrid')

# Try to import Rust extension if available
try:
    import rust_demo as rd
    HAS_RUST = True
except Exception:
    HAS_RUST = False


def time_function(fn, *args, repeats=5, warmup=1, **kwargs):
    for _ in range(max(0, warmup)):
        fn(*args, **kwargs)
    times = []
    for _ in range(max(1, repeats)):
        t0 = time.perf_counter()
        fn(*args, **kwargs)
        times.append(time.perf_counter() - t0)
    return min(times), stats.median(times), times


def bench_sum_of_squares(N, repeats, warmup):
    x = np.random.rand(N).astype(np.float64)

    def py_sum_squares(arr):
        s = 0.0
        for v in arr:
            s += v * v
        return s

    def np_sum_squares(arr):
        return float((arr * arr).sum())

    funcs = [("python loop", py_sum_squares),
             ("numpy (baseline)", np_sum_squares)]
    if HAS_RUST:
        funcs += [
            ("rust (pyo3)", lambda a: rd.sum_squares_numpy(a)),
            ("rust parallel", lambda a: rd.sum_squares_numpy_parallel(a)),
        ]

    results = []
    for name, fn in funcs:
        best, med, _ = time_function(fn, x, repeats=repeats, warmup=warmup)
        results.append((name, best, med))

    # speedups vs NumPy baseline for reference
    baseline_med = next(med for (name, best, med)
                        in results if "baseline" in name)
    rows = [(name, best, med, baseline_med / med)
            for (name, best, med) in results]
    return rows


def bench_primes(n, repeats, warmup):
    def py_count_primes(k):
        is_prime = [True] * (k + 1)
        if k >= 0:
            is_prime[0] = False
        if k >= 1:
            is_prime[1] = False
        p = 2
        while p * p <= k:
            if is_prime[p]:
                for m in range(p * p, k + 1, p):
                    is_prime[m] = False
            p += 1
        return sum(is_prime)

    funcs = [("python sieve (baseline)", py_count_primes)]
    if HAS_RUST:
        funcs.append(("rust sieve", lambda k: rd.count_primes(k)))

    results = []
    for name, fn in funcs:
        best, med, _ = time_function(fn, n, repeats=repeats, warmup=warmup)
        results.append((name, best, med))

    baseline_med = next(med for (name, best, med)
                        in results if "baseline" in name)
    rows = [(name, best, med, baseline_med / med)
            for (name, best, med) in results]
    return rows


# ---------- Plotting helpers focused on clarity for Python vs Rust ----------

COLORS = {
    "python": "#1f77b4",        # blue
    "numpy": "#7f7f7f",         # gray
    "rust": "#ff7f0e",          # orange
    "rust parallel": "#2ca02c",  # green
}


def pick_color(label: str) -> str:
    l = label.lower()
    if "rust parallel" in l:
        return COLORS["rust parallel"]
    if "rust" in l:
        return COLORS["rust"]
    if "numpy" in l or "baseline" in l:
        return COLORS["numpy"]
    if "python" in l:
        return COLORS["python"]
    return "#4c78a8"


def seconds_formatter(x, _pos):
    if x < 0.001:
        return f'{x * 1_000_000:.0f} µs'
    if x < 1.0:
        return f'{x * 1_000:.1f} ms'
    return f'{x:.3f} s'


def make_time_chart(title, labels, values, ylabel, path, logy=True):
    fig, ax = plt.subplots(figsize=(9, 5.2), constrained_layout=True)

    colors = [pick_color(lbl) for lbl in labels]
    bars = ax.bar(labels, values, color=colors,
                  edgecolor='#333', linewidth=0.8)

    ax.set_title(title, fontsize=12, pad=10)
    ax.set_ylabel(ylabel, fontsize=11)
    plt.setp(ax.get_xticklabels(), rotation=15, ha='right', fontsize=10)
    ax.tick_params(axis='y', labelsize=10)

    # Grid and spines
    ax.grid(axis='y', linestyle='--', linewidth=0.6, alpha=0.6)
    ax.grid(axis='x', visible=False)
    for spine in ('top', 'right'):
        ax.spines[spine].set_visible(False)

    if logy:
        ax.set_yscale('log')

    ax.yaxis.set_major_formatter(FuncFormatter(seconds_formatter))

    # Annotate bars with time values (in human units)
    def fmt(v):
        if v < 0.001:
            return f'{v * 1_000_000:.0f} µs'
        if v < 1.0:
            return f'{v * 1_000:.1f} ms'
        return f'{v:.3f} s'

    for rect, v in zip(bars, values):
        ax.annotate(fmt(v), xy=(rect.get_x() + rect.get_width() / 2, rect.get_height()),
                    xytext=(0, 4), textcoords="offset points",
                    ha='center', va='bottom', fontsize=9)

    fig.savefig(path, dpi=220)
    plt.close(fig)


def make_speedup_vs_python_chart(title, labels, values, path):
    """
    Plot speedup relative to the Python baseline (Python = 1.0).
    If Python is missing, falls back to the first item as baseline.
    """
    # Find python baseline index
    python_idx = None
    for i, lbl in enumerate(labels):
        if "python" in lbl.lower() and "baseline" in lbl.lower():
            python_idx = i
            break
        if "python" in lbl.lower() and "loop" in lbl.lower():
            python_idx = i
            break
        if "python" in lbl.lower() and "sieve" in lbl.lower():
            python_idx = i
            break

    if python_idx is None:
        python_idx = 0

    baseline = values[python_idx]
    speedups = [baseline / v if v > 0 else np.nan for v in values]

    fig, ax = plt.subplots(figsize=(9, 5.2), constrained_layout=True)
    colors = [pick_color(lbl) for lbl in labels]
    bars = ax.bar(labels, speedups, color=colors,
                  edgecolor='#333', linewidth=0.8)

    ax.set_title(title, fontsize=12, pad=10)
    ax.set_ylabel("× faster vs Python (higher is better)", fontsize=11)
    plt.setp(ax.get_xticklabels(), rotation=15, ha='right', fontsize=10)
    ax.tick_params(axis='y', labelsize=10)

    ax.grid(axis='y', linestyle='--', linewidth=0.6, alpha=0.6)
    ax.grid(axis='x', visible=False)
    for spine in ('top', 'right'):
        ax.spines[spine].set_visible(False)

    ax.yaxis.set_major_formatter(FuncFormatter(lambda x, _pos: f'{x:.1f}×'))

    # Annotate bars with × values
    for rect, v in zip(bars, speedups):
        ax.annotate(f'{v:.1f}×', xy=(rect.get_x() + rect.get_width() / 2, rect.get_height()),
                    xytext=(0, 4), textcoords="offset points",
                    ha='center', va='bottom', fontsize=9)

    # Draw a reference line at 1× (Python)
    ax.axhline(1.0, color='#888', linewidth=0.8, linestyle='--')

    fig.savefig(path, dpi=220)
    plt.close(fig)


# ---------- High-level plotting orchestration ----------

def save_plots(outdir, sos, primes, size, n_primes):
    os.makedirs(outdir, exist_ok=True)

    labels_s = [n for (n, _, m, s) in sos]
    med_s = [m for (n, _, m, s) in sos]
    # speed_s = [s for (n, _, m, s) in sos]  # vs NumPy (kept if needed)

    labels_p = [n for (n, _, m, s) in primes]
    med_p = [m for (n, _, m, s) in primes]

    p1 = os.path.join(outdir, "sum_of_squares_median_time.png")
    p2 = os.path.join(outdir, "sum_of_squares_speedup_vs_python.png")
    p3 = os.path.join(outdir, "primes_median_time.png")
    p4 = os.path.join(outdir, "primes_speedup_vs_python.png")

    # Time (log-scale) — clearer gaps
    make_time_chart(
        f"Sum of Squares — Median Time (N={size:,})",
        labels_s, med_s, "seconds", p1, logy=True
    )
    make_speedup_vs_python_chart(
        "Sum of Squares — × Faster vs Python",
        labels_s, med_s, p2
    )

    make_time_chart(
        f"Count Primes — Median Time (n={n_primes:,})",
        labels_p, med_p, "seconds", p3, logy=True
    )
    make_speedup_vs_python_chart(
        "Count Primes — × Faster vs Python",
        labels_p, med_p, p4
    )

    return [p1, p2, p3, p4]


def main():
    ap = argparse.ArgumentParser(
        description="Benchmark and plot Python vs Rust (clear Python vs Rust comparisons)"
    )
    ap.add_argument("--size", type=int, default=1_000_000,
                    help="Array size for sum of squares")
    ap.add_argument("--n-primes", type=int, default=500_000,
                    help="Upper bound for prime counting")
    ap.add_argument("--n-fib", type=int, default=200_000,
                    help="n for iterative Fibonacci (default 200,000)")
    ap.add_argument("--repeats", type=int, default=5, help="Timed repeats")
    ap.add_argument("--warmup", type=int, default=1, help="Warmup runs")
    ap.add_argument("--outdir", type=str, default=".",
                    help="Directory to save plots")
    args = ap.parse_args()

    sos = bench_sum_of_squares(args.size, args.repeats, args.warmup)
    primes = bench_primes(args.n_primes, args.repeats, args.warmup)

    paths = save_plots(args.outdir, sos, primes,
                       args.size, args.n_primes)
    for p in paths:
        print(p)


if __name__ == "__main__":
    main()
