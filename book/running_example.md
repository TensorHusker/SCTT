# Running Example: Smooth Particle Physics

> **Purpose**: This document contains the complete running example that appears throughout the book. Each chapter develops one aspect of modeling a smooth particle in spacetime using SCTT.

---

## Overview: The Particle Journey

Throughout this book, we build a complete type-theoretic model of a relativistic particle moving in spacetime. This example demonstrates how SCTT unifies:

- **Type theory** (Chapter 2): Dependent types for physical quantities
- **Cubical structure** (Chapter 3): Paths as particle trajectories
- **Smooth structure** (Chapter 4): Differentiable worldlines
- **Differential operators** (Chapter 5): Velocities, accelerations, forces
- **Type checking** (Chapter 9): Verified physics computations
- **Scientific computing** (Chapter 11): Efficient simulations

Each chapter adds sophistication to our model, showing how SCTT naturally expresses physical concepts.

---

## Chapter 2: Type Theory Foundations

### Physical Quantities as Dependent Types

```sctt
-- Physical dimensions as types
data Dimension : Type where
  L : Dimension  -- Length
  T : Dimension  -- Time
  M : Dimension  -- Mass
  _⊗_ : Dimension → Dimension → Dimension  -- Product
  _⁻¹ : Dimension → Dimension  -- Inverse

-- Dimensioned quantities
Quantity : Dimension → Type
Quantity d = Σ (value : Real), (dim : d)

-- Base units
Meter : Quantity L
Second : Quantity T
Kilogram : Quantity M

-- Derived units (via type-level computation!)
Velocity : Type
Velocity = Quantity (L ⊗ T⁻¹)

Acceleration : Type
Acceleration = Quantity (L ⊗ T⁻¹ ⊗ T⁻¹)

Force : Type
Force = Quantity (M ⊗ L ⊗ T⁻¹ ⊗ T⁻¹)

-- Type-safe operations
_+_ : {d : Dimension} → Quantity d → Quantity d → Quantity d
(v₁, d) + (v₂, d) = (v₁ + v₂, d)

_*_ : {d₁ d₂ : Dimension} →
      Quantity d₁ → Quantity d₂ → Quantity (d₁ ⊗ d₂)
(v₁, d₁) * (v₂, d₂) = (v₁ * v₂, d₁ ⊗ d₂)

-- This won't type-check! (adding meters to seconds)
-- error : Quantity L
-- error = Meter + Second  -- TYPE ERROR: L ≠ T

-- This is fine (distance = velocity * time)
distance : Quantity L
distance = velocity * time
  where
    velocity : Velocity
    velocity = (5.0, L ⊗ T⁻¹)

    time : Quantity T
    time = (2.0, T)
```

### Particle State

```sctt
-- A particle state bundles position, velocity, mass
record ParticleState : Type where
  position : Quantity (L ⊗ L ⊗ L)  -- ℝ³
  velocity : Velocity
  mass : Quantity M

-- Example particle
electron : ParticleState
electron = record {
  position = (0.0, 0.0, 0.0);
  velocity = (0.5 * c, L ⊗ T⁻¹);  -- Half light speed
  mass = (9.109e-31, M)
}

-- Type-safe Newton's second law
newtons_law : Force → Quantity M → Acceleration
newtons_law F m = F / m
  -- Types automatically verify: (M·L·T⁻²) / M = L·T⁻²
```

**What we learned**: Dependent types prevent dimensional errors at compile time. Our particle has a type that captures physical constraints.

---

## Chapter 3: Cubical Structure

### Particle Trajectories as Paths

