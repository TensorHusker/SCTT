# Preface

> "The purpose of abstraction is not to be vague, but to create a new semantic level in which one can be absolutely precise." — Edsger W. Dijkstra

## What is This Book?

This book introduces **Smooth Cubical Type Theory (SCTT)**, a revolutionary mathematical foundation that unifies three powerful frameworks:

1. **Type Theory** — Mathematics as computation
2. **Homotopy Theory** — Spaces and continuous transformations
3. **Differential Geometry** — Smooth manifolds and calculus

SCTT enables verified scientific computing where every calculation comes with mathematical guarantees. It makes differentiation computable, paths first-class, and proofs executable.

## Who Should Read This Book?

This book is written for multiple audiences. Choose your path:

### 🎓 For Students & Newcomers

**You want to**: Learn SCTT from scratch

**Prerequisites**:
- Basic programming (any language)
- Calculus I & II
- Mathematical maturity (comfortable reading proofs)

**Your path**:
1. Read Chapter 1 completely for motivation
2. Work through Chapter 2 carefully (foundations)
3. Study Chapters 3-5 (core theory)
4. Practice with Chapter 10 (programming)
5. Explore applications in Chapters 11-12

**Time investment**: 6-8 weeks at 10-15 hours/week

**Exercises**: Start with ★ exercises, work up to ★★★

---

### 🔬 For Researchers (Mathematics/CS)

**You want to**: Understand the theoretical foundations and open problems

**Prerequisites**:
- Familiarity with type theory OR category theory
- Differential geometry (manifolds, tangent bundles)
- Comfort with formal systems

**Your path**:
1. Skim Chapter 1 for context
2. Review Chapter 2 if needed (type theory basics)
3. Deep dive: Chapters 3-6 (cubical + smooth structures)
4. Study Chapters 7-8 (formal system + metatheory)
5. Explore Chapters 13-15 (advanced topics + open problems)

**Focus on**: Theorems, proofs, formal rules, open conjectures

**Skip**: Implementation details (Chapter 9) unless interested

---

### 💻 For Software Engineers & Implementers

**You want to**: Build tools and applications using SCTT

**Prerequisites**:
- Strong programming skills (Rust/OCaml/Haskell helpful)
- Type systems knowledge
- Algorithms & data structures

**Your path**:
1. Read Chapter 1 for the vision
2. Skim Chapter 2 (refresh type theory)
3. Understand Chapters 3-5 (what SCTT provides)
4. **Deep dive**: Chapter 9 (type checking algorithms)
5. **Deep dive**: Chapter 10 (programming patterns)
6. Review Chapters 11-12 for application ideas

**Focus on**: Code examples, algorithms, implementation strategies

**Reference**: Chapters 7-8 when you need formal details

---

### 🔧 For Scientists & Engineers

**You want to**: Apply SCTT to real-world problems

**Prerequisites**:
- Domain expertise (physics, ML, control theory, etc.)
- Numerical methods
- Scientific computing experience

**Your path**:
1. Read Chapter 1 (motivation)
2. **Skim** Chapters 2-3 (enough to understand syntax)
3. **Focus**: Chapter 4-5 (smooth types + calculus)
4. **Deep dive**: Chapter 11 (scientific computing)
5. **Deep dive**: Chapter 12 (physics & engineering)
6. Explore Chapter 10 for programming when needed

**Focus on**: Applications, verified algorithms, case studies

**Skip**: Chapters 13-15 (unless pursuing research)

---

### 🧠 For Philosophers & Logicians

**You want to**: Understand foundational questions

**Prerequisites**:
- Mathematical logic
- Philosophy of mathematics
- Constructive mathematics

**Your path**:
1. Chapter 1 (philosophical context)
2. Chapter 2 (constructive foundations)
3. Chapter 3 (identity and homotopy)
4. Chapter 6 (limitations and boundaries)
5. Chapter 8 (metatheory and consistency)
6. Chapter 15 (future directions)

**Focus on**: Foundations, constructivity, what can/cannot be expressed

---

## How to Use This Book

### 📖 Reading Conventions

**Code Blocks**:
```sctt
-- This is executable SCTT code
-- Try it in the SCTT playground
f : ℝ → ℝ
f x = x² + 2*x + 1
```

**Pseudocode** (marked clearly):
```pseudocode
// Conceptual algorithm, not executable
algorithm solve_problem:
  step 1: ...
  step 2: ...
```

