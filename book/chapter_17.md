# Chapter 17: Certified Machine Learning

> "All models are wrong, but some are provably bounded." — Adapted from George Box

## Introduction

Machine learning models lack mathematical guarantees. A neural network might classify an image correctly yet flip its answer when a single pixel changes. An ODE solver might return plausible trajectories that silently diverge from the true solution. A gradient computation might accumulate floating-point drift that compounds across millions of parameters.

These are not hypothetical concerns. Adversarial attacks exploit the absence of robustness guarantees. Neural ODE solvers fail silently when dynamics violate Lipschitz conditions the solver assumes but never checks. Automatic differentiation frameworks compute gradients that are correct *most of the time* — and subtly wrong when it matters.

SCTT provides the type-theoretic tools to make these guarantees static and compile-time:

- **Lipschitz types** bound how much outputs change under input perturbation ([Chapter 7](./chapter_07.md), Reed and Pierce, ICFP 2010)
- **Smooth types** make backpropagation algebraically exact via the Kock-Lawvere axiom ([Chapter 4](./chapter_04.md), [Chapter 5](./chapter_05.md))
- **Path types** connect training trajectories to proof-theoretic structure ([Chapter 3](./chapter_03.md))

This chapter walks through typing real ML constructs in SCTT — not toy examples, but architectures and algorithms that practitioners actually use.

### Prerequisites

This chapter builds directly on:

- **Lipschitz types** from [Chapter 7](./chapter_07.md): the `Lip(A, B, k)` type and its composition rules
- **Smooth functions** from [Chapter 4](./chapter_04.md): `C∞(M, N)` and the Kock-Lawvere axiom
- **Tangent and cotangent bundles** from [Chapter 5](./chapter_05.md): `T M`, `T* M`, and the chain rule
- **Path types** from [Chapter 3](./chapter_03.md): `Path A a b` for connecting states

```sctt
-- The three layers of SCTT that this chapter combines
certified_ml_stack : Type
certified_ml_stack = {
  sensitivity : Lip(Input, Output, k),    -- how much can outputs change?
  smoothness  : C∞(Params, Loss),          -- are gradients exact?
  topology    : Path Weights w_init w_opt  -- does training converge?
}
```

## 17.1 Neural Networks as Typed Functions {#typed-networks}

A neural network is a composition of parameterized functions. In standard ML frameworks, these functions carry no type-level information about their sensitivity. In SCTT, every layer declares its Lipschitz constant.

### A Linear Layer

The simplest building block is a linear map followed by a bias:

```sctt
-- A linear layer: y = Wx + b
-- The Lipschitz constant equals the operator norm of W
linear_layer :
  (n m : Nat) ->
  (W : Matrix m n) ->
  (b : Vec m) ->
  Lip(Vec n, Vec m, operator_norm W)
linear_layer n m W b =
  lip (λ x → mat_vec_mul W x + b) (operator_norm_proof W)
```

The key fact: the Lipschitz constant of a linear map is the **operator norm** of its weight matrix. This is not an assumption — it follows from the definition of operator norm as the supremum of `‖Wx‖ / ‖x‖`. The proof term `operator_norm_proof W` witnesses this.

For a matrix `W : Matrix m n`, the operator norm (with respect to the ℓ₂ norm) equals its largest singular value σ₁(W). Computing this is an `O(min(m,n) · mn)` operation — expensive, but exact.

### ReLU and Non-Smooth Activations

ReLU is the workhorse activation of modern deep learning. It is Lipschitz but not smooth:

```sctt
-- ReLU is 1-Lipschitz (non-expansive)
relu : Lip(ℝ, ℝ, 1.0)
relu = lip (λ x → max 0 x) relu_lipschitz_proof

-- Proof sketch: |max(0,x) - max(0,y)| ≤ |x - y| for all x, y
-- This is a case analysis on the signs of x and y
```

Because ReLU is not differentiable at 0, it lives in `Lip` but not in `C∞`. For SCTT's smooth differentiation machinery, we need smooth approximations:

```sctt
-- Softplus: smooth approximation to ReLU
softplus : C∞(ℝ, ℝ)
softplus = smooth (λ x → log(1 + exp(x)))

-- Its derivative is the sigmoid — exact via Kock-Lawvere
-- D[softplus](x) = exp(x) / (1 + exp(x)) = sigmoid(x)

-- GELU: another smooth activation used in transformers
gelu : C∞(ℝ, ℝ)
gelu = smooth (λ x → x * Φ(x))
-- where Φ is the standard normal CDF (smooth)

-- Smooth activations also have Lipschitz bounds
softplus_lip : Lip(ℝ, ℝ, 1.0)
softplus_lip = smooth_implies_lip softplus softplus_lip_proof
```

The practical tradeoff: ReLU gives tighter Lipschitz bounds (constant 1, exactly), while smooth activations give exact gradients. SCTT makes this tradeoff explicit in the types.

### Composing Layers

The composition rule for Lipschitz types is the chain rule for sensitivities:

```sctt
-- Lip composition: constants multiply
lip_compose :
  Lip(A, B, k1) ->
  Lip(B, C, k2) ->
  Lip(A, C, k1 * k2)

-- A two-layer network with ReLU
two_layer_relu :
  (W1 : Matrix 128 784) ->
  (b1 : Vec 128) ->
  (W2 : Matrix 10 128) ->
  (b2 : Vec 10) ->
  Lip(Vec 784, Vec 10, operator_norm W1 * operator_norm W2)
two_layer_relu W1 b1 W2 b2 =
  lip_compose
    (lip_compose
      (linear_layer 784 128 W1 b1)  -- Lip(Vec 784, Vec 128, ‖W1‖)
      relu_vec)                       -- Lip(Vec 128, Vec 128, 1.0)
    (linear_layer 128 10 W2 b2)     -- Lip(Vec 128, Vec 10, ‖W2‖)
```

The total Lipschitz constant `‖W1‖ · 1.0 · ‖W2‖ = ‖W1‖ · ‖W2‖` bounds the worst-case output change per unit input perturbation. This is computed statically — no need to estimate it empirically.

### Residual Connections

ResNets add skip connections that help bound Lipschitz constants:

```sctt
-- A residual block: h(x) = x + f(x)
-- If f is k-Lipschitz, then h is (1+k)-Lipschitz
residual_block :
  (f : Lip(Vec n, Vec n, k)) ->
  Lip(Vec n, Vec n, 1.0 + k)
residual_block f =
  lip (λ x → x + apply_lip f x) (residual_lip_proof f)

-- Proof: ‖h(x) - h(y)‖ = ‖(x-y) + (f(x)-f(y))‖
--                       ≤ ‖x-y‖ + ‖f(x)-f(y)‖
--                       ≤ ‖x-y‖ + k·‖x-y‖
--                       = (1+k)·‖x-y‖
```

If each residual block has a small Lipschitz constant for `f` (say k = 0.1), the total constant after L blocks is `(1.1)^L` — exponential in depth, but with a much smaller base than an unconstrained network.

## 17.2 Certified Adversarial Robustness {#robustness}

### The Robustness Problem

Given a classifier `net : Vec n → Vec c` and an input `x`, adversarial robustness asks: how large a perturbation `δ` can we apply to `x` before the classification changes? Standard ML provides no answer. SCTT gives one at compile time.

### The Robustness Theorem

```sctt
-- Classification margin: the gap between the top class score and the runner-up
margin : Vec c -> ℝ
margin scores =
  let sorted = sort_descending scores
  in sorted[0] - sorted[1]

-- The robustness certificate
robustness_certificate :
  (net : Lip(Vec n, Vec c, k)) ->
  (x : Vec n) ->
  (m : margin (apply_lip net x) > 0) ->  -- net classifies x with positive margin
  ∀ (x' : Vec n),
    d(x, x') < margin (apply_lip net x) / (2 * k) ->
    classify net x' ≡ classify net x
robustness_certificate net x m x' d_bound =
  -- Proof: ‖net(x) - net(x')‖ ≤ k · ‖x - x'‖ < k · (m/(2k)) = m/2
  -- Each score moves by less than m/2, so a gap of m cannot close
  margin_preservation net x x' (lip_bound net x x' d_bound) m
```

This is a **type-level guarantee**, checked at compile time. The robustness radius `m/(2k)` depends on two quantities:

1. The **margin** `m`: how confidently the network classifies `x` (computed at inference time)
2. The **Lipschitz constant** `k`: how sensitive the network is (computed at compile time)

### Spectral Normalization as Type Enforcement

Spectral normalization (Miyato et al., ICLR 2018) constrains each layer's operator norm during training. In SCTT, this is a type-preserving transformation:

```sctt
-- Spectral normalization: rescale W so its operator norm ≤ k
spectral_normalize :
  (W : Matrix m n) ->
  (k : ℝ) ->
  Σ (W' : Matrix m n), operator_norm W' ≤ k
spectral_normalize W k =
  let σ1 = largest_singular_value W
      W' = if σ1 > k then (k / σ1) * W else W
  in (W', spectral_norm_proof W' k)

-- A spectrally normalized layer has a guaranteed Lip bound
sn_layer :
  (n m : Nat) ->
  (W : Matrix m n) ->
  (b : Vec m) ->
  (k : ℝ) ->
  Lip(Vec n, Vec m, k)
sn_layer n m W b k =
  let (W', _) = spectral_normalize W k
  in linear_layer n m W' b  -- type checks because ‖W'‖ ≤ k
```

### Computing Certified Radii

For a practical network with L layers, each spectrally normalized to have operator norm at most `k_per_layer`:

```sctt
-- Network with per-layer spectral normalization
sn_network :
  (layers : Vec L (Σ (W : Matrix _ _), operator_norm W ≤ k_per_layer)) ->
  Lip(Vec n_in, Vec n_out, k_per_layer ^ L)
sn_network layers = fold_lip_compose layers

-- Example: 10-layer network, each layer normalized to ‖W‖ ≤ 1.5
-- Total Lipschitz constant: 1.5^10 ≈ 57.7
-- With margin m = 0.5, robustness radius = 0.5 / (2 × 57.7) ≈ 0.0043

-- Compare: without normalization, ‖W‖ might be 5 per layer
-- Total: 5^10 ≈ 9.7 million. Robustness radius: effectively 0.
```

This makes the tradeoff between expressiveness and robustness quantitative and explicit.

## 17.3 Neural ODEs as Smooth Paths {#neural-odes}

Neural ODEs (Chen et al., NeurIPS 2018) parameterize a continuous-depth network as an ODE. In SCTT, this is natural: the solution is a smooth path, and the Lipschitz condition on the dynamics guarantees existence and uniqueness.

### The Neural ODE Type

```sctt
-- A neural ODE defines continuous dynamics on a state space
-- dh/dt = f(h(t), t, θ)
-- where f must be Lipschitz in h for existence and uniqueness
neural_ode :
  (f : (θ : Params) -> Lip(ℝⁿ × ℝ, ℝⁿ, k)) -> -- dynamics, k-Lip in state
  (h0 : ℝⁿ) ->                                    -- initial condition
  (t0 t1 : ℝ) ->                                   -- time interval
  SmoothPath ℝⁿ                                    -- solution trajectory
```

The Lipschitz typing of `f` does double duty:

1. **Picard-Lindelöf**: `k`-Lipschitz dynamics guarantee a unique solution exists. This is a theorem, but in SCTT it is enforced by the type checker — you cannot construct a `neural_ode` with non-Lipschitz dynamics.

2. **Sensitivity**: The Lipschitz constant `k` bounds how fast nearby trajectories can diverge. This feeds directly into Gronwall's inequality.

### The Adjoint Method as Cotangent Transport

Training a neural ODE requires gradients of the loss with respect to parameters θ. The adjoint method computes these by solving a backward ODE. In SCTT, this is cotangent transport along a smooth path:

```sctt
-- Forward pass: solve the ODE, get the trajectory
forward :
  (f : (θ : Params) -> Lip(ℝⁿ × ℝ, ℝⁿ, k)) ->
  (θ : Params) ->
  (h0 : ℝⁿ) ->
  SmoothPath ℝⁿ

-- The adjoint state a(t) = ∂L/∂h(t) lives in the cotangent bundle
-- It solves: da/dt = -a(t)ᵀ · ∂f/∂h(h(t), t, θ)
adjoint :
  (trajectory : SmoothPath ℝⁿ) ->
  (loss : C∞(ℝⁿ, ℝ)) ->
  SmoothPath (T* ℝⁿ)  -- path in the cotangent space
adjoint trajectory loss =
  cotangent_transport trajectory (D[loss])
```