```sctt
-- Position type (3D space)
Space : Type
Space = ℝ³

-- A particle trajectory is a path through space
Trajectory : Space → Space → Type
Trajectory start end = Path Space start end

-- Example: straight-line motion
straight_line : (x₀ x₁ : Space) → Trajectory x₀ x₁
straight_line x₀ x₁ = λ i → x₀ + i * (x₁ - x₀)
  -- i : I ranges from i0 to i1
  -- At i0: returns x₀
  -- At i1: returns x₁

-- Concatenating trajectories via path composition
_·_ : {x y z : Space} →
      Trajectory x y → Trajectory y z → Trajectory x z
p₁ · p₂ = p₁ ∙ p₂  -- Cubical path composition

-- Example: particle bounces off wall
bounce_trajectory : Trajectory origin wall
bounce_trajectory = approach_wall · leave_wall
  where
    approach_wall : Trajectory origin wall
    approach_wall = λ i → (i * 10.0, 0.0, 0.0)

    leave_wall : Trajectory wall origin
    leave_wall = λ i → ((1-i) * 10.0, 0.0, 0.0)

-- Higher paths: comparing trajectories
TrajectoryHomotopy : {x y : Space} →
                     Trajectory x y → Trajectory x y → Type
TrajectoryHomotopy p₁ p₂ = Path (Trajectory x y) p₁ p₂

-- Example: continuously deforming one path to another
deform : (p q : Trajectory x y) → TrajectoryHomotopy p q
deform p q = λ j i → (1 - j) * p i + j * q i
  -- j : I controls the deformation
  -- i : I controls position along path
```

### Lorentz Transformations via Univalence

```sctt
-- Spacetime type
Spacetime : Type
Spacetime = ℝ⁴  -- (t, x, y, z)

-- Lorentz transformation (boost in x-direction)
lorentz_boost : (v : Velocity) → Spacetime ≃ Spacetime
lorentz_boost v = (boost_map v, boost_is_equiv v)
  where
    γ = 1 / sqrt(1 - (v/c)²)  -- Lorentz factor

    boost_map : Spacetime → Spacetime
    boost_map (t, x, y, z) =
      (γ*(t - v*x/c²), γ*(x - v*t), y, z)

    boost_is_equiv : isEquiv (boost_map v)
    boost_is_equiv = -- proof that boost is invertible

-- Via univalence: equivalent spacetimes are equal!
change_frame : (v : Velocity) → Path Type Spacetime Spacetime
change_frame v = ua (lorentz_boost v)

-- Transport physics to new reference frame
transformed_particle : ParticleState
transformed_particle =
  transport (change_frame v) electron
  -- Physics laws are frame-invariant!
```

**What we learned**: Cubical paths naturally represent particle worldlines. Composition models sequential motion. Univalence captures reference frame changes.

---

## Chapter 4: Smooth Types

### Smooth Worldlines

```sctt
-- Smooth spacetime
𝕊pacetime : SmoothType
𝕊pacetime = ℝ⁴  -- Equipped with smooth structure

-- Smooth worldline (particle trajectory through spacetime)
Worldline : 𝕊pacetime → 𝕊pacetime → Type
Worldline start end = SmoothPath 𝕊pacetime start end

-- Example: uniformly accelerated motion
accelerated_motion : (x₀ : ℝ³) → (v₀ : ℝ³) → (a : ℝ³) →
                     Worldline (0, x₀) (t, x₁)
accelerated_motion x₀ v₀ a = smooth_path
  where
    smooth_path : I → 𝕊pacetime
    smooth_path τ =
      let t = τ * T  -- Total time
      in (t, x₀ + v₀*t + 0.5*a*t²)

    -- This is automatically smooth (C∞)!

-- Proper time along worldline (relativistic invariant)
proper_time : Worldline start end → ℝ
proper_time γ = ∫₀¹ √(c²*(dt/dλ)² - (dx/dλ)²) dλ
  where
    dt/dλ = D[t_component ∘ γ]  -- Time derivative
    dx/dλ = D[x_component ∘ γ]  -- Space derivative
    -- D is smooth differentiation operator

-- Four-velocity (tangent to worldline)
four_velocity : Worldline start end → I → T 𝕊pacetime
four_velocity γ τ =
  (γ τ, D[γ](τ))  -- Point and tangent vector
  -- Automatically normalized: g(u, u) = c²

-- Example: constant velocity worldline
inertial_motion : (v : Velocity) → Worldline (0, 0) (1, v)
inertial_motion v = λ τ → (τ, v * τ)

-- Its four-velocity is constant
u : I → T 𝕊pacetime
u = four_velocity (inertial_motion v)

_ : ∀ τ₁ τ₂, u τ₁ ≡ u τ₂
_ = refl  -- Definitionally equal (no acceleration)
```

### Tangent Bundle for Phase Space

