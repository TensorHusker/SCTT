#!/usr/bin/env python3
"""
SCTT Playground: Experiment with smooth and cubical concepts
This is your laboratory for building intuition about SCTT
"""

import numpy as np
import sympy as sp
from typing import Callable, Optional, Tuple, List
from dataclasses import dataclass
import matplotlib.pyplot as plt
from abc import ABC, abstractmethod

# ============================================================================
# PART 1: SMOOTH FUNCTIONS
# ============================================================================

class SmoothFunction:
    """Represent a smooth function with all its derivatives"""
    
    def __init__(self, expr_str: str, var: str = 'x'):
        self.var = sp.Symbol(var)
        self.expr = sp.sympify(expr_str)
        self._derivatives = [self.expr]
    
    def derivative(self, n: int = 1):
        """Get the nth derivative"""
        while len(self._derivatives) <= n:
            last_deriv = self._derivatives[-1]
            next_deriv = sp.diff(last_deriv, self.var)
            self._derivatives.append(next_deriv)
        return self._derivatives[n]
    
    def evaluate(self, x: float, deriv: int = 0) -> float:
        """Evaluate function or its derivative at a point"""
        expr = self.derivative(deriv)
        return float(expr.subs(self.var, x))
    
    def taylor_series(self, center: float, order: int) -> str:
        """Get Taylor series expansion"""
        series = 0
        for n in range(order + 1):
            coeff = self.evaluate(center, n) / np.math.factorial(n)
            series += coeff * (self.var - center)**n
        return str(sp.simplify(series))
    
    def compose(self, other: 'SmoothFunction') -> 'SmoothFunction':
        """Compose two smooth functions"""
        new_expr = self.expr.subs(self.var, other.expr)
        return SmoothFunction(str(new_expr), str(other.var))
    
    def __str__(self):
        return f"f(x) = {self.expr}"
    
    def plot(self, x_range=(-5, 5), include_derivatives=0):
        """Visualize the function and its derivatives"""
        x = np.linspace(x_range[0], x_range[1], 1000)
        
        fig, axes = plt.subplots(1, include_derivatives + 1, 
                                figsize=(5*(include_derivatives+1), 4))
        if include_derivatives == 0:
            axes = [axes]
        
        for i in range(include_derivatives + 1):
            y = [self.evaluate(xi, i) for xi in x]
            axes[i].plot(x, y)
            axes[i].grid(True, alpha=0.3)
            axes[i].set_title(f"f{'′'*i}(x)" if i > 0 else "f(x)")
            axes[i].axhline(y=0, color='k', linewidth=0.5)
            axes[i].axvline(x=0, color='k', linewidth=0.5)
        
        plt.tight_layout()
        plt.show()

# ============================================================================
# PART 2: INTERVAL AND PATHS
# ============================================================================

@dataclass
class Interval:
    """The unit interval [0,1] - fundamental to cubical type theory"""
    value: float
    
    def __post_init__(self):
        if not 0 <= self.value <= 1:
            raise ValueError(f"Interval value must be in [0,1], got {self.value}")
    
    def __and__(self, other: 'Interval') -> 'Interval':
        """Meet operation (minimum)"""
        return Interval(min(self.value, other.value))
    
    def __or__(self, other: 'Interval') -> 'Interval':
        """Join operation (maximum)"""
        return Interval(max(self.value, other.value))
    
    def __neg__(self) -> 'Interval':
        """Negation (1 - i)"""
        return Interval(1 - self.value)
    
    @classmethod
    def zero(cls):
        return cls(0.0)
    
    @classmethod
    def one(cls):
        return cls(1.0)