**Tested Code**: Look for ✓ markers indicating verified examples

---

### 🎯 Learning Strategies

**Active Reading**:
- Work through exercises (solutions in Appendix C)
- Type out code examples
- Prove simple theorems yourself
- Build small projects

**Concept Mapping**:
- Draw connections between chapters
- Relate new concepts to familiar ones
- Use the running examples as anchors

**Spaced Repetition**:
- Review previous chapters before moving forward
- Revisit difficult concepts after a few days
- Use the notation guide (Appendix B) frequently

---

### 💡 Special Features

Throughout the book, you'll find:

**Quick Start Boxes**: Get oriented quickly
```
⚡ Quick Start: This section covers X. Key takeaway: Y.
Skip if you already know Z.
```

**Deep Dives**: Optional advanced material
```
🔍 Deep Dive: Advanced topic for researchers
```

**Practical Tips**: Implementation guidance
```
💻 Implementation Note: Watch out for X when coding Y
```

**Historical Context**: Mathematical lineage
```
📜 Historical Note: This idea came from X (1950)
```

**Open Problems**: Research frontiers
```
🔬 Open Problem: We don't yet know whether X...
```

---

## Structure of the Book

### Part I: Foundations (Chapters 1-3)
The bedrock: type theory and cubical structure

### Part II: Smooth Structure (Chapters 4-6)
Adding differential geometry to types

### Part III: Formal System (Chapters 7-8)
Rigorous rules and metatheory

### Part IV: Implementation (Chapters 9-10)
Making SCTT real and practical

### Part V: Applications (Chapters 11-12)
Scientific computing and engineering

### Part VI: Advanced Topics (Chapters 13-15)
Cutting edge and future directions

---

## Prerequisites Summary

### Essential for Everyone
- Mathematical maturity (comfortable with proofs)
- Basic calculus (derivatives, integrals)
- Some programming experience

### Helpful But Not Required
- Type theory or functional programming
- Differential geometry
- Category theory
- Homotopy theory

**Missing background?** See Appendix A for quick reviews.

---

## How Long Will This Take?

**Quick overview**: 10-15 hours (read selectively)
**Working knowledge**: 40-60 hours (with exercises)
**Deep mastery**: 100-150 hours (everything + projects)
**Research level**: 200+ hours (including advanced topics)

**Recommended pace**:
- 1 chapter per week for careful study
- 2-3 chapters per week for overview
- Adjust based on your background

---

## Getting Help

### Resources
- 📚 **Appendix B**: Notation guide (look up symbols)
- 📚 **Appendix C**: Exercise solutions
- 🌐 **Online**: sctt-lang.org (documentation)
- 💬 **Community**: GitHub discussions, forum

### When You're Stuck
1. Check the notation guide (Appendix B)
2. Review prerequisite chapters
3. Try simpler examples first
4. Ask in the community forum
5. Read related research papers (bibliography)

---

## A Note on Difficulty

SCTT is genuinely challenging. It combines:
- Abstract mathematics (type theory, homotopy)
- Concrete computation (algorithms, programming)
- Physical intuition (differential geometry)

**This is normal**. Even experts find parts difficult.

**Strategy**:
- Don't expect to understand everything on first read
- Build intuition with examples before diving into proofs
- Take breaks when frustrated
- Return to difficult sections later with fresh perspective

---

## Why SCTT Matters

Traditional scientific computing lacks mathematical guarantees. Simulations might be wrong, optimizations might fail, models might be flawed—and we often don't know until disaster strikes.

SCTT changes this paradigm:

- **Every computation** has a correctness proof
- **Every simulation** has error bounds
- **Every optimization** has convergence guarantees
- **Every model** has consistency certificates

This isn't just academic—it's essential for:
- Safe autonomous systems
- Trusted AI/ML models
- Reliable scientific discoveries
- Critical infrastructure

---

## Acknowledgments

SCTT stands on the shoulders of giants:
- Per Martin-Löf (dependent type theory)
- Vladimir Voevodsky (univalent foundations)
- Thierry Coquand (cubical type theory)
- Anders Kock (synthetic differential geometry)
- Urs Schreiber (higher differential geometry)

And the entire HoTT and cubical communities.

---

## Let's Begin

Mathematics and computation are converging. Type theory and differential geometry are unifying. Proofs and programs are becoming one.

Welcome to the smooth cubical revolution.

**Choose your path above and start reading!**

---

*Next: [Chapter 1: Introduction](./chapter_01.md) →*
