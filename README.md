# VectorCraft Studio

An open-source, sovereign vector illustration and graphic design application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![VectorCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode vector UI with Bézier pen controls, Pathfinder operations, and artboards.
- **`crates/engine`**: Precision 2D geometric boolean solver, SVG 2.0 parser, and high-DPI GPU path renderer.

## Legal & Compliance Notice

VectorCraft is an independent open-source vector editor. It is not affiliated with Adobe Inc. Adobe, Illustrator, and Creative Cloud are trademarks of Adobe Inc. Bézier curve manipulation and 2D boolean constructive geometry are public domain mathematical operations (17 U.S.C. § 102(b)).

## License

Dual-licensed under MIT OR Apache-2.0.