class Path:
    """A path between two points - can be smooth or discrete"""
    
    def __init__(self, start, end, path_fn: Optional[Callable] = None):
        self.start = start
        self.end = end
        self.path_fn = path_fn or (lambda t: start + t * (end - start))
    
    def __call__(self, t: float):
        """Evaluate path at parameter t ∈ [0,1]"""
        if not 0 <= t <= 1:
            raise ValueError(f"Path parameter must be in [0,1], got {t}")
        return self.path_fn(t)
    
    def compose(self, other: 'Path') -> 'Path':
        """Compose two paths (if endpoints match)"""
        if abs(self.end - other.start) > 1e-10:
            raise ValueError("Paths don't connect: endpoint mismatch")
        
        def composed_fn(t):
            if t <= 0.5:
                return self(2 * t)
            else:
                return other(2 * t - 1)
        
        return Path(self.start, other.end, composed_fn)
    
    def reverse(self) -> 'Path':
        """Reverse the path direction"""
        return Path(self.end, self.start, lambda t: self(1 - t))
    
    def is_smooth(self, sample_points: int = 100) -> bool:
        """Check if path is approximately smooth (no sharp turns)"""
        t_vals = np.linspace(0, 1, sample_points)
        vals = [self(t) for t in t_vals]
        
        # Check second differences (acceleration)
        first_diff = np.diff(vals)
        second_diff = np.diff(first_diff)
        
        # Smooth if second differences are small
        return np.max(np.abs(second_diff)) < 0.1
    
    def visualize(self):
        """Plot the path"""
        t_vals = np.linspace(0, 1, 100)
        vals = [self(t) for t in t_vals]
        
        plt.figure(figsize=(8, 4))
        plt.subplot(1, 2, 1)
        plt.plot(t_vals, vals)
        plt.xlabel('Parameter t')
        plt.ylabel('Value')
        plt.title('Path in parameter space')
        plt.grid(True, alpha=0.3)
        
        plt.subplot(1, 2, 2)
        plt.plot(vals, [0]*len(vals), 'b-', linewidth=2)
        plt.plot(self.start, 0, 'go', markersize=10, label='Start')
        plt.plot(self.end, 0, 'ro', markersize=10, label='End')
        plt.xlabel('Value')
        plt.title('Path visualization')
        plt.legend()
        plt.grid(True, alpha=0.3)
        
        plt.tight_layout()
        plt.show()

# ============================================================================
# PART 3: SMOOTH PATHS - THE KEY INNOVATION
# ============================================================================

class SmoothPath(Path):
    """A path that is guaranteed to be smooth (C∞)"""
    
    def __init__(self, start: float, end: float, smooth_fn: str = None):
        if smooth_fn:
            # Parse the smooth function
            t = sp.Symbol('t')
            expr = sp.sympify(smooth_fn)
            
            # Verify boundary conditions
            if abs(float(expr.subs(t, 0)) - start) > 1e-10:
                raise ValueError(f"Path doesn't start at {start}")
            if abs(float(expr.subs(t, 1)) - end) > 1e-10:
                raise ValueError(f"Path doesn't end at {end}")
            
            path_fn = lambda t_val: float(expr.subs(t, t_val))
        else:
            # Default smooth path (cubic)
            path_fn = lambda t: start + (end - start) * (3*t**2 - 2*t**3)
        
        super().__init__(start, end, path_fn)
        self.smooth_fn_str = smooth_fn or f"{start} + {end-start}*(3*t^2 - 2*t^3)"
    
    def derivative_at(self, t: float, n: int = 1) -> float:
        """Get nth derivative of the path at parameter t"""
        t_sym = sp.Symbol('t')
        expr = sp.sympify(self.smooth_fn_str.replace('^', '**'))
        
        deriv = expr
        for _ in range(n):
            deriv = sp.diff(deriv, t_sym)
        
        return float(deriv.subs(t_sym, t))
    
    def verify_smoothness(self) -> bool:
        """Verify the path is truly smooth at all points"""
        # Check continuity of first few derivatives
        t_vals = np.linspace(0, 1, 50)
        
        for deriv_order in range(3):
            derivs = [self.derivative_at(t, deriv_order) for t in t_vals]
            if np.max(np.abs(np.diff(derivs))) > 0.5:
                return False
        
        return True

# ============================================================================
# PART 4: COMPOSITION STRUCTURE
# ============================================================================