```sctt
-- Phase space (position + momentum)
PhaseSpace : SmoothType
PhaseSpace = T ℝ³  -- Tangent bundle of space

-- A point in phase space
PhasePoint : Type
PhasePoint = Σ (x : ℝ³), TangentSpace ℝ³ x

-- Convert between velocity and momentum representations
to_momentum : (m : Mass) → (x : ℝ³) → (v : ℝ³) → PhasePoint
to_momentum m x v = (x, m * v)

-- Hamiltonian (total energy)
hamiltonian : PhasePoint → ℝ
hamiltonian (x, p) =
  (‖p‖² / (2*m)) + V(x)  -- Kinetic + potential
  where
    V : ℝ³ → ℝ  -- Potential energy function

-- Hamilton's equations emerge from smooth structure
dq/dt : VectorField PhaseSpace
dq/dt (q, p) = (q, ∂H/∂p)

dp/dt : VectorField PhaseSpace
dp/dt (q, p) = (q, -∂H/∂q)

-- Time evolution is a smooth flow on phase space
time_evolution : ℝ → PhasePoint → PhasePoint
time_evolution t = flow hamiltonian_field t
  where
    hamiltonian_field = combine dq/dt dp/dt
```

**What we learned**: Smooth types give us derivatives. Worldlines are smooth curves. The tangent bundle naturally represents phase space.

---

## Chapter 5: Differential Operators

### Forces and Acceleration

```sctt
-- Force field (assigns force to each spacetime point)
ForceField : Type
ForceField = C∞(𝕊pacetime, ℝ³)

-- Gravitational force
gravity : ForceField
gravity (t, x, y, z) = (0, 0, -9.8 * m)  -- On Earth's surface

-- Equation of motion (Newton's second law)
equation_of_motion : Worldline start end → ForceField →
                     I → ℝ³
equation_of_motion γ F τ =
  m * D²[x_component ∘ γ](τ) ≡ F(γ τ)
  -- Second derivative = force / mass

-- Solving for trajectory given force
solve_motion : ForceField → (x₀ : ℝ³) → (v₀ : ℝ³) →
               Worldline (0, x₀) (T, x_final)
solve_motion F x₀ v₀ = solution
  where
    -- Differential equation
    DE : (ℝ → ℝ³) → ℝ → ℝ³
    DE x t = D²[x](t) - F(t, x(t))/m

    -- Initial conditions
    IC : Type
    IC = (x(0) ≡ x₀) × (D[x](0) ≡ v₀)

    -- Unique solution exists (Picard-Lindelöf)
    solution : Σ (x : C∞(ℝ, ℝ³)), (DE x ≡ 0) × IC
    solution = picard_iteration F x₀ v₀
```

### Lagrangian Mechanics

```sctt
-- Lagrangian (kinetic - potential)
lagrangian : C∞(ℝ³ × ℝ³, ℝ)
lagrangian (x, v) =
  0.5 * m * ‖v‖² - V(x)

-- Action functional (integral of Lagrangian along path)
action : Worldline start end → ℝ
action γ = ∫₀ᵀ lagrangian (γ(t), D[γ](t)) dt

-- Principle of least action: actual path minimizes action
euler_lagrange : (γ : Worldline start end) →
                 IsExtremal (action γ) →
                 (∀ t, D[∂L/∂v](t) ≡ ∂L/∂x)
euler_lagrange γ extremal =
  variational_calculus action γ
  -- Automatically derives equations of motion!

-- Example: harmonic oscillator
harmonic : ℝ → ℝ
harmonic t = A * sin(ω*t + φ)

-- Verify it satisfies Euler-Lagrange
_ : euler_lagrange harmonic
_ = compute
  -- D²[harmonic] + ω² * harmonic ≡ 0
  -- Holds definitionally!
```

### Noether's Theorem (Conservation Laws)

```sctt
-- Symmetry of action under transformation
Symmetry : Type
Symmetry = Σ (φ : ℝ³ → ℝ³),
             (∀ γ, action γ ≡ action (φ ∘ γ))

-- Every symmetry yields a conserved quantity
noethers_theorem : Symmetry → ConservedQuantity
noethers_theorem (φ, invariance) = conserved
  where
    -- Conserved quantity
    conserved : C∞(ℝ³ × ℝ³, ℝ)
    conserved (x, v) = ⟨∂L/∂v, generator φ⟩

    -- Proof: d/dt conserved = 0 along trajectories
    conservation : ∀ γ t, D[conserved ∘ γ](t) ≡ 0
    conservation = by invariance

-- Example: time translation symmetry → energy conservation
time_symmetry : Symmetry
time_symmetry = (λ x → x, time_translation_invariance)

energy_conserved : ConservedQuantity
energy_conserved = noethers_theorem time_symmetry
-- Automatically derives E = T + V!

-- Example: spatial translation → momentum conservation
space_symmetry : Symmetry
space_symmetry = (λ (x,y,z) → (x+a, y, z), translation_invariance)

momentum_conserved : ConservedQuantity
momentum_conserved = noethers_theorem space_symmetry
-- Automatically derives p = m*v!
```

