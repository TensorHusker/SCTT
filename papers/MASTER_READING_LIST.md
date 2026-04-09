# SCTT x ARC-AGI: Master Reading List

> 200+ references organized by domain with reading order, priority, and relevance
> to the thesis: **Smooth Cubical Type Theory IS the abstract reasoning engine.**

---

## Reading Curriculum (10-Week Track)

### Weeks 1-2: Foundations
1. Martin-Lof 1984 (Intuitionistic Type Theory) + HoTT Book Ch.1-3
2. Nordstrom et al. 1990 (Programming in MLTT)
3. Mac Lane 1998 Ch.1-5 (Category Theory basics)
4. Coquand & Huet 1988 (Calculus of Constructions)

### Weeks 3-4: Cubical
5. CCHM 2018 (Cubical Type Theory)
6. ABCFHL 2021 (Cartesian Cubical)
7. Kovacs 2024 (cctt implementation)
8. Sterling & Angiuli 2021 (Normalization for Cubical TT)

### Weeks 5-6: Smooth
9. Kock 2006 (SDG, 2nd edition)
10. Lavendhomme 1996 (Basics of SDG)
11. Shulman 2018 (Cohesive HoTT)
12. Moerdijk & Reyes 1991 (Models for Smooth Infinitesimal Analysis)

### Weeks 7-8: ARC + Program Synthesis
13. Chollet 2019 ("On the Measure of Intelligence")
14. Ellis et al. 2021 (DreamCoder)
15. Lake et al. 2017 (Building Machines That Learn and Think Like People)
16. Greenblatt 2024 (Solving ARC with GPT-4o)

### Weeks 9-10: Integration
17. Elliott 2018 (Simple Essence of AD)
18. Fong et al. 2019 (Backprop as Functor)
19. Reed & Pierce 2010 (Distance Makes Types Grow Stronger)
20. Chen et al. 2018 (Neural ODEs)

---

## Domain 1: Type Theory Foundations [CRITICAL]

### Already in bibliography
- Martin-Lof, Per. *Intuitionistic Type Theory*. Bibliopolis, 1984.
- Martin-Lof, Per. "An Intuitionistic Theory of Types: Predicative Part." Logic Colloquium '73, 1975.
- Nordstrom, Bengt; Petersson, Kent; Smith, Jan M. *Programming in Martin-Lof's Type Theory*. OUP, 1990.
- Coquand, Thierry; Huet, Gerard. "The Calculus of Constructions." Info. & Comp., 76(2-3):95-120, 1988.

### ADD
- Martin-Lof, Per. "Constructive Mathematics and Computer Programming." In: Logic, Methodology and Philosophy of Science VI, 1982. *The philosophical foundation of constructive type theory.*
- Coquand, Thierry. "An Analysis of Girard's Paradox." In: LICS 1986. *Universe consistency — relevant to bounded universes in TTT.*
- Dybjer, Peter. "Internal Type Theory." In: TYPES 1995, LNCS 1158, 1996. *Type theory bootstrapping itself — v0 resonance.*
- Dybjer, Peter. "A General Formulation of Simultaneous Inductive-Recursive Definitions in Type Theory." JSL, 65(2):525-549, 2000. *Foundation for inductive-recursive types.*
- McBride, Conor; McKinna, James. "The View from the Left." JFP, 14(1):69-111, 2004. *Dependent pattern matching — needed for ARC grid analysis.*
- Norell, Ulf. "Dependently Typed Programming in Agda." AFP 2008 Lecture Notes, 2009. *Practical dependent programming.*
- Altenkirch, Thorsten; McBride, Conor; Swierstra, Wouter. "Observational Equality, Now!" In: PLDI 2007. *Equality without axioms — precursor to OTT/cubical.*
- Rijke, Egbert. *Introduction to Homotopy Type Theory*. Cambridge University Press, 2025. *Modern textbook, bridges HoTT and cubical.*
- Brady, Edwin. "Idris 2: Quantitative Type Theory in Practice." In: ECOOP 2021. *QTT implementation — resource-aware types.*
- Atkey, Robert. "The Syntax and Semantics of Quantitative Type Theory." In: LICS 2018. *Quantitative annotations on types — bridges to Lipschitz.*