class CubicalComposition:
    """Implements composition operations from cubical type theory"""
    
    @staticmethod
    def hcomp(paths: List[Path], base_point) -> Path:
        """Homogeneous composition - fill in a square from its edges"""
        if len(paths) != 4:
            raise ValueError("Need exactly 4 paths for square composition")
        
        # paths = [top, right, bottom, left]
        # Verify they form a square
        if paths[0].start != paths[3].end:
            raise ValueError("Top-left corner doesn't match")
        if paths[0].end != paths[1].start:
            raise ValueError("Top-right corner doesn't match")
        if paths[1].end != paths[2].end:
            raise ValueError("Bottom-right corner doesn't match")
        if paths[2].start != paths[3].start:
            raise ValueError("Bottom-left corner doesn't match")
        
        # Create a 2D interpolation
        def surface(s, t):
            # Bilinear interpolation
            top_val = paths[0](s)
            bottom_val = paths[2](s)
            left_val = paths[3](t)
            right_val = paths[1](t)
            
            # Weighted average
            return (1-t)*left_val + t*right_val + (1-s)*bottom_val + s*top_val - \
                   (1-s)*(1-t)*paths[2].start - s*t*paths[0].end - \
                   (1-s)*t*paths[1].end - s*(1-t)*paths[3].end
        
        return surface
    
    @staticmethod
    def transport(path: Path, value_at_start):
        """Transport a value along a path"""
        # For simple types, transport is just following the path
        # For dependent types, this would be more complex
        return path(1)  # Value at the end

# ============================================================================
# PART 5: COHERENCE CHECKER
# ============================================================================

class CoherenceChecker:
    """Check if smooth and cubical structures are compatible"""
    
    @staticmethod
    def check_smooth_composition(f: SmoothFunction, g: SmoothFunction) -> bool:
        """Verify that composition preserves smoothness"""
        h = f.compose(g)
        
        # Check chain rule
        x_test = 2.0
        
        # (f∘g)'(x) should equal f'(g(x)) * g'(x)
        left = h.evaluate(x_test, 1)  # (f∘g)'(x)
        
        g_at_x = g.evaluate(x_test, 0)
        right = f.evaluate(g_at_x, 1) * g.evaluate(x_test, 1)  # f'(g(x)) * g'(x)
        
        return abs(left - right) < 1e-10
    
    @staticmethod
    def check_path_smoothness(path: Path) -> Tuple[bool, str]:
        """Check if a path maintains smooth structure"""
        if not isinstance(path, SmoothPath):
            is_smooth = path.is_smooth()
            if is_smooth:
                return True, "Path is approximately smooth"
            else:
                return False, "Path has discontinuous derivatives"
        
        if path.verify_smoothness():
            return True, "Path is C∞ smooth"
        else:
            return False, "Path claims to be smooth but isn't"
    
    @staticmethod
    def check_transport_smoothness(path: SmoothPath, smooth_value) -> bool:
        """Check if transport preserves smooth structure"""
        # In full SCTT, this would verify that transporting smooth values
        # along smooth paths yields smooth results
        # For now, we just check the path is smooth
        return path.verify_smoothness()

# ============================================================================
# PART 6: EXPERIMENTS AND EXERCISES
# ============================================================================

def experiment_1_smooth_functions():
    """Experiment with smooth functions and derivatives"""
    print("=" * 60)
    print("EXPERIMENT 1: Smooth Functions")
    print("=" * 60)
    
    # Create some smooth functions
    f = SmoothFunction("sin(x)")
    g = SmoothFunction("x**2")
    
    print(f"f(x) = {f.expr}")
    print(f"f'(x) = {f.derivative(1)}")
    print(f"f''(x) = {f.derivative(2)}")
    print()
    
    print(f"g(x) = {g.expr}")
    print(f"g'(x) = {g.derivative(1)}")
    
    # Compose them
    h = f.compose(g)
    print(f"\n(f∘g)(x) = {h.expr}")
    print(f"(f∘g)'(x) = {h.derivative(1)}")
    
    # Verify chain rule
    checker = CoherenceChecker()
    if checker.check_smooth_composition(f, g):
        print("✓ Chain rule verified!")
    else:
        print("✗ Chain rule violation!")
    
    # Taylor series
    print(f"\nTaylor series of sin(x) around 0 (order 5): {f.taylor_series(0, 5)}")