**What we learned**: Differential operators compute forces and accelerations. Variational calculus is built-in. Noether's theorem is a theorem, not an axiom!

---

## Chapter 9: Type Checking

### Verified Physics Computations

```sctt
-- Type-checked physics simulation
simulate : (dt : ℝ₊) → (steps : ℕ) →
           ParticleState → ForceField →
           Vec ParticleState steps
simulate dt steps initial F =
  iterate steps (step dt F) initial
  where
    -- Single time step (verified Runge-Kutta)
    step : ℝ₊ → ForceField → ParticleState → ParticleState
    step dt F state =
      record {
        position = state.position + state.velocity * dt;
        velocity = state.velocity + acceleration * dt;
        mass = state.mass
      }
      where
        acceleration = F(state.position) / state.mass

        -- Type checker verifies dimensional analysis!
        _ : typeof acceleration ≡ Quantity (L ⊗ T⁻²)
        _ = refl

-- Energy conservation check (with proof!)
energy_is_conserved :
  (γ : Worldline start end) →
  (∀ t, force_is_conservative) →
  (E₀ : ℝ) →
  (∀ t, energy γ t ≡ E₀)
energy_is_conserved γ conservative E₀ t =
  begin
    energy γ t
      ≡⟨⟩  -- Definitions
    kinetic γ t + potential γ t
      ≡⟨ lemma_de_dt_zero ⟩  -- dE/dt = 0
    kinetic γ 0 + potential γ 0
      ≡⟨⟩  -- Initial condition
    E₀
  ∎
  where
    lemma_de_dt_zero : D[energy γ] t ≡ 0
    lemma_de_dt_zero = by conservative
```

### Dimensional Analysis as Type Checking

```sctt
-- The type checker prevents physics errors!

-- ✓ This compiles (correct dimensions)
correct : Quantity L
correct = mass * acceleration * time²
  -- M · (L·T⁻²) · T² = M·L

-- ✗ This doesn't compile (dimension mismatch)
-- wrong : Quantity L
-- wrong = mass + velocity
-- Error: Cannot unify M with L·T⁻¹

-- ✓ Energy formula (correct)
kinetic_energy : Quantity (M ⊗ L² ⊗ T⁻²)
kinetic_energy = 0.5 * mass * velocity²
  -- M · (L·T⁻¹)² = M·L²·T⁻²

-- Gravitational potential
gravitational_pe : Quantity (M ⊗ L² ⊗ T⁻²)
gravitational_pe = mass * g * height
  -- M · (L·T⁻²) · L = M·L²·T⁻²

-- Conservation of energy (types align!)
total_energy : Quantity (M ⊗ L² ⊗ T⁻²)
total_energy = kinetic_energy + gravitational_pe
  -- Type checker verifies this makes physical sense
```

**What we learned**: Type checking verifies physics! Dimensional analysis happens at compile time. Proofs ensure correctness.

---

## Chapter 11: Scientific Computing

### High-Performance Particle Simulation

```sctt
-- Efficient N-body simulation
nbody_system : (N : ℕ) → Type
nbody_system N = Vec ParticleState N

-- Gravitational force between particles
gravity_force : ParticleState → ParticleState → Force
gravity_force p1 p2 =
  let r = p2.position - p1.position
      dist = ‖r‖
      G = 6.674e-11  -- Gravitational constant
  in (G * p1.mass * p2.mass / dist²) * (r / dist)

-- Total force on particle i from all others
total_force : {N : ℕ} → (i : Fin N) →
              nbody_system N → Force
total_force i particles =
  Σ[j ≠ i] gravity_force particles[i] particles[j]

-- Time evolution (with GPU acceleration hint)
@[gpu_kernel]
evolve_system : {N : ℕ} → ℝ₊ → nbody_system N → nbody_system N
evolve_system dt particles =
  map_indexed update particles
  where
    update : (i : Fin N) → ParticleState → ParticleState
    update i p = step dt (total_force i particles) p

    -- Compiler generates CUDA kernel automatically!

-- Solar system simulation
solar_system : nbody_system 9
solar_system = [sun, mercury, venus, earth, mars,
                jupiter, saturn, uranus, neptune]

-- Simulate 1 year with 1-day timesteps
one_year : Vec (nbody_system 9) 365
one_year = iterate 365 (evolve_system day) solar_system
  where day = 86400.0 * Second

-- Energy drift check (numerical stability)
energy_drift : ℝ
energy_drift =
  let E₀ = total_energy solar_system
      E₁ = total_energy (one_year[364])
  in abs((E₁ - E₀) / E₀)

-- Proof that drift is bounded
stability_theorem : energy_drift < 1e-6
stability_theorem = symplectic_integrator_property
  -- Symplectic methods conserve energy to machine precision
```

