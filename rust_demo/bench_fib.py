#!/usr/bin/env python3
import argparse
import sys
import time
import statistics as stats
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

# Clean global style
plt.style.use('seaborn-v0_8-whitegrid')

MASK64 = (1 << 64) - 1


def py_fib_iter_wrap64(n: int) -> int:
    a = 0
    b = 1
    for _ in range(n):
        a, b = b & MASK64, (a + b) & MASK64
    return a


def bench(fn, n, repeats, warmup):
    for _ in range(max(0, warmup)):
        fn(n)
    times = []
    for _ in range(max(1, repeats)):
        t0 = time.perf_counter()
        fn(n)
        times.append(time.perf_counter() - t0)
    return min(times), stats.median(times)


def fmt_seconds(v: float) -> str:
    if v < 0.001:
        return f'{v*1_000_000:.0f} µs'
    if v < 1.0:
        return f'{v*1_000:.1f} ms'
    return f'{v:.3f} s'


def main():
    p = argparse.ArgumentParser(
        description="Side-by-side Fibonacci (mod 2^64) — Python vs Rust"
    )
    p.add_argument("--n", type=int, default=20_000_000, help="Fibonacci steps")
    p.add_argument("--repeats", type=int, default=3, help="Timed repeats")
    p.add_argument("--warmup", type=int, default=1, help="Warmup runs")
    p.add_argument("--out", type=str, default="fib_python_vs_rust.png",
                   help="Output PNG for chart")
    args = p.parse_args()

    # Python baseline
    py_best, py_med = bench(py_fib_iter_wrap64, args.n,
                            args.repeats, args.warmup)

    # Rust (must be available as rust_demo.fib_iter_mod)
    have_rust = False
    rs_best = rs_med = None
    rust_err = ""
    try:
        import rust_demo as rd
        if not hasattr(rd, "fib_iter_mod"):
            raise AttributeError("rust_demo.fib_iter_mod is missing")

        def rust_fn(n: int) -> int:
            return rd.fib_iter_mod(n)

        rs_best, rs_med = bench(rust_fn, args.n, args.repeats, args.warmup)
        have_rust = True
    except Exception as e:
        rust_err = str(e)

    # Console table
    print(
        f"Fibonacci mod 2^64 — n={args.n:,}, repeats={args.repeats}, warmup={args.warmup}")
    print("Language, best_s, median_s, speedup_vs_python")
    print(f"Python,   {py_best:.6f}, {py_med:.6f}, 1.00x")
    if have_rust:
        speed = py_med / rs_med if rs_med and rs_med > 0 else float('nan')
        print(f"Rust,     {rs_best:.6f}, {rs_med:.6f}, {speed:.2f}x")
    else:
        print("Rust,     N/A, N/A, N/A  (Build your extension and expose fib_iter_mod)")
        print(
            f"Hint: maturin develop  # ensure fib_iter_mod is in src/lib.rs  ({rust_err})")

    # Plot: median time (log scale) and speedup vs Python
    labels = ["Python"] + (["Rust"] if have_rust else [])
    med_values = [py_med] + ([rs_med] if have_rust else [])
    colors = ["#1f77b4", "#ff7f0e"][:len(labels)]  # Python blue, Rust orange

    # Create figure
    if have_rust:
        fig, axes = plt.subplots(1, 2, figsize=(
            11, 4.8), constrained_layout=True)
        ax_time, ax_speed = axes
    else:
        fig, ax_time = plt.subplots(
            figsize=(6.5, 4.8), constrained_layout=True)
        ax_speed = None

    # Left panel: median time (log y)
    bars = ax_time.bar(labels, med_values, color=colors,
                       edgecolor="#333", linewidth=0.8)
    ax_time.set_title(f"Fibonacci mod 2^64 — median time (n={args.n:,})")
    ax_time.set_ylabel("seconds (log scale: lower is better)")
    ax_time.set_yscale('log')
    ax_time.grid(axis='y', linestyle='--', linewidth=0.6, alpha=0.6)
    ax_time.grid(axis='x', visible=False)
    for rect, v in zip(bars, med_values):
        ax_time.annotate(fmt_seconds(v),
                         xy=(rect.get_x() + rect.get_width()/2, rect.get_height()),
                         xytext=(0, 4), textcoords="offset points",
                         ha='center', va='bottom', fontsize=9)

    # Right panel: × faster vs Python
    if have_rust and ax_speed is not None:
        speedups = [
            1.0, (py_med / rs_med if rs_med and rs_med > 0 else float('nan'))]
        bars2 = ax_speed.bar(labels, speedups, color=colors,
                             edgecolor="#333", linewidth=0.8)
        ax_speed.set_title("× faster vs Python (higher is better)")
        ax_speed.set_ylabel("×")
        ax_speed.grid(axis='y', linestyle='--', linewidth=0.6, alpha=0.6)
        ax_speed.grid(axis='x', visible=False)
        ax_speed.axhline(1.0, color="#888", linewidth=0.8, linestyle="--")
        ax_speed.yaxis.set_major_formatter(
            FuncFormatter(lambda x, _pos: f"{x:.1f}×"))
        for rect, v in zip(bars2, speedups):
            ax_speed.annotate(f"{v:.2f}×",
                              xy=(rect.get_x() + rect.get_width() /
                                  2, rect.get_height()),
                              xytext=(0, 4), textcoords="offset points",
                              ha='center', va='bottom', fontsize=9)

    fig.savefig(args.out, dpi=220)
    print(f"Saved chart → {args.out}")


if __name__ == "__main__":
    main()
