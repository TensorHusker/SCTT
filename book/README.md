# The Smooth Cubical Type Theory Book

## About This Book

This is the definitive guide to Smooth Cubical Type Theory (SCTT), a revolutionary foundation for mathematics and computation that unifies homotopy type theory with differential geometry.

## How to Read This Book

### For Beginners
Start with Chapter 1 for motivation, then carefully work through Chapters 2-4 to build foundations.

### For Type Theorists
You may skip Chapter 2 and focus on Chapters 3-5 where we introduce the cubical and smooth structures.

### For Differential Geometers
Chapter 4-5 will be of particular interest, showing how smooth structures emerge from type theory.

### For Implementers
Chapters 7-9 provide practical implementation guidance and examples.

## Building the Book

```bash
# HTML version
mdbook build

# PDF version
pandoc book.md -o sctt-book.pdf

# Interactive version
jupyter-book build .
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to the book.

## License

This book is licensed under CC BY-SA 4.0. You are free to share and adapt the material with attribution.

## Acknowledgments

This book builds on the foundational work of the Homotopy Type Theory community, the developers of Cubical Agda, and pioneers of Synthetic Differential Geometry.