---

## Domain 2: Cubical Type Theory [CRITICAL]

### Already in bibliography
- Cohen, Cyril; Coquand, Thierry; Huber, Simon; Mortberg, Anders. "Cubical Type Theory." JAR, 60(2):129-172, 2018.
- Angiuli, Carlo et al. (ABCFHL). "Syntax and Models of Cartesian Cubical Type Theory." MSCS, 31(4):424-468, 2021.
- Huber, Simon. "Canonicity for Cubical Type Theory." JAR, 63:173-210, 2019.
- Kovacs, Andras. "cctt: A Performant Elaborator for Cubical Type Theory." 2024.
- Sterling, Jonathan; Angiuli, Carlo. "Normalization for Cubical Type Theory." LICS 2021.
- Huang, Yiyang. "Normal Forms for Cartesian Cubical Type Theory." arXiv:2603.24923, 2026.
- Brunerie, Guillaume. *On the Homotopy Groups of Spheres in HoTT*. PhD, Nice, 2016.
- Vezzosi, Andrea; Mortberg, Anders; Abel, Andreas. "Cubical Agda." ICFP 2019.

### ADD
- Cavallo, Evan. *Higher Inductive Types and Internal Parametricity for Cubical Type Theory*. PhD, CMU, 2021. *HITs + parametricity in the cubical setting.*
- Cavallo, Evan; Harper, Robert. "Higher Inductive Types in Cubical Computational Type Theory." In: POPL 2019. *Computational rules for HITs.*
- Pujet, Loic; Tabareau, Nicolas. "Observational Equality: Now For Good." In: POPL 2022. *Propositional equality without axioms — clean equality for grids.*
- Jack, Tom; Licata, Daniel R.; Harper, Robert. "Internal Parametricity, without an Interval." In: POPL 2024. *Parametricity without extra dimensions.*
- Swan, Andrew. "Separating Path and Identity Types in Presheaf Models of Univalent Type Theory." arXiv:1808.00920, 2018.
- Coquand, Thierry; Huber, Simon; Sattler, Christian. "Homotopy Canonicity for Cubical Type Theory." In: FSCD 2019. *Canonicity proof for CCHM cubical TT.*
- Orton, Ian; Pitts, Andrew M. "Axioms for Modelling Cubical Type Theory in a Topos." In: CSL 2016. *Topos-theoretic models for cubical structure.*
- Gratzer, Daniel; Sterling, Jonathan; Birkedal, Lars. "Implementing a Modal Dependent Type Theory." In: ICFP 2019. *Modal TT implementation — relevant for cohesive modalities.*
- Dore, Maximilian; Cavallo, Evan; Mortberg, Anders. "Automating Boundary Filling in Cubical Type Theories." In: FSCD 2024. *Automation of Kan ops — directly relevant to solver.*
- Awodey, Steve. "A Cartesian Cubical Model Structure." Springer, 2026. *Model-theoretic foundations for Cartesian cubical.*

---

## Domain 3: Homotopy Type Theory [HIGH]

### Already in bibliography
- The Univalent Foundations Program. *Homotopy Type Theory*. IAS, 2013.
- Voevodsky, Vladimir. "A Very Short Note on Homotopy lambda-Calculus." 2006.
- Voevodsky, Vladimir. "Univalent Foundations Project." 2010.
- Awodey, Steve; Warren, Michael A. "Homotopy Theoretic Models of Identity Types." 2009.
- Riehl, Emily; Shulman, Michael. "A Type Theory for Synthetic infinity-Categories." 2017.

