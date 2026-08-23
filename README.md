# Molecular

Molecular is a molecule drawing tool for Rust, inspired by ChemDraw. It renders
onto an iced canvas and uses kurbo for the geometry.

## Features

- Pan the canvas with the middle mouse button.
- Draw with the left mouse button: click to drop a point, drag to draw a line.
- Double- or triple-click starts text entry; Esc commits the text.
- Save and load the whole drawing state as JSON through a native file dialog
  (see `saved.json`).

Three tools are available: pick, draw, and typing. State is serialized with
serde, so drawings survive restarts.

## Status

Early and a bit rough around the edges. The `pubchem` crate is declared as a
dependency but is not wired up yet. The only shapes so far are points, lines,
and text labels.

## Main goals

- performant
- stable
- usable
- modern
- stylish

## Non-goals

- replacing ChemDraw
- making money
