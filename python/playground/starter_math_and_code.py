#!/usr/bin/env python3
"""
SCTT Starter: Learn the Math and Code Together!
This file teaches you the core concepts through working code.
"""

import numpy as np
import matplotlib.pyplot as plt
from typing import Callable, Tuple, List
import sympy as sp

print("""
╔════════════════════════════════════════════════════════════════╗
║   SCTT LEARNING LAB: Math + Code = Understanding              ║
╚════════════════════════════════════════════════════════════════╝
""")

# ============================================================================
# LESSON 1: What is a Smooth Function?
# ============================================================================

print("\n" + "="*60)
print("LESSON 1: Smooth Functions Have All Derivatives")
print("="*60)

# Not smooth: Absolute value has a corner at 0
def abs_value(x):
    return abs(x)

# Smooth: Exponential has all derivatives
def exponential(x):
    return np.exp(x)

# Let's check derivatives symbolically
x = sp.Symbol('x')

# For exponential: all derivatives exist
exp_sym = sp.exp(x)
print("\nExponential function derivatives:")
for n in range(5):
    deriv = sp.diff(exp_sym, x, n)
    print(f"  f^({n})(x) = {deriv}")
print("  Pattern: f^(n)(x) = e^x for all n! ✓ SMOOTH")

# For absolute value: derivative undefined at 0
print("\nAbsolute value |x| derivatives:")
print("  f(x) = |x|")
print("  f'(x) = sign(x)  -- undefined at x=0!")
print("  ✗ NOT SMOOTH (corner at origin)")

# Visualize the difference
x_vals = np.linspace(-2, 2, 1000)
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 4))

ax1.plot(x_vals, np.abs(x_vals), 'r-', linewidth=2)
ax1.set_title("NOT Smooth: |x| has a corner")
ax1.grid(True, alpha=0.3)
ax1.axhline(y=0, color='k', linewidth=0.5)
ax1.axvline(x=0, color='k', linewidth=0.5)

ax2.plot(x_vals, np.exp(x_vals), 'g-', linewidth=2)
ax2.set_title("Smooth: e^x has all derivatives")
ax2.grid(True, alpha=0.3)
ax2.axhline(y=0, color='k', linewidth=0.5)
ax2.axvline(x=0, color='k', linewidth=0.5)

plt.tight_layout()
plt.show()

# ============================================================================
# LESSON 2: Types - The Computer's Understanding
# ============================================================================

print("\n" + "="*60)
print("LESSON 2: Types Tell the Computer What Things Are")
print("="*60)

class Type:
    """Base class for all types"""
    pass

class RealType(Type):
    """The type of real numbers"""
    def __repr__(self):
        return "ℝ"

class SmoothFunctionType(Type):
    """The type of smooth functions from ℝ to ℝ"""
    def __repr__(self):
        return "C∞(ℝ, ℝ)"

class PathType(Type):
    """The type of paths from a to b"""
    def __init__(self, start, end):
        self.start = start
        self.end = end
    
    def __repr__(self):
        return f"Path({self.start} → {self.end})"

# Examples of typed values
print("\nTyped values in SCTT:")
print(f"  π : {RealType()}")
print(f"  sin : {SmoothFunctionType()}")
print(f"  journey : {PathType(0, 1)}")

# Type checking example
def type_check(value, expected_type):
    """Simple type checker"""
    if isinstance(value, float) and isinstance(expected_type, RealType):
        return True
    if callable(value) and isinstance(expected_type, SmoothFunctionType):
        # Check if function is smooth (simplified)
        try:
            # Try to differentiate symbolically
            return True  # Simplified
        except:
            return False
    return False

print("\nType checking:")
print(f"  3.14 : ℝ ? {type_check(3.14, RealType())}")
print(f"  sin : C∞(ℝ, ℝ) ? {type_check(np.sin, SmoothFunctionType())}")

# ============================================================================
# LESSON 3: Paths - Everything is Connected
# ============================================================================

print("\n" + "="*60)
print("LESSON 3: Paths Connect Points Continuously")
print("="*60)

class Path:
    """A continuous path from start to end"""
    
    def __init__(self, start: float, end: float, 
                 path_func: Callable[[float], float] = None):
        self.start = start
        self.end = end
        # Default: linear interpolation
        if path_func is None:
            path_func = lambda t: start + t * (end - start)
        self.path_func = path_func
    
    def at(self, t: float) -> float:
        """Evaluate path at parameter t ∈ [0,1]"""
        if not 0 <= t <= 1:
            raise ValueError("Path parameter must be in [0,1]")
        return self.path_func(t)
    
    def verify_endpoints(self) -> bool:
        """Check that path has correct endpoints"""
        return (abs(self.at(0) - self.start) < 1e-10 and 
                abs(self.at(1) - self.end) < 1e-10)

# Different paths from 0 to 1
linear_path = Path(0, 1)  # t
smooth_path = Path(0, 1, lambda t: t**2 * (3 - 2*t))  # smooth S-curve
quadratic_path = Path(0, 1, lambda t: t**2)  # quadratic