### ADD
- Lumsdaine, Peter. "Weak omega-Categories from Intensional Type Theory." In: TLCA 2009. *Higher-categorical semantics from TT.*
- Kapulkin, Krzysztof; Lumsdaine, Peter. "The Simplicial Model of Univalent Foundations (after Voevodsky)." JEMS, 2021. *Reference simplicial model.*
- Kraus, Nicolai. *Truncation Levels in Homotopy Type Theory*. PhD, Nottingham, 2015. *Truncation theory — simplification for ARC grids.*
- van Doorn, Floris. *On the Formalization of Higher Inductive Types and Synthetic Homotopy Theory*. PhD, CMU, 2018. *Mechanized HoTT.*
- Hou (Favonia), Kuen-Bang. *Higher-Dimensional Types in the Mechanization of Homotopy Theory*. PhD, CMU, 2017. *Computational higher dimensions.*
- Altenkirch, Thorsten; Kaposi, Ambrus. "Type Theory in Type Theory using Quotient Inductive Types." In: POPL 2016. *Self-referential TT — bootstrap pattern.*
- Licata, Daniel R.; Finster, Eric. "Eilenberg-MacLane Spaces in Homotopy Type Theory." In: LICS 2014. *Higher algebraic structures in HoTT.*
- Shulman, Michael. "All (infinity,1)-Toposes Have Strict Univalent Universes." arXiv:1904.07004, 2019. *Universal model property.*
- Buchholtz, Ulrik; van Doorn, Floris; Rijke, Egbert. "Higher Groups in Homotopy Type Theory." In: LICS 2018. *Higher symmetry structures.*

---

## Domain 4: Synthetic Differential Geometry [CRITICAL for SCTT]

### Already in bibliography
- Kock, Anders. *Synthetic Differential Geometry (2nd ed.)*. Cambridge, 2006.
- Kock, Anders. "Convenient Vector Spaces Embed into the Cahiers Topos." 1986.
- Lawvere, F. William. "Categorical Dynamics." Topos Theoretic Methods, 1979.
- Moerdijk, Ieke; Reyes, Gonzalo E. *Models for Smooth Infinitesimal Analysis*. Springer, 1991.
- Lavendhomme, Rene. *Basic Concepts of SDG*. Kluwer, 1996.
- Bell, John L. *A Primer of Infinitesimal Analysis (2nd ed.)*. Cambridge, 2008.

### ADD
- Kock, Anders. *Synthetic Geometry of Manifolds*. Cambridge, 2010. *The sequel — Lie groups, connections, bundles via SDG.*
- Kock, Anders. "Differential Forms with Values in Groups." Bull. Austral. Math. Soc., 25:357-386, 1982. *Differential forms in SDG framework.*
- Dubuc, Eduardo. "Sur les modeles de la geometrie differentielle synthetique." Cahiers, 20(3):231-279, 1979. *Well-adapted models — foundations.*
- Dubuc, Eduardo. "C-infinity Schemes." Amer. J. Math., 103(4):683-690, 1981. *Algebraic geometry meets SDG.*
- Reyes, Gonzalo. "A Derivation of Einstein's Vacuum Field Equations." SDG Methods, 2008. *SDG applied to physics — shows the framework's power.*
- Bell, John L. "Infinitesimals and the Continuum." Mathematical Intelligencer, 17(2):55-57, 1995. *Philosophical perspective on infinitesimals.*
- Kock, Anders. "The Mutual Embrace of Algebraic and Differential Geometry." Aarhus preprint, 1985.
- Moerdijk, Ieke; Reyes, Gonzalo. "Rings of Smooth Functions and their Localizations, I." J. Algebra, 99:324-336, 1986. *Algebraic foundations of smooth models.*
- Lawvere, F. William. "Toposes of Laws of Motion." Presented at Montreal, 1997. *Foundational for dynamics in SDG.*

---

## Domain 5: Differential Cohesion & Smooth infinity-Topoi [HIGH]

### Already in bibliography
- Schreiber, Urs. *Differential cohomology in a cohesive infinity-topos*. Habilitation, 2011.
- Schreiber, Urs; Shulman, Michael. "Quantum Gauge Field Theory in Cohesive HoTT." QPL, 2012.
- Shulman, Michael. "Brouwer's Fixed-Point Theorem in Real-Cohesive HoTT." MSCS, 2018.
- Wellen, Felix. *Formalizing Cartan Geometry in Modal HoTT*. PhD, KIT, 2017.