In standard implementations, the adjoint method introduces numerical error from the backward ODE solver. In SCTT, because `trajectory` is a `SmoothPath` and `loss` is `C∞`, the cotangent transport is **exact**. The chain rule is definitional ([Chapter 5](./chapter_05.md)), not approximate.

### Parameter Gradients

```sctt
-- The parameter gradient accumulates along the trajectory
param_gradient :
  (f : (θ : Params) -> Lip(ℝⁿ × ℝ, ℝⁿ, k)) ->
  (θ : Params) ->
  (adjoint_path : SmoothPath (T* ℝⁿ)) ->
  T* Params  -- gradient in parameter space
param_gradient f θ adj =
  -- ∂L/∂θ = -∫₀¹ a(t)ᵀ · ∂f/∂θ(h(t), t, θ) dt
  smooth_integral (λ t →
    cotangent_apply (adj t) (partial_deriv (λ θ' → f θ' (h t, t)) θ))
```

### Stability via Gronwall's Inequality

Gronwall's inequality bounds how fast two solution trajectories can diverge. In SCTT, this is a theorem about Lipschitz-typed dynamics:

```sctt
-- Gronwall's inequality as a type
gronwall :
  (f : Lip(ℝⁿ, ℝⁿ, k)) ->
  (h1 h2 : SmoothPath ℝⁿ) ->
  (solutions : is_solution f h1 ∧ is_solution f h2) ->
  ∀ (t : ℝ), t ≥ 0 ->
    d(h1(t), h2(t)) ≤ exp(k · t) · d(h1(0), h2(0))
```

The exponential bound `exp(k · t)` tells you exactly how much initial perturbations amplify over time. For a neural ODE with `k = 2` run for time `t = 1`, perturbations grow by at most a factor of `e² ≈ 7.4`. This is a hard upper bound — the actual growth may be less, but it cannot be more.

### Continuous Normalizing Flows

A continuous normalizing flow uses a neural ODE to define a probability distribution:

```sctt
-- A normalizing flow maps a base distribution to a target distribution
-- via a smooth, invertible transformation
continuous_nf :
  (f : (θ : Params) -> Lip(ℝⁿ × ℝ, ℝⁿ, k)) ->
  (base : Distribution ℝⁿ) ->
  Distribution ℝⁿ
continuous_nf f base =
  pushforward (flow_map (neural_ode f)) base

-- The log-density changes via the instantaneous change of variables
-- log p(h(t)) = log p(h(0)) - ∫₀ᵗ tr(∂f/∂h(h(s), s)) ds
-- The trace term is exact in SCTT (not estimated via Hutchinson)
log_density_change :
  (f : (θ : Params) -> C∞(ℝⁿ × ℝ, ℝⁿ)) ->
  (trajectory : SmoothPath ℝⁿ) ->
  C∞(ℝ, ℝ)  -- log-density as a function of time
log_density_change f trajectory =
  smooth (λ t → neg (trace (jacobian (λ h → f θ (h, t)) (trajectory t))))
```

## 17.4 Automatic Differentiation in SCTT {#autodiff}

SCTT does not *implement* automatic differentiation — it *is* automatic differentiation. The Kock-Lawvere axiom makes differentiation a primitive operation, not an algorithm applied to code.

### Forward Mode via the Kock-Lawvere Axiom

Recall from [Chapter 4](./chapter_04.md): for any smooth function `f : ℝ → ℝ` and nilsquare `ε` (satisfying ε² = 0), the Kock-Lawvere axiom gives:

```
f(x + ε) = f(x) + f'(x) · ε
```

This is not a Taylor approximation — it is **exact** because ε² = 0 kills all higher-order terms. Forward-mode AD falls out as a special case:

```sctt
-- Forward-mode AD is the Kock-Lawvere axiom applied
forward_ad : C∞(ℝ, ℝ) → (ℝ → ℝ × ℝ)  -- (value, derivative)
forward_ad f x =
  let ε : D  -- D is the object of nilsquares, ε² = 0
      result = f(x + ε)  -- = f(x) + f'(x) · ε by Kock-Lawvere
  in (coefficient_0 result, coefficient_1 result)

-- For multivariate functions: the tangent functor
tangent_map : C∞(ℝⁿ, ℝᵐ) → C∞(T ℝⁿ, T ℝᵐ)
tangent_map f (x, v) = (f(x), Df(x) · v)
-- Df(x) · v is the Jacobian-vector product (JVP)
```

### Reverse Mode via Cotangent Transport

Reverse-mode AD — backpropagation — corresponds to the pullback on cotangent bundles:

```sctt
-- Reverse-mode AD is the cotangent functor
cotangent_map : C∞(ℝⁿ, ℝᵐ) → C∞(T* ℝᵐ, T* ℝⁿ)
cotangent_map f (y, w) =
  let x = f⁻¹(y)  -- (in context where f is evaluated at known x)
  in (x, Df(x)ᵀ · w)
-- Df(x)ᵀ · w is the vector-Jacobian product (VJP)

-- The gradient of a scalar function
gradient : C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝⁿ)
gradient f x = cotangent_map f (f(x), 1.0)
-- Seed the cotangent with 1.0, pull back to get ∇f(x)
```

### Correctness by Construction (Elliott's Theorem)

Elliott (ICFP 2018) showed that AD can be viewed as a homomorphic functor on categories of smooth maps. In SCTT, this is not a theorem to prove — it is a consequence of the type theory:

```sctt
-- AD correctness: the functorial equations hold definitionally
-- Forward mode preserves composition
forward_compose :
  (f : C∞(A, B)) -> (g : C∞(B, C)) ->
  tangent_map (g ∘ f) ≡ tangent_map g ∘ tangent_map f
-- This is the chain rule, which is definitional in SCTT

-- Reverse mode reverses composition
reverse_compose :
  (f : C∞(A, B)) -> (g : C∞(B, C)) ->
  cotangent_map (g ∘ f) ≡ cotangent_map f ∘ cotangent_map g
-- Contravariance of the cotangent functor
```

The categorical perspective (Fong, Spivak, and Tuyéras, LICS 2019; Cruttwell et al., ESOP 2022) views a parameterized learner as a morphism in a category where composition *is* backpropagation. SCTT makes this literal: composing smooth functions composes their derivatives.

### Higher-Order Differentiation

Because differentiation is a type-level operation, it composes:

```sctt
-- Second derivative: differentiate twice
hessian : C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, Matrix n n)
hessian f x = D[D[f]](x)

-- The Hessian is symmetric by Schwarz's theorem
-- In SCTT, this follows from the commutativity of partial derivatives
-- on smooth functions — a consequence of the smooth structure
hessian_symmetric :
  (f : C∞(ℝⁿ, ℝ)) -> (x : ℝⁿ) ->
  hessian f x ≡ transpose (hessian f x)
```