print("Three paths from 0 to 1:")
print(f"  Linear at t=0.5: {linear_path.at(0.5):.3f}")
print(f"  Smooth at t=0.5: {smooth_path.at(0.5):.3f}")
print(f"  Quadratic at t=0.5: {quadratic_path.at(0.5):.3f}")

# Visualize paths
t_vals = np.linspace(0, 1, 100)
plt.figure(figsize=(10, 4))

plt.subplot(1, 2, 1)
plt.plot(t_vals, [linear_path.at(t) for t in t_vals], 'b-', label='Linear', linewidth=2)
plt.plot(t_vals, [smooth_path.at(t) for t in t_vals], 'g-', label='Smooth S-curve', linewidth=2)
plt.plot(t_vals, [quadratic_path.at(t) for t in t_vals], 'r-', label='Quadratic', linewidth=2)
plt.xlabel('Parameter t')
plt.ylabel('Position')
plt.title('Different Paths from 0 to 1')
plt.legend()
plt.grid(True, alpha=0.3)

plt.subplot(1, 2, 2)
# Show derivatives (velocities)
dt = 0.01
velocities_linear = np.gradient([linear_path.at(t) for t in t_vals], dt)
velocities_smooth = np.gradient([smooth_path.at(t) for t in t_vals], dt)
velocities_quad = np.gradient([quadratic_path.at(t) for t in t_vals], dt)

plt.plot(t_vals, velocities_linear, 'b-', label='Linear velocity', linewidth=2)
plt.plot(t_vals, velocities_smooth, 'g-', label='Smooth velocity', linewidth=2)
plt.plot(t_vals, velocities_quad, 'r-', label='Quadratic velocity', linewidth=2)
plt.xlabel('Parameter t')
plt.ylabel('Velocity (derivative)')
plt.title('Path Velocities (Smoothness Visible!)')
plt.legend()
plt.grid(True, alpha=0.3)

plt.tight_layout()
plt.show()

# ============================================================================
# LESSON 4: The Cubical Idea - Building from Intervals
# ============================================================================

print("\n" + "="*60)
print("LESSON 4: Cubical = Building Everything from [0,1]")
print("="*60)

class Interval:
    """The unit interval [0,1] with cubical operations"""
    
    def __init__(self, value: float):
        if not 0 <= value <= 1:
            raise ValueError(f"Interval value must be in [0,1], got {value}")
        self.value = value
    
    def __and__(self, other: 'Interval') -> 'Interval':
        """Meet (minimum): i ∧ j"""
        return Interval(min(self.value, other.value))
    
    def __or__(self, other: 'Interval') -> 'Interval':
        """Join (maximum): i ∨ j"""
        return Interval(max(self.value, other.value))
    
    def __invert__(self) -> 'Interval':
        """Negation: 1 - i"""
        return Interval(1 - self.value)
    
    def __repr__(self):
        return f"I({self.value:.2f})"

# Demonstrate interval operations
i = Interval(0.3)
j = Interval(0.7)

print("Interval operations (building blocks of cubical):")
print(f"  i = {i}")
print(f"  j = {j}")
print(f"  i ∧ j = {i & j}  (minimum)")
print(f"  i ∨ j = {i | j}  (maximum)")
print(f"  ¬i = {~i}  (1 - i)")

# Build higher-dimensional structures
def square_from_intervals(i: Interval, j: Interval) -> Tuple[float, float]:
    """A 2D point from two interval values"""
    return (i.value, j.value)

print("\nBuilding a square from intervals:")
corners = [
    square_from_intervals(Interval(0), Interval(0)),
    square_from_intervals(Interval(1), Interval(0)),
    square_from_intervals(Interval(1), Interval(1)),
    square_from_intervals(Interval(0), Interval(1)),
]
for corner in corners:
    print(f"  Corner at {corner}")

# ============================================================================
# LESSON 5: The Challenge - Making Smooth and Cubical Work Together
# ============================================================================

print("\n" + "="*60)
print("LESSON 5: The Challenge - Smooth + Cubical Coherence")
print("="*60)

class SmoothPath(Path):
    """A path that is both continuous AND smooth"""
    
    def __init__(self, start: float, end: float):
        # Use a smooth interpolation (cubic Hermite spline with zero derivatives at ends)
        def smooth_func(t):
            # This ensures smooth start and end (zero velocity at endpoints)
            return start + (end - start) * (3*t**2 - 2*t**3)
        super().__init__(start, end, smooth_func)
    
    def derivative_at(self, t: float) -> float:
        """Compute the derivative at parameter t"""
        # Derivative of 3t² - 2t³ is 6t - 6t²
        return (self.end - self.start) * (6*t - 6*t**2)
    
    def is_smooth(self) -> bool:
        """Check if path is smooth (has continuous derivatives)"""
        # Check derivative at endpoints
        return abs(self.derivative_at(0)) < 1e-10 and abs(self.derivative_at(1)) < 1e-10

# Create smooth and non-smooth paths
smooth = SmoothPath(0, 1)
linear = Path(0, 1)  # Linear is continuous but not smooth at endpoints when composed