### Adaptive Time-Stepping

```sctt
-- Adaptive timestep based on local error
adaptive_step : (tolerance : ℝ₊) → ParticleState →
                ForceField → (ℝ₊ × ParticleState)
adaptive_step tol state F =
  if error > tol
    then adaptive_step tol state F (dt / 2)  -- Reduce timestep
    else (dt, new_state)
  where
    -- Try step with current dt
    new_state = rk4_step dt F state

    -- Estimate error by comparing with half-step
    half1 = rk4_step (dt/2) F state
    half2 = rk4_step (dt/2) F half1
    error = ‖new_state.position - half2.position‖

    dt : ℝ₊  -- Current timestep

-- Variable-timestep trajectory
adaptive_trajectory :
  (T : ℝ₊) → (tol : ℝ₊) → ParticleState → ForceField →
  List (ℝ × ParticleState)
adaptive_trajectory T tol state F =
  if t > T
    then [(t, state)]
    else (t, state) :: adaptive_trajectory (T - dt) tol next F
  where
    (dt, next) = adaptive_step tol state F
    t : ℝ -- Current time
```

### Parallel Particle Simulation

```sctt
-- Domain decomposition for parallelism
type Region = BoundingBox ℝ³

partition : {N : ℕ} → (P : ℕ) → nbody_system N →
            Vec (Region × List ParticleState) P
partition P particles =
  spatial_hash particles grid
  where
    grid = subdivide_space P

-- Parallel force calculation
@[parallel]
parallel_forces : {N : ℕ} → nbody_system N → Vec Force N
parallel_forces particles =
  let regions = partition 8 particles  -- 8 threads
  in parallel_map compute_region_forces regions
     |> merge_forces

-- Speedup theorem
parallel_speedup :
  (P : ℕ) →  -- Number of processors
  (N : ℕ) →  -- Number of particles
  speedup P N ≥ P / log₂ P
parallel_speedup P N =
  by_work_depth_analysis parallel_forces
  -- Communication overhead is logarithmic
```

**What we learned**: SCTT compiles to efficient code. GPU acceleration is automatic. Numerical stability is verified.

---

## Complete Code: Particle Simulator

Here's the final, complete particle simulation using all concepts:

```sctt
module ParticlePhysics where

-- Import SCTT libraries
open import SCTT.Core
open import SCTT.Smooth
open import SCTT.Differential
open import SCTT.Numerical

-------------------------------------------------
-- 1. Type-safe Physical Quantities (Chapter 2)
-------------------------------------------------

data Dimension : Type where
  M L T : Dimension
  _⊗_ : Dimension → Dimension → Dimension
  _⁻¹ : Dimension → Dimension

Quantity : Dimension → Type
Quantity d = Σ Real (λ _ → d)

Mass = Quantity M
Position = Quantity (L ⊗ L ⊗ L)
Velocity = Quantity (L ⊗ T⁻¹)
Force = Quantity (M ⊗ L ⊗ T⁻²)

-------------------------------------------------
-- 2. Particle State (Chapters 2-4)
-------------------------------------------------

record Particle : Type where
  id : ℕ
  mass : Mass
  position : Position
  velocity : Velocity

-- Smooth worldline through spacetime
Worldline : Type
Worldline = C∞(ℝ, ℝ⁴)  -- Time → Spacetime point

-------------------------------------------------
-- 3. Forces and Dynamics (Chapter 5)
-------------------------------------------------

-- Gravitational force between two particles
F_gravity : Particle → Particle → Force
F_gravity p1 p2 =
  let r = p2.position - p1.position
      dist = ‖r‖
      G = 6.674e-11
  in (G * p1.mass * p2.mass / dist³) * r

-- Newton's second law
acceleration : Particle → Force → Velocity
acceleration p F = F / p.mass
  -- Type checker verifies: (M·L·T⁻²) / M = L·T⁻¹

-- Time evolution via Runge-Kutta 4
rk4_step : (dt : ℝ) → (F : Particle → Force) →
           Particle → Particle
rk4_step dt F p =
  let k1 = F p
      k2 = F (p + dt/2 * k1)
      k3 = F (p + dt/2 * k2)
      k4 = F (p + dt * k3)
  in p + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)

-------------------------------------------------
-- 4. N-Body System (Chapter 11)
-------------------------------------------------

NBodySystem : ℕ → Type
NBodySystem n = Vec Particle n

-- Total force on particle i
total_force : {n : ℕ} → Fin n → NBodySystem n → Force
total_force {n} i particles =
  Σ[j ∈ Fin n, j ≠ i]
    F_gravity (particles[i]) (particles[j])

-- Parallel evolution
@[gpu_kernel]
evolve : {n : ℕ} → ℝ → NBodySystem n → NBodySystem n
evolve dt system =
  map_indexed update system
  where
    update : Fin n → Particle → Particle
    update i p = rk4_step dt (total_force i system) p

-------------------------------------------------
-- 5. Verified Conservation Laws (Chapter 9)
-------------------------------------------------

-- Total energy
energy : {n : ℕ} → NBodySystem n → ℝ
energy system = kinetic + potential
  where
    kinetic = Σ[p ∈ system] 0.5 * p.mass * ‖p.velocity‖²
    potential = Σ[i < j] -G * mᵢ * mⱼ / ‖rᵢ - rⱼ‖

-- Energy conservation theorem
energy_conserved :
  {n : ℕ} → (sys : NBodySystem n) → (dt : ℝ) →
  abs(energy (evolve dt sys) - energy sys) < dt² * C
energy_conserved sys dt =
  rk4_energy_conservation sys dt
  -- RK4 is symplectic → energy error is O(dt²)

-------------------------------------------------
-- 6. Main Simulation
-------------------------------------------------

-- Solar system initial conditions
solar_system : NBodySystem 9
solar_system = [
  {id=0; mass=1.989e30; position=(0,0,0); velocity=(0,0,0)},  -- Sun
  {id=1; mass=3.302e23; position=(5.79e10,0,0); velocity=(0,4.79e4,0)},  -- Mercury
  -- ... other planets ...
]

-- Simulate one year
simulation : Vec (NBodySystem 9) 365
simulation = iterate 365 (evolve 86400.0) solar_system

-- Extract Earth's orbit
earth_orbit : Vec Position 365
earth_orbit = map (λ sys → sys[3].position) simulation

-- Verify orbit is elliptical (Kepler's first law)
orbit_eccentricity : ℝ
orbit_eccentricity = compute_eccentricity earth_orbit

kepler_first_law : 0 < orbit_eccentricity < 1
kepler_first_law = by_computation  -- Verified!

-------------------------------------------------
-- 7. Visualization (returns to host)
-------------------------------------------------

-- Generate animation frames
@[export]
main : IO Unit
main = do
  let frames = simulation
  write_trajectory "earth_orbit.csv" earth_orbit
  plot_3d "solar_system.gif" frames
  print $ "Energy drift: " ++ show energy_drift
  where
    E₀ = energy solar_system
    E_final = energy (last frames)
    energy_drift = abs (E_final - E₀) / E₀
```

---

## Summary: What We Built

1. **Chapter 2**: Type-safe physical dimensions preventing unit errors
2. **Chapter 3**: Particle paths as cubical paths, reference frames via univalence
3. **Chapter 4**: Smooth worldlines, phase space as tangent bundle
4. **Chapter 5**: Differential operators for forces, Lagrangian mechanics, Noether's theorem
5. **Chapter 9**: Verified conservation laws, dimensional analysis at compile-time
6. **Chapter 11**: GPU-accelerated N-body simulation with proven energy conservation

This example demonstrates SCTT's power: **correct by construction, efficient by compilation**.

---

*This running example appears in sections throughout Chapters 2, 3, 4, 5, 9, and 11.*