def experiment_2_paths():
    """Experiment with paths and smoothness"""
    print("\n" + "=" * 60)
    print("EXPERIMENT 2: Paths and Smoothness")
    print("=" * 60)
    
    # Create different types of paths
    linear_path = Path(0, 1)
    smooth_path = SmoothPath(0, 1)  # Default cubic
    custom_smooth = SmoothPath(0, 1, "t**2 * (3 - 2*t)")
    
    print("Linear path at t=0.5:", linear_path(0.5))
    print("Smooth path at t=0.5:", smooth_path(0.5))
    print("Custom smooth at t=0.5:", custom_smooth(0.5))
    
    # Check smoothness
    checker = CoherenceChecker()
    for name, path in [("Linear", linear_path), 
                       ("Default smooth", smooth_path),
                       ("Custom smooth", custom_smooth)]:
        is_smooth, msg = checker.check_path_smoothness(path)
        print(f"{name}: {msg}")
    
    # Derivatives of smooth path
    print(f"\nSmooth path first derivative at t=0.5: {smooth_path.derivative_at(0.5, 1)}")
    print(f"Smooth path second derivative at t=0.5: {smooth_path.derivative_at(0.5, 2)}")

def experiment_3_composition():
    """Experiment with path composition"""
    print("\n" + "=" * 60)
    print("EXPERIMENT 3: Path Composition")
    print("=" * 60)
    
    # Create two connecting paths
    path1 = SmoothPath(0, 2, "2*t**3")
    path2 = SmoothPath(2, 3, "2 + t**2")
    
    # Compose them
    composed = path1.compose(path2)
    
    print(f"Path 1: 0 → 2")
    print(f"Path 2: 2 → 3")
    print(f"Composed: 0 → 3")
    print(f"Composed path at t=0.25: {composed(0.25)}")
    print(f"Composed path at t=0.75: {composed(0.75)}")
    
    # Verify endpoints
    assert abs(composed(0) - 0) < 1e-10
    assert abs(composed(1) - 3) < 1e-10
    print("✓ Composition preserves endpoints!")

def your_exercise():
    """
    YOUR EXERCISE: Implement a smooth homotopy between two paths
    
    Challenge: Create two different smooth paths from 0 to 1,
    then create a smooth deformation from one to the other.
    """
    print("\n" + "=" * 60)
    print("YOUR EXERCISE: Smooth Homotopy")
    print("=" * 60)
    print("TODO: Implement a smooth deformation between two paths")
    
    # Hints:
    # 1. Create two SmoothPath objects with same endpoints
    # 2. Create a function H(s, t) where:
    #    - H(0, t) = first path
    #    - H(1, t) = second path
    #    - H(s, 0) = 0 for all s
    #    - H(s, 1) = 1 for all s
    # 3. Verify H is smooth in both parameters
    
    # Your code here:
    path_a = SmoothPath(0, 1, "t")  # Linear
    path_b = SmoothPath(0, 1, "t**2 * (3 - 2*t)")  # Cubic
    
    def homotopy(s, t):
        """Smooth deformation from path_a to path_b"""
        # Linear interpolation (simplest homotopy)
        return (1 - s) * path_a(t) + s * path_b(t)
    
    print(f"At s=0 (path_a), t=0.5: {homotopy(0, 0.5)}")
    print(f"At s=1 (path_b), t=0.5: {homotopy(1, 0.5)}")
    print(f"At s=0.5 (midway), t=0.5: {homotopy(0.5, 0.5)}")
    
    # Advanced: Make the homotopy itself smooth in s
    # This is where SCTT becomes powerful!

# ============================================================================
# MAIN PLAYGROUND
# ============================================================================

if __name__ == "__main__":
    print("""
    ╔════════════════════════════════════════════════════════╗
    ║     SCTT PLAYGROUND - Build Your Intuition!           ║
    ╠════════════════════════════════════════════════════════╣
    ║  This is your laboratory for understanding how         ║
    ║  smooth and cubical structures can work together.      ║
    ╚════════════════════════════════════════════════════════╝
    """)
    
    # Run experiments
    experiment_1_smooth_functions()
    experiment_2_paths()
    experiment_3_composition()
    your_exercise()
    
    print("\n" + "=" * 60)
    print("NEXT STEPS:")
    print("1. Modify the smooth functions and see what happens")
    print("2. Create your own path types")
    print("3. Implement the homotopy exercise")
    print("4. Try to break the coherence - find edge cases!")
    print("5. Imagine: What if ALL of mathematics worked this way?")
    print("=" * 60)