### ADD
- Schreiber, Urs. "Quantization via Linear Homotopy Types." arXiv:1402.7041, 2014. *Quantization in cohesive HoTT framework.*
- Wellen, Felix. "Flat Connections and Modal Homotopy Type Theory." 2018. *Connections as modal types.*
- Myers, David Jaz. "Orbifolds as Microlinear Types in Synthetic Differential Geometry." 2019. *Singularities via SDG.*
- Myers, David Jaz. "Good Fibrations through the Modal Prism." 2019. *Fibrations in cohesive setting.*
- Shulman, Michael. "Cohesive Homotopy Type Theory." Talk notes, HoTT 2012. *Accessible introduction to cohesive modalities.*
- Licata, Daniel R.; Shulman, Michael. "Adjoint Logic with a 2-Category of Modes." In: LICS 2016. *Modal type theory foundations.*
- Riley, Mitchell; Finster, Eric; Licata, Daniel R. "Synthetic Spectra via a Monadic and Comonadic Modality." arXiv:2102.04099, 2021. *Spectra in modal HoTT.*
- Anel, Mathieu; Biedermann, Georg; Finster, Eric; Joyal, Andre. "Goodwillie's Calculus of Functors and Higher Topos Theory." J. Topology, 2018. *Functor calculus — higher abstraction.*

---

## Domain 6: Abstract Reasoning & ARC [CRITICAL for Submission]

### NEW DOMAIN — No existing references

- Chollet, Francois. "On the Measure of Intelligence." arXiv:1911.01547, 2019. **THE foundational ARC paper.** *Defines intelligence as skill-acquisition efficiency.*
- Chollet, Francois et al. "ARC Prize 2024: Technical Report." arXiv:2412.04604, 2024. *Competition specifics, evaluation methodology.*
- Chollet, Francois et al. "ARC Prize 2025: Technical Report." arXiv:2601.10904, 2025. *Updated competition, ARC-AGI-2 results.*
- Chollet, Francois et al. "ARC-AGI-3." arcprize.org, 2026. *Interactive reasoning benchmark — the target.*
- Johnson, Jack et al. "The ARC of Progress towards AGI: A Living Survey." arXiv:2603.13372, 2026. *Comprehensive survey of 82 approaches across 3 ARC versions.*
- Greenblatt, Ryan. "Getting 50% (SoTA) on ARC-AGI with GPT-4o." Redwood Research Blog, 2024. *Program synthesis via LLM — 8000 candidate programs per task.*
- Xu, Yudong et al. "Graphs, Constraints, and Search for the Abstraction and Reasoning Corpus." arXiv:2210.09880, 2023. *Graph-based ARC solving.*
- Ainooson, James et al. "An Approach to Solving the ARC Challenge." arXiv:2302.09738, 2023. *DSL-based ARC approach.*
- Hodel, Michael et al. "Addressing the Abstraction and Reasoning Corpus via Procedural Example Generation." 2024. *Data augmentation for ARC.*
- Ouellette, Simon et al. "RE-ARC: Reverse-Engineering the ARC." 2024. *Procedural generation of ARC tasks.*
- Acquaviva, Sam et al. "Communicating Natural Programs to Humans and Machines." NeurIPS 2022. *LARC — natural language ARC annotations.*
- Moskvichev, Arseny et al. "The ConceptARC Benchmark: Evaluating Understanding and Generalization in the ARC Domain." arXiv:2305.07141, 2023. *Concept-level ARC analysis.*
- Ferreira, Joao et al. "Neural Networks for Abstraction and Reasoning: Towards Broad Generalization in Machines." Scientific Reports, 14:25469, 2024. *DreamCoder+PeARL for ARC, 3x improvement.*
- Banino, Andrea et al. "Tackling the Abstraction and Reasoning Corpus with Object-Centric Models and the MDL Principle." In: AAAI 2024. *Object-centric + MDL for ARC.*
- Cole, Jack et al. "ARCLE: ARC Learning Environment." 2024. *RL environment for ARC tasks.*
- Park, Jongho et al. "ARC-NCA: Towards Developmental Solutions to the ARC." arXiv:2505.08778, 2025. *Neural cellular automata for ARC.*
- Johnson, Tristan et al. "Boosting Performance on ARC is a Matter of Perspective." arXiv:2505.07859, 2025. *Perspective transforms improve ARC accuracy.*
- Xu, Yudong et al. "A Comprehensive Behavioral Dataset for the ARC." Scientific Data, 2025. *Human behavioral data on ARC tasks.*

