# The Smooth Cubical Type Theory Book

> A Revolutionary Foundation for Mathematics and Computation

## About This Book

This is the definitive guide to **Smooth Cubical Type Theory (SCTT)**, a revolutionary foundation that unifies:
- **Type Theory** — Mathematics as computation
- **Homotopy Theory** — Spaces and continuous transformations
- **Differential Geometry** — Smooth manifolds and calculus

SCTT enables verified scientific computing where every calculation comes with mathematical guarantees.

## 📖 Reading the Book

### Online (Recommended)
Visit **[sctt-book.github.io](https://sctt-book.github.io)** for the rendered HTML version with:
- Searchable content
- Syntax-highlighted code
- Interactive navigation
- Mobile-friendly design

### Locally
1. Install [mdBook](https://rust-lang.github.io/mdBook/):
   ```bash
   cargo install mdbook mdbook-katex mdbook-mermaid
   ```

2. Build and serve:
   ```bash
   cd book
   mdbook serve --open
   ```

3. View at `http://localhost:3000`

### PDF Version
Generate a PDF (requires [mdbook-pdf](https://github.com/HollowMan6/mdbook-pdf)):
```bash
cd book
mdbook build --open
# PDF will be in book/pdf/output.pdf
```

## 🎯 Quick Start for Different Readers

- **Students**: Start with [Preface](./preface.md), then Chapter 1-2
- **Researchers**: Preface → Chapters 3-6, 13-15
- **Engineers**: Preface → Chapters 4-5, 9-12
- **Implementers**: Preface → Chapters 7-10

See the [Preface](./preface.md) for detailed reading paths.

## 📚 Book Structure

### Part I: Foundations
- [Chapter 1](./chapter_01.md): Introduction
- [Chapter 2](./chapter_02.md): Type Theory Foundations
- [Chapter 3](./chapter_03.md): Cubical Structure

### Part II: Smooth Structure
- [Chapter 4](./chapter_04.md): Smooth Types
- [Chapter 5](./chapter_05.md): Differential Operators
- [Chapter 6](./chapter_06.md): Limitations and Challenges

### Part III: Formal System
- [Chapter 7](./chapter_07.md): SCTT Formal Rules
- [Chapter 8](./chapter_08.md): Metatheory

### Part IV: Implementation
- [Chapter 9](./chapter_09.md): Type Checking Algorithm
- [Chapter 10](./chapter_10.md): Programming in SCTT

### Part V: Applications
- [Chapter 11](./chapter_11.md): Scientific Computing
- [Chapter 12](./chapter_12.md): Physics and Engineering

### Part VI: Advanced Topics
- [Chapter 13](./chapter_13.md): Modal SCTT
- [Chapter 14](./chapter_14.md): Higher Categories
- [Chapter 15](./chapter_15.md): Future Directions

### Appendices
- [Appendix B](./appendix_b.md): Notation Guide
- [Bibliography](./bibliography.md): References

## 🛠️ Building from Source

### Prerequisites
```bash
# Install Rust (for mdbook)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install mdBook and plugins
cargo install mdbook
cargo install mdbook-katex    # For math rendering
cargo install mdbook-mermaid  # For diagrams
```

### Build Commands
```bash
# Build HTML version
cd book && mdbook build

# Serve locally with live reload
cd book && mdbook serve

# Watch for changes
cd book && mdbook watch

# Test all code blocks (if SCTT compiler is installed)
cd book && mdbook test
```

### Build Output
- HTML: `docs/` (for GitHub Pages)
- PDF: `book/pdf/output.pdf` (if mdbook-pdf installed)

## 🧪 Development

### File Organization
```
book/
├── book.toml           # mdBook configuration
├── SUMMARY.md          # Table of contents
├── preface.md          # Reading guide
├── chapter_*.md        # Main chapters
├── appendix_*.md       # Appendices
└── bibliography.md     # References
```

### Adding Content
1. Create/edit markdown files
2. Update [SUMMARY.md](./SUMMARY.md) if adding new chapters
3. Follow notation conventions in [Appendix B](./appendix_b.md)
4. Test locally with `mdbook serve`
5. Submit pull request

### Code Examples
Use SCTT syntax highlighting:
````markdown
```sctt
-- Your SCTT code here
f : ℝ → ℝ
f x = x² + 2*x + 1
```
````

Mark pseudocode clearly:
````markdown
```pseudocode
// Conceptual algorithm
algorithm solve_problem:
  ...
```
````

## 🤝 Contributing

We welcome contributions! Please:
1. Read the [Preface](./preface.md) to understand the book's philosophy
2. Check [open issues](https://github.com/tensorhusker/SCTT/issues)
3. Follow the existing style and notation
4. Add exercise solutions to Appendix C if applicable
5. Update cross-references if adding new sections
6. Test that mdbook builds successfully

### Areas Needing Help
- Exercise solutions (Appendix C)
- Diagrams and visualizations
- Running examples
- Proof details
- Code verification
- Typo fixes

## 📝 License

This book is licensed under [Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)](https://creativecommons.org/licenses/by-sa/4.0/).

You are free to:
- **Share** — copy and redistribute the material
- **Adapt** — remix, transform, and build upon the material

Under the following terms:
- **Attribution** — Give appropriate credit
- **ShareAlike** — Distribute derivatives under the same license

## 🙏 Acknowledgments

This book builds on foundational work by:
- Per Martin-Löf (dependent type theory)
- Vladimir Voevodsky (univalent foundations)
- Thierry Coquand (cubical type theory)
- Anders Kock (synthetic differential geometry)
- Urs Schreiber (higher differential geometry)

And the entire HoTT, cubical, and differential geometry communities.

Special thanks to:
- The Homotopy Type Theory community
- Cubical Agda developers
- Synthetic Differential Geometry pioneers
- All contributors and reviewers

## 🔗 Links

- **Main Repository**: https://github.com/tensorhusker/SCTT
- **Book Website**: https://sctt-book.github.io (when deployed)
- **Issue Tracker**: https://github.com/tensorhusker/SCTT/issues
- **Discussions**: https://github.com/tensorhusker/SCTT/discussions
- **nLab**: https://ncatlab.org/
- **HoTT Website**: https://homotopytypetheory.org/

## 📧 Contact

For questions, corrections, or suggestions:
- Open an [issue](https://github.com/tensorhusker/SCTT/issues)
- Start a [discussion](https://github.com/tensorhusker/SCTT/discussions)
- Email: sctt-book@googlegroups.com (if established)

---

**Status**: 📚 Active development | ✓ Core chapters complete | 🚧 Appendices in progress

**Last Updated**: 2025-10-11

*Begin reading: [Preface](./preface.md) →*