# 🦁 LEONIX

<p align="center">
  <strong>A Man from Iran, A Lion from Khuzestan</strong><br />
  <em>Born in Iran. Powered by XGNR. Driven by innovation.</em>
</p>

<table align="center">
  <tr>
    <td bgcolor="#f07832" align="center">
      <a href="https://Zunoplay.ir"><strong><font color="#ffffff">WEBSITE</font></strong></a>
    </td>
  </tr>
</table>

---

## About LEONIX

**LEONIX** is an independent game-technology research project by **XGNR**. It explores graphics technology, performance analysis, rendering research, and practical software tools using original code and authorized test materials.

## Current features

The first Rust-based command-line toolkit is now in the repository:

- **System report:** operating system, CPU architecture, logical processor count, and working directory.
- **CPU benchmark:** a repeatable integer workload with elapsed time, operation rate, and checksum.
- **Memory benchmark:** a bounded memory fill-and-scan workload.
- **CLI commands:** simple info, bench, and help commands.
- **Automated checks:** integration tests and a GitHub Actions workflow.

The current standard-library-only build does not report physical RAM usage, GPU details, or in-game FPS. Benchmark results are local micro-benchmark measurements, not predictions of game performance.

## Quick start

Install the stable Rust toolchain, then run:

```sh
cargo run --release -- info
cargo run --release -- bench
cargo run --release -- bench 5000000
cargo test --all-targets
```

See [Command Guide](docs/COMMANDS.md) for usage and benchmark details.

## Project structure

| Path | Purpose |
| --- | --- |
| src/core/ | Command routing and application entry logic |
| src/config/ | Safe benchmark defaults and input validation |
| src/monitor/ | System and runtime information |
| src/performance/ | Benchmark orchestration and reporting |
| src/benchmarks/ | CPU and memory workload implementations |
| src/tools/ | Formatting and output helpers |
| tests/ | Command-line integration tests |
| .github/workflows/ | Automated build and quality checks |
| docs/ | Usage and technical documentation |

## Research roadmap

- Expand system diagnostics with reliable, platform-specific data sources.
- Add repeatable benchmark scenarios and machine-readable reports.
- Explore graphics and rendering experiments with original test assets.
- Measure frame pacing and loading behavior in authorized test applications.

Planned work will be marked clearly until implemented and tested.

## Principles

- **Independent development:** LEONIX is created by XGNR and is not an official game port or endorsed product.
- **Responsible testing:** Use original code and assets, or materials that we are authorized to use.
- **Honest reporting:** Separate experimental goals from verified results.
- **Reproducibility:** Document hardware, software, and conditions when reporting benchmark results.

## Disclaimer

LEONIX is an independent, unofficial research project. It is not affiliated with, endorsed by, or associated with Rockstar Games or Sony Interactive Entertainment.

Grand Theft Auto and related names, characters, and trademarks belong to their respective owners. LEONIX does not distribute proprietary game files or copyrighted game assets.

---

<p align="center">
  <strong>XGNR — A Man from Iran, A Lion from Khuzestan. 🇮🇷🦁</strong><br />
  <em>Built with ambition. Driven by innovation.</em>
</p>