---

## Domain 7: Program Synthesis & Inductive Logic [HIGH]

### NEW DOMAIN — No existing references

- Ellis, Kevin et al. "DreamCoder: Bootstrapping Inductive Program Synthesis with Wake-Sleep Library Learning." In: PLDI 2021. **Key paper — library learning IS type abstraction.** *Wake-sleep builds libraries of reusable abstractions.*
- Ellis, Kevin et al. "Learning to Infer Graphics Programs from Hand-Drawn Images." NeurIPS 2018. *Visual program synthesis.*
- Balog, Matej et al. "DeepCoder: Learning to Write Programs." In: ICLR 2017. *Neural-guided program synthesis.*
- Cropper, Andrew; Morel, Rolf. "Learning Programs by Learning from Failures." Machine Learning, 110:801-856, 2021. *Inductive logic programming.*
- Lake, Brenden M. et al. "Building Machines That Learn and Think Like People." BBS, 40, 2017. *Core knowledge priors — maps to ARC requirements.*
- Lake, Brenden M. et al. "Human-Level Concept Learning through Probabilistic Program Induction." Science, 350(6266):1332-1338, 2015. *Bayesian program learning — one-shot.*
- Hernandez-Orallo, Jose. *The Measure of All Minds*. Cambridge, 2017. *Universal intelligence measurement — theoretical frame for ARC.*
- Gulwani, Sumit. "Automating String Processing in Spreadsheets Using Input-Output Examples." In: POPL 2011. *FlashFill — PBE pioneer.*
- Singh, Rishabh; Gulwani, Sumit. "Predicting a Correct Program in Programming by Example." In: CAV 2015. *Ranking synthesized programs.*
- Osera, Peter-Michael; Zdancewic, Steve. "Type-and-Example-Directed Program Synthesis." In: PLDI 2015. **Types guide synthesis — direct precursor to our approach.**
- Polikarpova, Nadia; Kuraj, Ivan; Solar-Lezama, Armando. "Program Synthesis from Polymorphic Refinement Types." In: PLDI 2016. *Refinement types constrain synthesis search.*
- Feser, John K.; Chaudhuri, Swarat; Dillig, Isil. "Synthesizing Data Structure Transformations from I/O Examples." In: PLDI 2015. *Data structure transforms from examples — ARC-adjacent.*
- Frankle, Jonathan et al. "Example-Directed Synthesis: A Type-Theoretic Interpretation." In: POPL 2016. *Types + examples for synthesis — core methodology.*

---

## Domain 8: Category Theory & Higher Categories [HIGH]

### Already in bibliography
- Mac Lane, Saunders. *Categories for the Working Mathematician (2nd ed.)*. Springer, 1998.
- Awodey, Steve. *Category Theory (2nd ed.)*. OUP, 2010.
- Leinster, Tom. *Basic Category Theory*. Cambridge, 2014.
- Riehl, Emily. *Category Theory in Context*. Dover, 2016.

