# SCTT Problem Sets: Building Your Understanding

## Problem Set 1: Foundations (Warm-up)

### Problem 1.1: Types as Contracts
Write a function that takes a smooth function and verifies it satisfies the contract "all derivatives exist."

**Your Task:**
```python
def verify_smooth(f):
    # Return True if f has derivatives of all orders
    # Hint: Try computing first 10 derivatives symbolically
    pass
```

### Problem 1.2: Path Basics
Create three different paths from 0 to π that represent:
1. Linear interpolation
2. Smooth (sine-based) interpolation  
3. Your own creative smooth path

Verify all three reach the correct endpoints.

### Problem 1.3: The Interval
Implement the interval operations and prove De Morgan's laws:
- Prove: ¬(i ∧ j) = ¬i ∨ ¬j
- Prove: ¬(i ∨ j) = ¬i ∧ ¬j

---

## Problem Set 2: Smooth Structure (Getting Deeper)

### Problem 2.1: Chain Rule Detective
Given two composed smooth functions h = f ∘ g, and knowing h and g, can you recover f?

**Challenge:** Write an algorithm that:
- Takes h(x) = sin(x²) and g(x) = x²
- Recovers f(x) = sin(x)

### Problem 2.2: Smooth vs Non-Smooth
Create a function that approaches smoothness but isn't quite smooth:
```python
def almost_smooth(x, epsilon):
    # Should be smooth everywhere except one point
    # As epsilon → 0, should approach true smoothness
    pass
```

Explore: What happens in SCTT when we try to type-check this?

### Problem 2.3: The Derivative Tower
For f(x) = e^x, prove that f^(n)(x) = f(x) for all n.
Then find another non-trivial function with this property.

---

## Problem Set 3: Cubical Constructions (The Fun Part)

### Problem 3.1: Square Filling
Given four paths forming a square:
- Top: sin(πt) from 0 to 0
- Right: t from 0 to 1  
- Bottom: t from 0 to 1
- Left: t² from 0 to 1

Fill in the square with a smooth surface. Verify your filling is actually smooth.

### Problem 3.2: Path Reversal
Prove that for any smooth path p from A to B:
- reverse(reverse(p)) = p
- p composed with reverse(p) is homotopic to the constant path at A

### Problem 3.3: The Möbius Path
Design a path on the circle that when traversed twice gives a different result than traversing once and then again. (Hint: Think about paths in S¹ × ℝ)

---

## Problem Set 4: Coherence Challenges (The Hard Part)

### Problem 4.1: Finding Incoherence
Find a specific example where:
- Operation A preserves smoothness
- Operation B preserves cubical structure
- But A ∘ B doesn't preserve both

This is the kind of problem SCTT must solve!

### Problem 4.2: The Coherence Diamond
Given:
```
     f
  A ----> B
  |       |
g |       | h
  ↓       ↓
  C ----> D
     k
```

If all arrows are smooth functions and the diagram commutes (h∘f = k∘g), prove that the "diagonal fill" is also smooth.

### Problem 4.3: Transport Puzzle
If you transport a smooth function along a smooth path, is the result always smooth? Either prove it or find a counterexample.

---

## Problem Set 5: Applications (See the Power)

### Problem 5.1: Neural Network Smoothification
Take a ReLU neural network:
```python
def relu_net(x):
    return max(0, w1*x + b1)  # Not smooth at 0!
```

Design a smooth approximation that:
- Is actually C∞
- Approximates ReLU arbitrarily well
- Has bounded derivatives

### Problem 5.2: Optimization Proof
For f(x) = x⁴ - 2x² + 1:
1. Find all critical points
2. Classify each (min/max/saddle)
3. Prove your classification is correct
4. Extract an algorithm from your proof that finds the global minimum

### Problem 5.3: Physics Simulation
Model a pendulum as a smooth path in phase space:
- Position: θ(t)
- Momentum: p(t) = dθ/dt

Prove energy conservation: E = p²/2 + cos(θ) is constant along the path.

---

## Problem Set 6: Research Problems (Open-Ended)

### Problem 6.1: The Decidability Boundary
Characterize exactly which smooth functions have decidable equality:
- Polynomials: Yes
- Rational functions: Yes
- exp, log, sin, cos: Yes
- Your job: Find the boundary. What's the simplest undecidable case?

### Problem 6.2: Smooth HITs
Design a smooth version of the torus:
- It should be a higher inductive type
- All paths should be smooth
- The usual torus relations should hold
- Bonus: Prove π₁(T²) = ℤ × ℤ

### Problem 6.3: Compression Challenge
Take a trained neural network (any small one).
Find a smooth function with 10x fewer parameters that achieves 95% of the accuracy.
This demonstrates SCTT's compression superpower!

---

## Problem Set 7: The Breakthrough Problem

### The Challenge That Leads to Innovation

**Problem 7.1: The Coherence Constitution**

Write down a minimal set of axioms that guarantee smooth and cubical structures work together. Your axioms should:

1. Be as few as possible
2. Be verifiable algorithmically
3. Cover all the cases we care about
4. Not be too restrictive

This is essentially the core problem of SCTT. If you solve this cleanly, you've made a breakthrough!

**Hints:**
- Start with: "Composition preserves smoothness"
- Add: "Transport preserves smoothness"
- What else is minimal and sufficient?

**Test your axioms:**
- Do they allow the examples we want?
- Do they exclude the problematic cases?
- Can a computer check them?

---

## Meta-Problem: Pattern Recognition

After solving problems 1-7, answer:

1. What patterns did you notice?
2. Where does the traditional approach break down?
3. What would make these problems trivial?
4. What new mathematical structure would help?

Your answers here might be the key insight for SCTT!

---

## Bonus Problems for the Ambitious

### Bonus 1: Smooth Cryptography
Design a cryptographic hash function based on smooth functions. Is it secure? Why or why not?

### Bonus 2: The Bootstrap
Can SCTT type-check itself? Design a minimal SCTT that can verify its own consistency.

### Bonus 3: Quantum Smooth
Extend smoothness to complex-valued functions. Model quantum mechanics in SCTT.

---

## How to Use These Problems

1. **Start with Problem Set 1** - Build foundations
2. **Don't skip to solutions** - The struggle builds intuition
3. **Document your attempts** - Failed approaches teach as much as successes
4. **Look for patterns** - The meta-learning is as important as solving
5. **When stuck, ask**: "What would make this easy?"

Remember: The person who invents SCTT won't be the one who solves all these problems, but the one who finds a framework where these problems solve themselves!