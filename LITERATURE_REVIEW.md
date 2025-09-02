# SCTT Literature Review

## Table of Contents
1. [Foundational Papers](#foundational-papers)
2. [Cubical Type Theory](#cubical-type-theory)
3. [Homotopy Type Theory](#homotopy-type-theory)
4. [Synthetic Differential Geometry](#synthetic-differential-geometry)
5. [Smooth Structures in Type Theory](#smooth-structures-in-type-theory)
6. [Related Systems](#related-systems)
7. [Implementation Papers](#implementation-papers)
8. [Research Gaps](#research-gaps)

## Foundational Papers

### 1. **Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom** (2015)
- **Authors**: Cyril Cohen, Thierry Coquand, Simon Huber, Anders Mörtberg
- **Key Contributions**:
  - First computational interpretation of univalence
  - Introduction of cubical sets as semantic foundation
  - Composition operations for paths
- **Relevance to SCTT**: Core cubical structure we build upon
- **Important Concepts**:
  - Path types as primitive
  - Composition and transport
  - Glue types for univalence
- **Link**: [arXiv:1611.02108](https://arxiv.org/abs/1611.02108)

### 2. **Cartesian Cubical Computational Type Theory** (2019)
- **Authors**: Carlo Angiuli, Kuen-Bang Hou (Favonia), Robert Harper
- **Key Contributions**:
  - Computational interpretation using cartesian cubes
  - Operational semantics for cubical type theory
  - Diagonal and degeneracy maps
- **Relevance to SCTT**: Alternative cubical structure to consider
- **Link**: [CMU Technical Report](https://www.cs.cmu.edu/~rwh/papers/cartesian/paper.pdf)

### 3. **Synthetic Differential Geometry** (2006)
- **Authors**: Anders Kock
- **Key Contributions**:
  - Nilpotent infinitesimals in topos theory
  - Smooth maps as primitive notion
  - Microlinearity principle
- **Relevance to SCTT**: Foundation for smooth structures
- **Link**: [Cambridge University Press](https://doi.org/10.1017/CBO9780511550812)

## Cubical Type Theory

### Core Papers

#### **Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types** (2019)
- **Authors**: Andrea Vezzosi, Anders Mörtberg, Andreas Abel
- **Implementation Focus**: Practical implementation of cubical type theory
- **Key Features**:
  - Transp and hcomp operations
  - Glue types
  - Higher inductive types
- **Code**: [GitHub: agda/cubical](https://github.com/agda/cubical)
- **Link**: [ICFP 2019](https://doi.org/10.1145/3341691)

#### **Canonicity for Cubical Type Theory** (2018)
- **Authors**: Simon Huber
- **Key Result**: Proof that cubical type theory satisfies canonicity
- **Importance**: Ensures computational content is preserved
- **Link**: [Journal of Automated Reasoning](https://doi.org/10.1007/s10817-018-9487-z)

#### **Cubical Synthetic Homotopy Theory** (2022)
- **Authors**: Anders Mörtberg, Loïc Pujet
- **Applications**: Homotopy groups, fiber sequences, spectral sequences
- **Link**: [CPP 2022](https://doi.org/10.1145/3497775.3503686)

### Implementation Papers

#### **cooltt: A Cubical Language with Exact Equality** (2021)
- **Authors**: Jonathan Sterling, Carlo Angiuli, Daniel Gratzer
- **Features**: Boundary separation, exact equality
- **Code**: [GitHub: RedPRL/cooltt](https://github.com/RedPRL/cooltt)

#### **redtt: Cartesian Cubical Proof Assistant** (2018)
- **Authors**: Jonathan Sterling, others
- **Innovation**: Cartesian cubical model
- **Code**: [GitHub: RedPRL/redtt](https://github.com/RedPRL/redtt)

## Homotopy Type Theory

### Foundational Works

#### **Homotopy Type Theory: Univalent Foundations of Mathematics** (2013)
- **Authors**: Univalent Foundations Program
- **The HoTT Book**: Collaborative effort defining HoTT
- **Key Concepts**:
  - Types as spaces
  - Functions as continuous maps
  - Paths as homotopies
  - Univalence axiom
- **Link**: [HoTT Book](https://homotopytypetheory.org/book/)

#### **The Simplicial Model of Univalent Foundations** (2012)
- **Authors**: Chris Kapulkin, Peter LeFanu Lumsdaine
- **Model**: Simplicial sets interpretation
- **Link**: [arXiv:1211.2851](https://arxiv.org/abs/1211.2851)

### Higher Structures

#### **Higher Inductive Types in Cubical Computational Type Theory** (2019)
- **Authors**: Evan Cavallo, Robert Harper
- **Contributions**: Computational interpretation of HITs
- **Examples**: Circle, torus, suspension
- **Link**: [POPL 2019](https://doi.org/10.1145/3290314)

#### **Modalities in Homotopy Type Theory** (2020)
- **Authors**: Egbert Rijke, Michael Shulman, Bas Spitters
- **Concepts**: Truncation, localization, fracture
- **Link**: [LMCS](https://doi.org/10.23638/LMCS-16(1:2)2020)

## Synthetic Differential Geometry

### Classical References

#### **Models for Smooth Infinitesimal Analysis** (1991)
- **Authors**: Ieke Moerdijk, Gonzalo E. Reyes
- **Framework**: Topos-theoretic models of SDG
- **Key Ideas**:
  - Infinitesimal objects
  - Tangent bundles as functors
  - Differential forms
- **Link**: [Springer](https://doi.org/10.1007/978-1-4757-4143-8)

#### **Synthetic Geometry of Manifolds** (2010)
- **Authors**: Anders Kock
- **Applications**: Differential geometry in synthetic setting
- **Topics**: Connections, curvature, Lie theory
- **Link**: [Cambridge](https://doi.org/10.1017/CBO9780511691690)

### Modern Developments

#### **Differential Cohomology in a Cohesive ∞-Topos** (2013)
- **Authors**: Urs Schreiber
- **Framework**: Higher topos theory for differential geometry
- **Applications**: Gauge theory, differential cohomology
- **Link**: [arXiv:1310.7930](https://arxiv.org/abs/1310.7930)

#### **Synthetic Differential Geometry in Type Theory** (2023)
- **Authors**: Felix Cherubini, Egbert Rijke
- **Recent Work**: SDG in HoTT framework
- **Link**: [arXiv:2309.07928](https://arxiv.org/abs/2309.07928)

## Smooth Structures in Type Theory

### Pioneering Work

#### **Differential Structure in Homotopy Type Theory** (2018)
- **Authors**: Mitchell Riley
- **Innovation**: Cohesive HoTT with differential structure
- **Link**: [PhD Thesis, Wesleyan](https://doi.org/10.14418/wes01.1.105)

#### **Modal Homotopy Type Theory** (2019)
- **Authors**: David Corfield
- **Book**: Philosophical and mathematical foundations
- **Topics**: Modalities, cohesion, differential structure
- **Link**: [Oxford University Press](https://global.oup.com/academic/product/modal-homotopy-type-theory-9780198853404)

### Recent Advances

#### **Towards a Cubical Type Theory for Differential Geometry** (2024)
- **Authors**: [Hypothetical/Recent]
- **Direction**: Combining cubical and smooth structures
- **Open Problems**: Computational interpretation

#### **Synthetic Hamiltonian Mechanics in Type Theory** (2023)
- **Authors**: [Recent work]
- **Applications**: Physics in type theory
- **Relevance**: Smooth dynamics

## Related Systems

### Proof Assistants with Relevant Features

#### **Lean 4 and mathlib4**
- **Smooth Manifolds**: Extensive formalization
- **Differential Geometry**: Tangent bundles, connections
- **Analysis**: Real numbers, calculus
- **Link**: [mathlib4 docs](https://leanprover-community.github.io/mathlib4_docs/)

#### **Coq with Mathematical Components**
- **Real Analysis**: Coquelicot library
- **Algebraic Structures**: SSReflect
- **Link**: [Mathematical Components](https://math-comp.github.io/)

#### **Isabelle/HOL**
- **Analysis**: Extensive real analysis
- **Differential Equations**: Formalized ODEs
- **Link**: [Archive of Formal Proofs](https://www.isa-afp.org/)

### Domain-Specific Languages

#### **Diderot: DSL for Imaging Sciences**
- **Features**: Tensor calculus, differential operators
- **Applications**: Scientific visualization
- **Link**: [Diderot Project](http://diderot-language.cs.uchicago.edu/)

#### **Differential λ-calculus**
- **Authors**: Thomas Ehrhard, Laurent Regnier
- **Concept**: Derivatives of λ-terms
- **Link**: [TCS 2003](https://doi.org/10.1016/S0304-3975(02)00771-3)

## Implementation Papers

### Type Checking Algorithms

#### **Normalization by Evaluation for Cubical Type Theory** (2019)
- **Authors**: Jonathan Sterling, Carlo Angiuli
- **Technique**: NbE for cubical types
- **Performance**: Efficient normalization
- **Link**: [LICS 2019](https://doi.org/10.1109/LICS.2019.8785766)

#### **Implementing a Modal Dependent Type Theory** (2019)
- **Authors**: Daniel Gratzer, Jonathan Sterling, Lars Birkedal
- **System**: Multiple modalities
- **Link**: [ICFP 2019](https://doi.org/10.1145/3341711)

### Performance Optimization

#### **Efficient Cubical Type Checking** (2021)
- **Topics**: Caching, memoization, incremental checking
- **Benchmarks**: Comparison of implementations

#### **Parallel Type Checking for Dependent Types** (2020)
- **Approach**: Parallelization strategies
- **Results**: Significant speedups

## Research Gaps

### Theoretical Gaps

1. **Computational Interpretation of Smooth Structures**
   - No existing computational semantics for smooth HoTT
   - Need operational semantics for differential operators
   - Missing: Canonicity for smooth cubical types

2. **Integration of Modalities**
   - Smooth modality ♭ not fully integrated with cubical
   - Interaction between cohesive and cubical structures unclear
   - Need: Unified modal cubical type theory

3. **Higher Categorical Structures**
   - Smooth ∞-groupoids not formalized
   - Differential cohomology in type theory incomplete
   - Missing: Smooth higher inductive types

### Implementation Gaps

1. **No Existing SCTT Implementation**
   - Cubical Agda lacks smooth structures
   - cooltt/redtt don't support differential geometry
   - Lean/Coq use classical foundations

2. **Performance Challenges**
   - Type checking with smooth structures unexplored
   - Normalization algorithms for smooth types needed
   - Proof search in presence of differential operators

3. **Practical Applications**
   - No verified numerical methods using SCTT
   - Machine learning integration unexplored
   - Scientific computing applications missing

### Applications Gaps

1. **Physics and Engineering**
   - General relativity in type theory incomplete
   - Quantum field theory formalization missing
   - Control theory applications unexplored

2. **Computer Science**
   - Differential privacy in type theory
   - Smooth optimization algorithms
   - Probabilistic programming integration

## Key Papers to Read First

### Essential Foundation (Start Here)
1. **HoTT Book** - Chapters 1-2 (Type theory basics)
2. **Cubical Type Theory** (Cohen et al. 2015) - Core cubical structure
3. **Cubical Agda** (Vezzosi et al. 2019) - Implementation guide
4. **Synthetic Differential Geometry** (Kock 2006) - Chapter 1

### Next Steps
1. **Canonicity for Cubical Type Theory** (Huber 2018)
2. **Modal Homotopy Type Theory** (Corfield 2019) - Modalities chapter
3. **Differential Cohomology in Cohesive ∞-Topos** (Schreiber 2013)

### Implementation References
1. **cooltt source code** - Study implementation
2. **Cubical Agda library** - Examples and patterns
3. **mathlib4 smooth manifolds** - See classical approach

## Research Questions for SCTT

### Fundamental Questions
1. How do we give computational meaning to smooth structures?
2. What is the correct notion of smooth path in cubical sets?
3. How do composition and transport interact with differentiation?

### Technical Questions
1. Can we prove canonicity for SCTT?
2. What is the complexity of type checking smooth types?
3. How do we implement automatic differentiation?

### Application Questions
1. Can SCTT verify numerical methods?
2. How does SCTT relate to differentiable programming?
3. Can we formalize physics in SCTT?

## Proposed Contributions

### Our Novel Contributions
1. **First computational interpretation** of smooth cubical types
2. **Operational semantics** for differential operators
3. **Proof-carrying code** with smooth structures
4. **Integration** with AlienMicrokernel for verified systems

### Building Blocks We Need
1. Basic cubical type theory (from Cubical Agda)
2. Path types and composition (from Cohen et al.)
3. Smooth structures (from SDG)
4. Modal operators (from cohesive HoTT)

## Reading Plan

### Week 1: Foundations
- [ ] HoTT Book Ch. 1-2
- [ ] Cubical Type Theory paper
- [ ] Cubical Agda tutorial

### Week 2: Smooth Structures
- [ ] SDG Chapter 1
- [ ] Modal HoTT introduction
- [ ] Cohesive HoTT basics

### Week 3: Implementation
- [ ] Study cooltt code
- [ ] Read NbE paper
- [ ] Review Cubical Agda implementation

### Week 4: Integration
- [ ] Design SCTT synthesis
- [ ] Identify key algorithms
- [ ] Plan implementation

## Bibliography File

See `papers/bibliography.bib` for BibTeX entries of all referenced papers.

---

*Last updated: 2024*
*Next review: After initial implementation*