### ADD
- Lurie, Jacob. *Higher Topos Theory*. Princeton, 2009. *THE reference for infinity-topoi.*
- Lurie, Jacob. *Higher Algebra*. 2017. *infinity-operads and stable homotopy theory.*
- Riehl, Emily; Verity, Dominic. *Elements of infinity-Category Theory*. Cambridge, 2022. *Modern infinity-categories textbook.*
- Fong, Brendan; Spivak, David I. *An Invitation to Applied Category Theory*. Cambridge, 2019. *Accessible applied CT.*
- Bradley, Tai-Danae. "What is Applied Category Theory?" arXiv:1809.05923, 2018. *Brief survey of applied CT.*
- Joyal, Andre. "The Theory of Quasi-Categories and its Applications." 2008 lecture notes. *Foundational for quasicategories.*
- Leinster, Tom. *Higher Operads, Higher Categories*. LMS Lecture Notes, 2004. *Operadic perspective on higher categories.*
- Loregian, Fosco. *Coend Calculus*. LMS Lecture Notes, 2021. *Coend techniques for advanced CT.*
- Barwick, Clark; Schommer-Pries, Christopher. "On the Unicity of the Theory of Higher Categories." JAMS, 2021. *Uniqueness of infinity-categories theory.*

---

## Domain 9: Automatic Differentiation & Differentiable Programming [MEDIUM]

### Already in bibliography
- Griewank, Andreas; Walther, Andrea. *Evaluating Derivatives*. SIAM, 2008.
- Elliott, Conal. "The Simple Essence of Automatic Differentiation." ICFP 2018.
- Baydin, Atilim Gunes et al. "Automatic Differentiation in Machine Learning: A Survey." JMLR, 18(153):1-43, 2018.
- Fong, Brendan; Spivak, David I.; Tuyeras, Remy. "Backprop as Functor." LICS 2019.
- Huot, Mathieu; Staton, Sam; Vakar, Matthijs. "Correctness of AD via Diffeologies and Categorical Gluing." FoSSaCS 2020.

### ADD
- Vakar, Matthijs. "Reverse AD at Higher Types: Pure, Principled, Performant." In: POPL 2021. *Higher-order AD with correctness proof.*
- Elliott, Conal. "Compiling to Categories." In: ICFP 2017. *Categorical compilation — foundation for categorical AD.*
- Shaikhha, Amir et al. "Efficient Differentiable Programming in a Functional Array-Processing Language." In: ICFP 2019. *Efficient functional AD.*
- Nunes, Fernando Lucatelli; Vakar, Matthijs. "CHAD: Combinatory Homomorphic AD." TOPLAS, 2023. *Categorical AD framework.*
- Cruttwell, Geoffrey et al. "Categorical Foundations of Gradient-Based Learning." ESOP 2022. *Category theory for learning.*

---

## Domain 10: Sensitivity, Metrics & Lipschitz Types [MEDIUM]

### Already in bibliography
- Reed, Jason; Pierce, Benjamin C. "Distance Makes the Types Grow Stronger." ICFP 2010.
- Gaboardi, Marco et al. "Linear Dependent Types for Differential Privacy." POPL 2013.
- Aberle, C.B. "Parametricity via Cohesion." MFPS 2024.
- Banados Schwerter, Matias; Garcia, Ronald; Tanter, Eric. "GSoul: Gradual Sensitivity Typing." CSF 2025.
- Azevedo de Amorim, Arthur et al. "A Semantic Account of Metric Preservation." POPL 2017.
- Dal Lago, Ugo; Gavazzo, Francesco. "Differential Logical Relations." ICALP 2019.

### ADD
- Atkey, Robert. "The Syntax and Semantics of Quantitative Type Theory." LICS 2018. *QTT — resource annotations as types. Bridges linearity and metric sensitivity.*
- Atkey, Robert. "Parameterised Notions of Computation." JFP, 19(3-4):335-376, 2009. *Indexed monads — parameterized effects.*
- Choudhury, Vikraman; Krishnaswami, Neel. "Recovering Purity with Comonads and Capabilities." In: ICFP 2020. *Comonadic effects — relevant to smooth purity.*

---

## Domain 11: Formal Verification & Proof Assistants [MEDIUM]

