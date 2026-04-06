# Summary

[Preface](./preface.md)

# Part I: Foundations

- [Chapter 1: Introduction](./chapter_01.md)
  - [1.1 What is SCTT?](./chapter_01.md#what-is-sctt)
  - [1.2 Motivation and Applications](./chapter_01.md#motivation)
  - [1.3 Historical Context](./chapter_01.md#history)
  - [1.4 Reading Guide](./chapter_01.md#guide)

- [Chapter 2: Type Theory Foundations](./chapter_02.md)
  - [2.1 Types and Terms](./chapter_02.md#types-and-terms)
  - [2.2 Dependent Types](./chapter_02.md#dependent-types)
  - [2.3 Function Types](./chapter_02.md#function-types)
  - [2.4 Inductive Types](./chapter_02.md#inductive-types)
  - [2.5 Universe Hierarchy](./chapter_02.md#universes)
  - [Exercises](./chapter_02.md#exercises)

- [Chapter 3: Cubical Structure](./chapter_03.md)
  - [3.1 The Interval Type](./chapter_03.md#interval)
  - [3.2 Path Types](./chapter_03.md#paths)
  - [3.3 Composition and Transport](./chapter_03.md#composition)
  - [3.4 Higher Paths](./chapter_03.md#higher-paths)
  - [3.5 Univalence](./chapter_03.md#univalence)
  - [Exercises](./chapter_03.md#exercises)

# Part II: Smooth Structure

- [Chapter 4: Smooth Types](./chapter_04.md)
  - [4.1 Smooth Real Numbers](./chapter_04.md#smooth-reals)
  - [4.2 Smooth Functions](./chapter_04.md#smooth-functions)
  - [4.3 Tangent Bundles](./chapter_04.md#tangent-bundles)
  - [4.4 Manifolds](./chapter_04.md#manifolds)
  - [4.5 Smooth Paths](./chapter_04.md#smooth-paths)
  - [Exercises](./chapter_04.md#exercises)

- [Chapter 5: Differential Operators](./chapter_05.md)
  - [5.1 Differentiation](./chapter_05.md#differentiation)
  - [5.2 Chain Rule](./chapter_05.md#chain-rule)
  - [5.3 Integration](./chapter_05.md#integration)
  - [5.4 Differential Forms](./chapter_05.md#differential-forms)
  - [5.5 Stokes' Theorem](./chapter_05.md#stokes)
  - [Exercises](./chapter_05.md#exercises)

- [Chapter 6: Addressing Limitations and Challenges](./chapter_06.md)
  - [6.1 Fundamental Expressivity Limitations](./chapter_06.md#expressivity-limits)
  - [6.2 Computational Complexity Barriers](./chapter_06.md#complexity-barriers)
  - [6.3 Non-Smooth Phenomena](./chapter_06.md#non-smooth)
  - [6.4 Undecidability and Uncomputability](./chapter_06.md#undecidable)
  - [6.5 Practical Implementation Challenges](./chapter_06.md#implementation)
  - [6.6 Workarounds and Extensions](./chapter_06.md#workarounds)
  - [6.7 Topological Cryptography](./chapter_06.md#homotopical-crypto)

# Part III: Formal System

- [Chapter 7: SCTT Formal Rules](./chapter_07.md)
  - [7.1 Judgment Forms](./chapter_07.md#judgments)
  - [7.2 Type Formation Rules](./chapter_07.md#formation)
  - [7.3 Introduction Rules](./chapter_07.md#introduction)
  - [7.4 Elimination Rules](./chapter_07.md#elimination)
  - [7.5 Computation Rules](./chapter_07.md#computation)
  - [7.6 Equational Theories and Rewrite Rules](./chapter_07.md#rewrite-rules)
  - [7.7 Uniqueness Rules](./chapter_07.md#uniqueness)

- [Chapter 8: Metatheory](./chapter_08.md)
  - [8.1 Consistency](./chapter_08.md#consistency)
  - [8.2 Canonicity](./chapter_08.md#canonicity)
  - [8.3 Decidability](./chapter_08.md#decidability)
  - [8.4 Normalization](./chapter_08.md#normalization)
  - [8.5 Computational Complexity](./chapter_08.md#complexity)

# Part IV: Implementation

- [Chapter 9: Type Checking Algorithm](./chapter_09.md)
  - [9.1 Bidirectional Type Checking](./chapter_09.md#bidirectional)
  - [9.2 Constraint Solving](./chapter_09.md#constraints)
  - [9.3 Normalization by Evaluation](./chapter_09.md#nbe)
  - [9.4 Caching and Optimization](./chapter_09.md#optimization)
  - [Implementation](./chapter_09.md#implementation)

- [Chapter 10: Programming in SCTT](./chapter_10.md)
  - [10.1 Basic Programming](./chapter_10.md#basics)
  - [10.2 Proving Theorems](./chapter_10.md#proofs)
  - [10.3 Smooth Computations](./chapter_10.md#computations)
  - [10.4 Library Development](./chapter_10.md#libraries)
  - [10.5 Best Practices](./chapter_10.md#practices)

# Part V: Applications

- [Chapter 11: Scientific Computing](./chapter_11.md)
  - [11.1 Numerical Analysis](./chapter_11.md#numerical)
  - [11.2 Differential Equations](./chapter_11.md#diffeq)
  - [11.3 Optimization](./chapter_11.md#optimization)
  - [11.4 Machine Learning](./chapter_11.md#ml)

- [Chapter 12: Physics and Engineering](./chapter_12.md)
  - [12.1 Classical Mechanics](./chapter_12.md#classical)
  - [12.2 Quantum Mechanics](./chapter_12.md#quantum)
  - [12.3 General Relativity](./chapter_12.md#relativity)
  - [12.4 Control Theory](./chapter_12.md#control)

# Part VI: Advanced Topics

- [Chapter 13: Modal SCTT](./chapter_13.md)
  - [13.1 Modalities](./chapter_13.md#modalities)
  - [13.2 Cohesive Structure](./chapter_13.md#cohesion)
  - [13.3 Differential Cohomology](./chapter_13.md#cohomology)

- [Chapter 14: Higher Categories](./chapter_14.md)
  - [14.1 ∞-Groupoids](./chapter_14.md#groupoids)
  - [14.2 Smooth ∞-Groupoids](./chapter_14.md#smooth-groupoids)
  - [14.3 Higher Gauge Theory](./chapter_14.md#gauge)

- [Chapter 15: Open Problems and Research Frontiers](./chapter_15.md)
  - [15.1 Open Problems](./chapter_15.md#problems)
  - [15.2 Research Directions](./chapter_15.md#research)
  - [15.3 Vision](./chapter_15.md#vision)

# Part VII: Implementation Pathways

- [Chapter 16: Building the v0 Kernel](./chapter_16.md)
  - [16.1 Phase 1: Tiny Type Theory](./chapter_16.md#phase1)
  - [16.2 Phase 2: Cartesian Cubical Layer](./chapter_16.md#phase2)
  - [16.3 Phase 3: Smooth Primitives](./chapter_16.md#phase3)
  - [16.4 Phase 4: Lipschitz Sensitivity Types](./chapter_16.md#phase4)
  - [16.5 Testing and Verification](./chapter_16.md#testing)
  - [16.6 Architecture Decisions](./chapter_16.md#architecture)

- [Chapter 17: Certified Machine Learning](./chapter_17.md)
  - [17.1 Neural Networks as Typed Functions](./chapter_17.md#typed-networks)
  - [17.2 Certified Adversarial Robustness](./chapter_17.md#robustness)
  - [17.3 Neural ODEs as Smooth Paths](./chapter_17.md#neural-odes)
  - [17.4 Automatic Differentiation in SCTT](./chapter_17.md#autodiff)
  - [17.5 The Curry-Howard-Hinton Correspondence](./chapter_17.md#chh)
  - [17.6 Practical Implementation Pathway](./chapter_17.md#implementation)

- [Chapter 18: The Equational Theory Frontier](./chapter_18.md)
  - [18.1 Why Standard Rewriting Fails](./chapter_18.md#why-hard)
  - [18.2 RTT: The Foundation](./chapter_18.md#rtt)
  - [18.3 BiTTs: The Checking Algorithm](./chapter_18.md#bitts)
  - [18.4 LRTT: Local Scoping](./chapter_18.md#lrtt)
  - [18.5 The Gruissan Gap](./chapter_18.md#gruissan)
  - [18.6 Implementation Walkthrough](./chapter_18.md#implementation)
  - [18.7 Testing Your Rewrite Engine](./chapter_18.md#testing)

- [Chapter 19: Connecting to the Ecosystem](./chapter_19.md)
  - [19.1 Cubical Agda](./chapter_19.md#cubical-agda)
  - [19.2 Lean 4](./chapter_19.md#lean)
  - [19.3 Rocq and the Rewster](./chapter_19.md#rocq)
  - [19.4 Dedukti and Lambdapi](./chapter_19.md#dedukti)
  - [19.5 Two-Level Type Theory and Staging](./chapter_19.md#2ltt)
  - [19.6 The Translation Map](./chapter_19.md#translations)

# Appendices

- [Appendix A: Mathematical Background](./appendix_a.md)
- [Appendix B: Notation Guide](./appendix_b.md)
- [Appendix C: Solutions to Exercises](./appendix_c.md)
- [Appendix D: SCTT Standard Library](./appendix_d.md)

[Bibliography](./bibliography.md)
[Index](./index.md)
[About the Authors](./authors.md)