This matters for second-order optimization methods (Newton's method, natural gradient) that require exact Hessians or Hessian-vector products.

## 17.5 The Curry-Howard-Hinton Correspondence {#chh}

The Curry-Howard correspondence connects propositions to types and proofs to programs. The "Curry-Howard-Hinton" (CHH) correspondence extends this analogy to machine learning:

| Type Theory | Logic | Machine Learning |
|---|---|---|
| Type | Proposition | Specification |
| Term | Proof | Trained weights |
| Type checking | Proof verification | Inference |
| Proof search | Theorem proving | Training (optimization) |
| Normalization | Cut elimination | Compression / distillation |

This section makes the analogy precise where we can, and honest about where it remains informal.

### Loss Functions as Propositions

A loss function encodes what the network should do. Zero loss means the specification is fully satisfied:

```sctt
-- A training specification: a loss function over weights and data
TrainingSpec : Type
TrainingSpec = {
  weights_type : Type,
  data_type : Type,
  loss : C∞(weights_type × data_type, ℝ),
  -- loss(θ, D) = 0 means θ satisfies the spec on data D
}

-- A specification for image classification
classification_spec : TrainingSpec
classification_spec = {
  weights_type = NetworkWeights,
  data_type = List (Image × Label),
  loss = smooth (λ (θ, data) →
    mean [cross_entropy (net θ img) label | (img, label) ∈ data])
}
```

### Trained Weights as Approximate Proofs

In logic, a proof of `P` is a term of type `P`. In ML, trained weights that achieve zero loss on the specification are a "proof" that the specification is satisfiable:

```sctt
-- Exact proof: zero loss
exact_proof :
  (spec : TrainingSpec) ->
  (θ : spec.weights_type) ->
  (D : spec.data_type) ->
  (spec.loss(θ, D) ≡ 0) ->
  Satisfies spec θ D

-- Approximate proof: bounded loss (more realistic)
approx_proof :
  (spec : TrainingSpec) ->
  (θ : spec.weights_type) ->
  (D : spec.data_type) ->
  (spec.loss(θ, D) ≤ ε) ->
  ApproxSatisfies spec θ D ε
```

Most trained models are approximate proofs — they satisfy the specification up to some residual ε. The Lipschitz structure tells us how this residual translates to behavior:

```sctt
-- If the loss is L-Lipschitz in the weights, and we're ε-close to zero loss,
-- then perturbing weights by δ changes the loss by at most L·δ
loss_perturbation :
  (spec : TrainingSpec) ->
  (loss_lip : Lip(spec.weights_type, ℝ, L)) ->
  (θ : spec.weights_type) ->
  (spec.loss(θ, D) ≤ ε) ->
  ∀ (θ' : spec.weights_type),
    d(θ, θ') ≤ δ ->
    spec.loss(θ', D) ≤ ε + L · δ
```

### Gradient Descent as Proof Search

Proof search in type theory tries to construct a term of a given type. Gradient descent tries to find weights that satisfy a specification. The analogy becomes precise when the loss is smooth and strongly convex:

```sctt
-- Gradient descent step
gd_step :
  (loss : C∞(ℝⁿ, ℝ)) ->
  (η : ℝ) ->                       -- learning rate
  ℝⁿ -> ℝⁿ
gd_step loss η θ = θ - η · D[loss](θ)
-- D[loss] is exact in SCTT (Kock-Lawvere)

-- Under strong convexity, gradient descent is a contraction
gd_contraction :
  (loss : C∞(ℝⁿ, ℝ)) ->
  (μ_sc : StronglyConvex loss μ) ->    -- strong convexity constant μ
  (L_sm : LipschitzGradient loss L) -> -- gradient Lipschitz constant L
  (η : ℝ) ->
  (η_bound : η ≤ 2 / (μ + L)) ->
  Lip(ℝⁿ, ℝⁿ, 1 - η · μ)            -- contraction rate
gd_contraction loss μ_sc L_sm η η_bound =
  lip (gd_step loss η) (gd_contraction_proof loss μ_sc L_sm η η_bound)
```

When `η · μ > 0`, the contraction rate `1 - η · μ < 1`, so gradient descent brings every iterate closer to the optimum. After `T` steps, the distance to the optimum is at most `(1 - η · μ)^T · d(θ₀, θ*)`. The Lipschitz type makes this bound static.

### Convergence as a Path

Training traces a path through weight space. In SCTT, this path has type-theoretic structure:

```sctt
-- Training trajectory: a sequence of weight updates
-- viewed as a discrete approximation to a smooth path
training_path :
  (loss : C∞(ℝⁿ, ℝ)) ->
  (η : ℝ) ->
  (θ0 : ℝⁿ) ->
  (T : Nat) ->
  Vec (T + 1) ℝⁿ
training_path loss η θ0 T =
  iterate T (gd_step loss η) θ0

-- In the continuous limit (gradient flow), this is a smooth path
gradient_flow :
  (loss : C∞(ℝⁿ, ℝ)) ->
  (θ0 : ℝⁿ) ->
  SmoothPath ℝⁿ  -- dθ/dt = -∇loss(θ(t))
gradient_flow loss θ0 =
  neural_ode (λ _ → smooth_to_lip (λ (θ, _) → neg (D[loss](θ)))) θ0 0 ∞
```

Gradient flow is itself a neural ODE. Its Lipschitz constant is the Lipschitz constant of the gradient, which is the smoothness constant of the loss. The entire training process lives within SCTT's type system.

### What CHH Does Not (Yet) Cover

The analogy breaks down in several important places that represent open problems:

```sctt
-- OPEN: Generalization
-- Why do trained weights work on unseen data?
-- Parametricity (Aberlé, MFPS 2024) provides one angle:
-- a model that works "for all representations" of the input
-- must capture genuine structure, not surface patterns.
-- But formalizing this fully is OPEN.

-- OPEN: Non-convex loss landscapes
-- Real neural network losses are highly non-convex.
-- Gradient descent finds good minima in practice,
-- but we lack type-theoretic explanations for why.

-- OPEN: Connecting all three layers
-- We have:
--   ✓ Smooth types for exact gradient computation
--   ✓ Lipschitz types for convergence and robustness bounds
--   ✓ Path types for training trajectories
--   ✓ Parametricity for a notion of generalization
--   ✗ A unified framework connecting all four to
--     the full training-generalization pipeline: OPEN
```

## 17.6 Practical Implementation Pathway {#implementation}

This section outlines how a practitioner would use SCTT to build a certified ML system, today.

### Step 1: Type Your Network Architecture

Define each layer with explicit Lipschitz bounds. Use spectral normalization to enforce per-layer constraints:

```sctt
-- Define a small classifier with per-layer bounds
certified_classifier :
  (k_max : ℝ) ->  -- target total Lipschitz constant
  Lip(Vec 784, Vec 10, k_max)
certified_classifier k_max =
  let k_per_layer = k_max ^ (1.0 / 3.0)  -- cube root for 3 layers
      layer1 = sn_layer 784 256 W1 b1 k_per_layer
      layer2 = sn_layer 256 64  W2 b2 k_per_layer
      layer3 = sn_layer 64  10  W3 b3 k_per_layer
      act    = relu_vec  -- 1-Lipschitz, does not increase constant
  in lip_compose (lip_compose (lip_compose layer1 act) (lip_compose layer2 act)) layer3
```

### Step 2: Verify Robustness at Compile Time

The composed Lipschitz constant bounds worst-case perturbation sensitivity. The type checker verifies this:

```sctt
-- The type checker confirms: total Lip constant ≤ k_max
-- At inference time, compute the margin for each input
certify_input :
  (net : Lip(Vec 784, Vec 10, k_max)) ->
  (x : Vec 784) ->
  CertificationResult
certify_input net x =
  let scores = apply_lip net x
      m = margin scores
      radius = m / (2 * k_max)
  in { prediction = argmax scores,
       margin = m,
       certified_radius = radius,
       -- Any perturbation smaller than radius preserves the prediction
       certificate = robustness_certificate net x m }
```

### Step 3: Train with Exact Gradients

Use SCTT's Kock-Lawvere differentiation for symbolically exact backpropagation. For smooth activations, the gradients are definitionally correct:

```sctt
-- Training loop with exact gradients and convergence tracking
train :
  (net_init : Lip(Vec 784, Vec 10, k_max)) ->
  (data : List (Vec 784 × Vec 10)) ->
  (η : ℝ) ->
  (epochs : Nat) ->
  Lip(Vec 784, Vec 10, k_max)  -- trained network, SAME Lip bound
train net_init data η epochs =
  iterate epochs (λ net →
    let loss_grad = gradient (λ θ → mean_loss net θ data) (params net)
    in update_params net (neg (η · loss_grad)))
  net_init
-- Spectral normalization after each update preserves the Lip type
```

### Step 4: Certify the Result

After training, every prediction comes with a robustness certificate:

```sctt
-- Deployment: every prediction is certified
deploy :
  (net : Lip(Vec 784, Vec 10, k_max)) ->
  (x : Vec 784) ->
  { prediction : Fin 10,
    certified_robust_within : ℝ,  -- perturbation radius
    proof : ∀ x', d(x,x') < certified_robust_within ->
              classify net x' ≡ classify net x }
```

### What You Get

| Guarantee | Mechanism | Checked When |
|---|---|---|
| Adversarial robustness | Lip constant × margin | Compile time (Lip) + inference (margin) |
| Training convergence | Lip + strong convexity → contraction rate | Compile time |
| ODE solver correctness | Picard-Lindelöf via Lip types | Compile time |
| Gradient correctness | Kock-Lawvere axiom | Definitional (always) |
| Trajectory stability | Gronwall via Lip bound | Compile time |

### Limitations and Honest Assessment

SCTT does not solve all ML problems:

- **Lipschitz bounds can be loose.** The product of per-layer operator norms overestimates the true Lipschitz constant. Tighter bounds require global analysis that is computationally harder.
- **Smooth activations may reduce performance.** Replacing ReLU with softplus to get exact gradients can change model accuracy. The type system makes this tradeoff visible, but does not eliminate it.
- **Non-convex losses are common.** The contraction-mapping argument requires strong convexity, which real losses do not satisfy globally. SCTT can certify local convergence, not global.
- **Scalability is untested.** Type-checking a transformer with billions of parameters against Lipschitz bounds is an open engineering challenge.

## Exercises

### Typing Neural Networks

1. Type a one-dimensional convolutional layer with kernel size 3 as a `Lip` type. What is the Lipschitz constant in terms of the kernel weights? (*Hint:* A 1D convolution is a linear map; compute its operator norm.)

2. Implement spectral normalization as a function `normalize : Matrix m n → (k : ℝ) → Σ (W' : Matrix m n), operator_norm W' ≤ k`. Show that applying it twice is idempotent: `normalize (fst (normalize W k)) k ≡ normalize W k`.

3. Type a residual network with `L` blocks, each having internal Lipschitz constant at most `k_block`. Express the total Lipschitz constant. For `L = 50` and `k_block = 0.1`, compare this bound with a plain feedforward network of the same depth and per-layer constant `1 + k_block`.

### Neural ODEs

4. Implement a neural ODE solver that takes dynamics `f : Lip(ℝⁿ × ℝ, ℝⁿ, k)`, an initial condition `h0 : ℝⁿ`, and a time horizon `T : ℝ`, and returns a `SmoothPath ℝⁿ`. Verify that the Lipschitz bound on `f` implies the solution exists and is unique.

5. Implement the adjoint method for computing `∂L/∂θ` given a neural ODE trajectory and a smooth loss. Show that the parameter gradient is exact (not dependent on ODE solver step size).

6. For a neural ODE with Lipschitz constant `k = 3` on the dynamics, use Gronwall's inequality to bound the divergence of two trajectories starting `δ = 0.01` apart, at times `t = 0.5, 1.0, 2.0`. At what time horizon does the bound exceed 1.0?

### Certified Robustness

7. A trained network has Lipschitz constant `k = 5.0` and classifies an input `x` with margin `m = 0.3`. What is the certified robustness radius? If we reduce `k` to 2.0 via spectral normalization (possibly reducing accuracy), what is the new radius?

8. Design a training objective that jointly minimizes classification loss and Lipschitz constant. Write this as an SCTT type and show that the Pareto frontier between accuracy and robustness is navigable by adjusting a single hyperparameter.

9. Implement a verified adversarial attack detector: given a Lip-typed network, an input `x`, and a perturbation `δ`, either certify that the classification is robust or flag the input as potentially adversarial. What is the computational cost of this check?

### The CHH Correspondence

10. Formalize the CHH correspondence for a toy problem: a linear regression model with squared loss. Show that the global minimizer (the OLS solution) is the unique "proof" of the specification, and that gradient descent provably converges to it.

11. Show that gradient descent on a `μ`-strongly convex, `L`-smooth loss with learning rate `η = 2/(μ + L)` defines a contraction mapping in the `Lip` type. Compute the contraction rate and the number of steps to reach ε-accuracy.

12. The CHH correspondence suggests "generalization = parametricity." For a network typed as `∀ (R : Representation), C∞(R, Label)`, explain what parametricity forces: the network cannot inspect the internal structure of the representation, only its interface. How does this connect to the common ML observation that representations learned by intermediate layers are often transferable?

---

*Previous: [Chapter 16: Building the v0 Kernel](./chapter_16.md) | Next: [Chapter 18: The Equational Theory Frontier](./chapter_18.md)*
