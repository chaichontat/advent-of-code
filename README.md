# Advent of Code Solutions
![GitHub Actions](https://github.com/chaichontat/advent-of-code/actions/workflows/python-package-conda.yml/badge.svg)
![GitHub Actions](https://github.com/chaichontat/advent-of-code/actions/workflows/rust.yml/badge.svg)
![GitHub Actions](https://github.com/chaichontat/advent-of-code/actions/workflows/haskell.yml/badge.svg)

An evolving repository of best practices in software engineering that I'm aware of.

## Philosophy
### July 2021
- [Code should be written for humans.](https://douglasorr.github.io/2020-03-data-for-machines/article.html)
  - As declarative and functional as is reasonable.
  - Code focuses on the algorithm rather than the implementation. For example, explicitly name Dijkstra's algorithm rather than moving a bunch of things around a priority queue.
- Rust
  - Speed within 10× of the known C++ [state-of-the-art](https://github.com/Voltara?tab=repositories&q=advent&type=&language=&sort=).
  - Not handcoding x86-64 SIMD as [universal intrinsics](https://numpy.org/neps/nep-0038-SIMD-optimizations.html) are in active development.
- Python
  - Enforce strict static type checking.
  - Avoid [`None`](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/) like SARS-CoV-2.

### December 2020
- Brute force only when necessary.
- Readable and maintainable code.
- "Elegant" solutions.