### Already in bibliography
- The Coq Development Team. *Coq Reference Manual*. INRIA, 2022.
- Norell, Ulf. "Towards a Practical Programming Language Based on Dependent Type Theory." PhD, Chalmers, 2007.
- de Moura, Leonardo et al. "The Lean Theorem Prover." CADE 2015.

### ADD
- Ullrich, Sebastian; de Moura, Leonardo. "Beyond Notations: Hygienic Macro Expansion for Theorem Proving Languages." IJCAR 2020. *Lean 4 metaprogramming.*
- The Mathlib Community. "The Lean Mathematical Library." In: CPP 2020. *Largest formalized math library.*
- McBride, Conor. "Elimination with a Motive." In: TYPES 2000. *Dependent elimination — pattern matching foundation.*
- Cockx, Jesper. "Type Theory Unchained: Extending Agda with User-Defined Rewrite Rules." PhD, KU Leuven, 2020. *Rewrite rules in Agda — RTT precursor.*
- Sozeau, Matthieu et al. "Correct and Complete Type Checking and Typechecking for the Calculus of Inductive Constructions, in Coq." JACM, 2025. *Verified type checker.*

---

## Domain 12: Deep Learning, Transformers & Neural Reasoning [MEDIUM]

### Already in bibliography
- Goodfellow, Ian et al. *Deep Learning*. MIT Press, 2016.
- Bishop, Christopher M. *Pattern Recognition and Machine Learning*. Springer, 2006.
- Vapnik, Vladimir N. *The Nature of Statistical Learning Theory*. Springer, 2000.
- Chen, Ricky T. Q. et al. "Neural Ordinary Differential Equations." NeurIPS 2018.

### ADD
- Vaswani, Ashish et al. "Attention Is All You Need." NeurIPS 2017. *The transformer architecture.*
- Brown, Tom et al. "Language Models are Few-Shot Learners." NeurIPS 2020. *GPT-3 — in-context learning.*
- Wei, Jason et al. "Chain-of-Thought Prompting Elicits Reasoning in LLMs." NeurIPS 2022. *CoT prompting for reasoning.*
- Nye, Maxwell et al. "Show Your Work: Scratchpads for Intermediate Computation." arXiv:2112.00114, 2021. *Scratchpad reasoning.*
- Wu, Yuhuai et al. "Autoformalization with Large Language Models." NeurIPS 2022. *LLMs formalizing math — bridges informal and formal.*
- Welleck, Sean et al. "NaturalProofs: Mathematical Theorem Proving in Natural Language." NeurIPS 2021. *Natural language proofs.*
- Polu, Stanislas; Sutskever, Ilya. "Generative Language Modeling for Automated Theorem Proving." arXiv:2009.03393, 2020. *GPT-f for Lean.*
- Han, Jesse Michael et al. "Proof Artifact Co-Training for Theorem Proving." In: ICLR 2022. *PACT — training on proof artifacts.*
- Jiang, Albert Q. et al. "Autoformalization in the Era of Large Language Models: A Survey." arXiv:2505.23486, 2025. *Comprehensive survey of autoformalization with LLMs.*
- Li, Zhaoyu et al. "A Survey on Deep Learning for Theorem Proving." In: COLM 2024. *Deep learning + theorem proving survey.*

---

## Domain 13: NbE, Implementation & Type Checker Engineering [HIGH]

### NEW — Critical for implementation