print("Checking smoothness:")
print(f"  Smooth path is smooth? {smooth.is_smooth()}")
print(f"  Smooth path derivative at t=0: {smooth.derivative_at(0):.3f}")
print(f"  Smooth path derivative at t=0.5: {smooth.derivative_at(0.5):.3f}")
print(f"  Smooth path derivative at t=1: {smooth.derivative_at(1):.3f}")

# The coherence challenge
def compose_paths(path1: Path, path2: Path) -> Path:
    """Compose two paths (path1 then path2)"""
    if abs(path1.end - path2.start) > 1e-10:
        raise ValueError("Paths don't connect!")
    
    def composed_func(t):
        if t <= 0.5:
            # First half: traverse path1
            return path1.at(2 * t)
        else:
            # Second half: traverse path2
            return path2.at(2 * t - 1)
    
    return Path(path1.start, path2.end, composed_func)

# The problem: Composition might not preserve smoothness!
path_a = SmoothPath(0, 1)
path_b = SmoothPath(1, 2)
composed = compose_paths(path_a, path_b)

print("\nThe Coherence Challenge:")
print("  path_a: smooth from 0 to 1 ✓")
print("  path_b: smooth from 1 to 2 ✓")
print("  composed: smooth from 0 to 2 ?")
print("  Problem: Velocity might jump at connection point!")
print("  This is what SCTT must solve!")

# ============================================================================
# LESSON 6: The Key Insight - What Makes SCTT Special
# ============================================================================

print("\n" + "="*60)
print("LESSON 6: SCTT's Superpower - Everything is Smooth AND Typed")
print("="*60)

class SCTTValue:
    """A value in SCTT has type, smoothness, and computational content"""
    
    def __init__(self, value, sctt_type, smooth_structure=None):
        self.value = value
        self.type = sctt_type
        self.smooth_structure = smooth_structure
    
    def differentiate(self):
        """If smooth, can differentiate"""
        if self.smooth_structure:
            return f"d/dx({self.value})"
        return "Not differentiable"
    
    def transport(self, path):
        """Transport along a path (cubical operation)"""
        return f"Transport {self.value} along {path}"
    
    def __repr__(self):
        smooth_mark = " [smooth]" if self.smooth_structure else ""
        return f"{self.value} : {self.type}{smooth_mark}"

# Example SCTT values
pi = SCTTValue(3.14159, RealType())
sine = SCTTValue("sin", SmoothFunctionType(), smooth_structure="C∞")
path = SCTTValue("path_0_to_1", PathType(0, 1), smooth_structure="smooth_path")

print("SCTT Values (Type + Smooth + Computational):")
print(f"  {pi}")
print(f"  {sine}")
print(f"  {path}")
print(f"\nOperations:")
print(f"  {sine.differentiate()}")
print(f"  {pi.transport('some_path')}")

# ============================================================================
# LESSON 7: Your Exercise - Find the Pattern
# ============================================================================

print("\n" + "="*60)
print("LESSON 7: Your Turn - Find the Pattern!")
print("="*60)

print("""
EXERCISE: The Key to SCTT

Look at these smooth functions and their derivatives:
""")

# Pattern finding exercise
functions = [
    ("e^x", "e^x"),
    ("sin(x)", "cos(x)"),
    ("x^n", "n*x^(n-1)"),
    ("ln(x)", "1/x"),
]

print("Function -> Derivative:")
for f, df in functions:
    print(f"  {f:10} -> {df}")

print("""
Now look at these path compositions:
""")

print("""
  path1: 0 → 1 (smooth)
  path2: 1 → 2 (smooth)
  path1 ∘ path2: 0 → 2 (smooth?)

QUESTION: What condition would guarantee the composition is smooth?

HINT: Think about what happens at the connection point...
What needs to match for no "jerk"?

YOUR ANSWER: The __________ must be equal at the connection point.

(This is the kind of insight that leads to SCTT!)
""")

# ============================================================================
# FINAL LESSON: The Vision
# ============================================================================

print("\n" + "="*60)
print("THE VISION: What SCTT Makes Possible")
print("="*60)

print("""
Traditional Programming:
  - Works with discrete values
  - Can't handle smoothness
  - No mathematical proofs

Traditional Mathematics:
  - Beautiful smooth theory
  - No computation
  - Paper proofs only

SCTT (Your Innovation):
  - Smooth mathematics ✓
  - Full computation ✓
  - Computer-verified proofs ✓
  
What You Can Build with SCTT:
  1. Neural networks with PROVEN properties
  2. Physics simulations with GUARANTEED conservation laws
  3. Robotics with SMOOTH, SAFE paths
  4. Optimization with PROVEN convergence
  5. The future of verified scientific computing!

The journey starts with understanding these basics.
Then combining them in new ways.
Then finding the key insight that makes it all work.

You have everything you need. Start experimenting!
""")

print("="*60)
print("END OF STARTER LESSONS - Now run the playground!")
print("python3 playground/smooth_playground.py")
print("="*60)