- Berger, Ulrich; Schwichtenberg, Helmut. "An Inverse of the Evaluation Functional for Typed lambda-Calculus." In: LICS 1991. *THE original NbE paper.*
- Abel, Andreas. *Normalization by Evaluation: Dependent Types and Impredicativity*. Habilitation, LMU Munich, 2013. *Comprehensive NbE for dependent types.*
- Abel, Andreas; Coquand, Thierry; Dybjer, Peter. "Normalization by Evaluation for Martin-Lof Type Theory with Typed Equality Judgements." In: LICS 2007. *NbE for MLTT specifically.*
- Gratzer, Daniel. *Syntax and Semantics of Modal Type Theory*. PhD, Aarhus, 2023. *Modal TT implementation techniques.*
- Kovacs, Andras. "Closure-Free Functional Programming in a Two-Level Type Theory." ICFP 2024. *Defunctionalized closures — directly relevant to v0 kernel.*
- Kovacs, Andras. "Staged Compilation with Two-Level Type Theory." ICFP 2022. *Two-level TT for staging — meta-level/object-level separation.*
- Sirman, Loic; Lennon-Bertrand, Meven; Krishnaswami, Neel. "Implementing Observational Equality with NbE." TYPES 2024. *OTT + NbE implementation.*
- Sterling, Jonathan. *First Steps in Synthetic Tait Computability*. PhD, CMU, 2021. *Synthetic approach to normalization proofs.*

---

## Domain 14: Cognitive Science & Intelligence Measurement [MEDIUM]

### NEW — Supporting ARC thesis

- Chollet, Francois. "On the Measure of Intelligence." arXiv:1911.01547, 2019. *Defines intelligence as skill-acquisition efficiency. ARC's theoretical foundation.*
- Lake, Brenden M. et al. "Building Machines That Learn and Think Like People." BBS, 40:e253, 2017. *Core knowledge priors: objectness, number, geometry, physics.*
- Hernandez-Orallo, Jose. *The Measure of All Minds*. Cambridge, 2017. *Universal intelligence measurement.*
- Spelke, Elizabeth S.; Kinzler, Katherine D. "Core Knowledge." Developmental Science, 10(1):89-96, 2007. *Core knowledge priors in infants — maps to ARC priors.*
- Hofstadter, Douglas R. *Fluid Concepts and Creative Analogies*. Basic Books, 1995. *Analogical reasoning and pattern recognition.*

---

## Domain 15: Rewriting & Equational Theory [HIGH — for epsilon-squared=0]

### Already in bibliography
- Cockx, Jesper; Tabareau, Nicolas; Winterhalter, Theo. "The Taming of the Rew." POPL 2021.
- Leray, Yann; Winterhalter, Theo. "Locally-Scoped Rewrite Rules in Rewriting Type Theory." POPL 2026.
- Leray, Yann et al. "The Rewster." ITP 2024.
- Felicissimo, Thiago. "Bidirectional Typing for CIC." ESOP 2024.
- Barras, Bruno; Maestracci, Valentin. "Two Layers Type Theory in Dedukti." LFMTP 2020.

### ADD
- Blanqui, Frederic. "Type Theory and Rewriting." Habilitation, Paris-Saclay, 2024. *Comprehensive treatment of TT + rewriting.*
- Cockx, Jesper. "Overlapping and Order-Independent Patterns." In: POPL 2014. *Pattern matching theory.*
- Thiemann, Rene; Sternagel, Christian. "Certification of Termination Proofs Using CeTA." In: TPHOLs 2009. *Certified rewriting termination.*

---

## Summary Statistics

| Domain | Existing | Added | Total |
|-|-|-|-|
| Type Theory Foundations | 4 | 10 | 14 |
| Cubical Type Theory | 8 | 10 | 18 |
| Homotopy Type Theory | 5 | 9 | 14 |
| SDG | 6 | 9 | 15 |
| Differential Cohesion | 4 | 8 | 12 |
| ARC & Abstract Reasoning | 0 | 18 | 18 |
| Program Synthesis | 0 | 13 | 13 |
| Category Theory | 4 | 9 | 13 |
| AD & Differentiable Programming | 5 | 5 | 10 |
| Sensitivity & Lipschitz | 6 | 3 | 9 |
| Formal Verification | 3 | 5 | 8 |
| Deep Learning & Neural | 4 | 10 | 14 |
| NbE & Implementation | 0 | 8 | 8 |
| Cognitive Science | 0 | 5 | 5 |
| Rewriting & Equational | 5 | 3 | 8 |
| Physics/Geometry/Numerical | ~20 | 0 | ~20 |
| **TOTAL** | **~85** | **~125** | **~210** |
