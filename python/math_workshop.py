#!/usr/bin/env python3
"""
SCTT Math Workshop: Let's work through the mathematics together!
Interactive lessons where we build understanding step by step.
"""

import numpy as np
import sympy as sp
import matplotlib.pyplot as plt
from typing import Callable, List, Tuple
import time

# Make output pretty
sp.init_printing(use_unicode=True)

def pause():
    """Pause for user to read"""
    input("\n[Press Enter to continue...]\n")

def section(title: str):
    """Print a section header"""
    print("\n" + "="*70)
    print(f"  {title}")
    print("="*70 + "\n")

# ============================================================================
# WORKSHOP 1: Understanding Smoothness
# ============================================================================

def workshop_1_smoothness():
    section("Workshop 1: What Does 'Smooth' Really Mean?")
    
    print("Let's explore smoothness by looking at different functions.\n")
    
    # Define symbolic variable
    x = sp.Symbol('x', real=True)
    
    # Example 1: Polynomial (infinitely smooth)
    print("📌 Example 1: Polynomial f(x) = x³ - 2x")
    f1 = x**3 - 2*x
    print(f"   f(x) = {f1}")
    
    print("\n   Let's compute derivatives:")
    for n in range(5):
        deriv = sp.diff(f1, x, n)
        print(f"   f^({n})(x) = {deriv}")
    
    print("\n   💡 Notice: Eventually becomes 0, but all derivatives exist!")
    print("   This is C^∞ (smooth)!")
    
    pause()
    
    # Example 2: Exponential (all derivatives equal itself)
    print("📌 Example 2: Exponential f(x) = e^x")
    f2 = sp.exp(x)
    print(f"   f(x) = {f2}")
    
    print("\n   Computing derivatives:")
    for n in range(5):
        deriv = sp.diff(f2, x, n)
        print(f"   f^({n})(x) = {deriv}")
    
    print("\n   💡 Amazing property: f^(n)(x) = e^x for all n!")
    print("   This is why e^x is special in calculus.")
    
    pause()
    
    # Example 3: Absolute value (NOT smooth)
    print("📌 Example 3: Absolute value f(x) = |x|")
    print("   f(x) = |x|")
    
    print("\n   What happens at x = 0?")
    print("   For x > 0: f'(x) = 1")
    print("   For x < 0: f'(x) = -1")
    print("   At x = 0: f'(0) = ???")
    
    print("\n   ❌ Derivative doesn't exist at x = 0!")
    print("   This is NOT smooth (not even C^1).")
    
    pause()
    
    # Visualize the difference
    print("Let's visualize these functions and their derivatives...\n")
    
    x_vals = np.linspace(-2, 2, 1000)
    
    fig, axes = plt.subplots(2, 3, figsize=(15, 8))
    
    # Polynomial
    y1 = x_vals**3 - 2*x_vals
    y1_prime = 3*x_vals**2 - 2
    axes[0, 0].plot(x_vals, y1, 'b-', linewidth=2)
    axes[0, 0].set_title("x³ - 2x (Smooth)")
    axes[0, 0].grid(True, alpha=0.3)
    axes[1, 0].plot(x_vals, y1_prime, 'b-', linewidth=2)
    axes[1, 0].set_title("Derivative: 3x² - 2")
    axes[1, 0].grid(True, alpha=0.3)
    
    # Exponential
    y2 = np.exp(x_vals)
    y2_prime = np.exp(x_vals)
    axes[0, 1].plot(x_vals, y2, 'g-', linewidth=2)
    axes[0, 1].set_title("e^x (Smooth)")
    axes[0, 1].grid(True, alpha=0.3)
    axes[1, 1].plot(x_vals, y2_prime, 'g-', linewidth=2)
    axes[1, 1].set_title("Derivative: e^x")
    axes[1, 1].grid(True, alpha=0.3)
    
    # Absolute value
    y3 = np.abs(x_vals)
    y3_prime = np.where(x_vals > 0, 1, -1)
    y3_prime[np.abs(x_vals) < 0.01] = np.nan  # Undefined at 0
    axes[0, 2].plot(x_vals, y3, 'r-', linewidth=2)
    axes[0, 2].set_title("|x| (NOT Smooth)")
    axes[0, 2].grid(True, alpha=0.3)
    axes[1, 2].plot(x_vals, y3_prime, 'r-', linewidth=2)
    axes[1, 2].set_title("Derivative: Discontinuous!")
    axes[1, 2].grid(True, alpha=0.3)
    
    plt.suptitle("Smooth vs Non-Smooth Functions", fontsize=16)
    plt.tight_layout()
    plt.show()
    
    print("\n🎯 Key Insight: Smooth means ALL derivatives exist and are continuous!")
    print("   In SCTT, we work exclusively with smooth functions.")

# ============================================================================
# WORKSHOP 2: The Chain Rule and Composition
# ============================================================================

def workshop_2_chain_rule():
    section("Workshop 2: The Chain Rule - Why Composition Preserves Smoothness")
    
    print("If f and g are smooth, is f∘g smooth? Let's prove it!\n")
    
    x = sp.Symbol('x', real=True)
    
    # Define two smooth functions
    print("📌 Let's compose two smooth functions:")
    g = x**2 + 1
    f = sp.sin(x)
    
    print(f"   g(x) = {g}")
    print(f"   f(x) = {f}")
    print(f"   (f∘g)(x) = f(g(x)) = sin(x² + 1)")
    
    pause()
    
    # Compute composition
    fog = f.subs(x, g)
    print(f"\n📌 The composition: {fog}")
    
    # Apply chain rule
    print("\n📌 Chain Rule: (f∘g)'(x) = f'(g(x)) · g'(x)")
    
    g_prime = sp.diff(g, x)
    f_prime = sp.diff(f, x)
    
    print(f"\n   g'(x) = {g_prime}")
    print(f"   f'(x) = {f_prime}")
    
    # Chain rule result
    f_prime_at_g = f_prime.subs(x, g)
    chain_result = f_prime_at_g * g_prime
    
    print(f"\n   f'(g(x)) = {f_prime_at_g}")
    print(f"   f'(g(x)) · g'(x) = {sp.simplify(chain_result)}")
    
    # Verify by direct differentiation
    direct = sp.diff(fog, x)
    print(f"\n   Direct differentiation: {sp.simplify(direct)}")
    
    print("\n   ✅ They match! Chain rule works!")
    
    pause()
    
    # Higher derivatives
    print("\n📌 What about higher derivatives?")
    print("   We need Faà di Bruno's formula (generalized chain rule):")
    
    print("\n   Second derivative:")
    fog_2 = sp.diff(fog, x, 2)
    print(f"   (f∘g)''(x) = {sp.simplify(fog_2)}")
    
    print("\n   This involves:")
    print("   - f''(g(x)) · (g'(x))²")
    print("   - f'(g(x)) · g''(x)")
    
    print("\n🎯 Key Insight: If f and g have all derivatives,")
    print("   then f∘g has all derivatives too!")
    print("   Composition preserves smoothness! ✨")

# ============================================================================
# WORKSHOP 3: Paths and Homotopy
# ============================================================================

def workshop_3_paths():
    section("Workshop 3: Paths - Continuous Deformations")
    
    print("A path is a continuous function from [0,1] to a space.\n")
    
    # Define parameter
    t = sp.Symbol('t', real=True)
    
    print("📌 Example: Different paths from 0 to 1")
    
    # Linear path
    path1 = t
    print(f"\n   Path 1 (linear): p₁(t) = {path1}")
    print(f"   p₁(0) = {path1.subs(t, 0)}, p₁(1) = {path1.subs(t, 1)}")
    
    # Quadratic path
    path2 = t**2
    print(f"\n   Path 2 (quadratic): p₂(t) = {path2}")
    print(f"   p₂(0) = {path2.subs(t, 0)}, p₂(1) = {path2.subs(t, 1)}")
    
    # Smooth S-curve
    path3 = 3*t**2 - 2*t**3
    print(f"\n   Path 3 (smooth): p₃(t) = {path3}")
    print(f"   p₃(0) = {path3.subs(t, 0)}, p₃(1) = {path3.subs(t, 1)}")
    
    pause()
    
    # Check velocities
    print("\n📌 Let's check the velocities (derivatives):")
    
    v1 = sp.diff(path1, t)
    v2 = sp.diff(path2, t)
    v3 = sp.diff(path3, t)
    
    print(f"\n   v₁(t) = p₁'(t) = {v1}")
    print(f"   v₂(t) = p₂'(t) = {v2}")
    print(f"   v₃(t) = p₃'(t) = {v3}")
    
    print("\n   At endpoints:")
    for i, (p, v) in enumerate([(path1, v1), (path2, v2), (path3, v3)], 1):
        v0 = v.subs(t, 0)
        v1_val = v.subs(t, 1)
        print(f"   Path {i}: v(0) = {v0}, v(1) = {v1_val}")
    
    print("\n   💡 Notice: Path 3 has zero velocity at endpoints!")
    print("   This makes it 'smoother' for composition.")
    
    pause()
    
    # Visualize paths
    print("\nLet's visualize these paths and their velocities...\n")
    
    t_vals = np.linspace(0, 1, 100)
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))
    
    # Paths
    p1_vals = t_vals
    p2_vals = t_vals**2
    p3_vals = 3*t_vals**2 - 2*t_vals**3
    
    ax1.plot(t_vals, p1_vals, 'b-', label='Linear: t', linewidth=2)
    ax1.plot(t_vals, p2_vals, 'g-', label='Quadratic: t²', linewidth=2)
    ax1.plot(t_vals, p3_vals, 'r-', label='Smooth: 3t² - 2t³', linewidth=2)
    ax1.set_xlabel('Parameter t')
    ax1.set_ylabel('Position')
    ax1.set_title('Different Paths from 0 to 1')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # Velocities
    v1_vals = np.ones_like(t_vals)
    v2_vals = 2*t_vals
    v3_vals = 6*t_vals - 6*t_vals**2
    
    ax2.plot(t_vals, v1_vals, 'b-', label='Linear velocity', linewidth=2)
    ax2.plot(t_vals, v2_vals, 'g-', label='Quadratic velocity', linewidth=2)
    ax2.plot(t_vals, v3_vals, 'r-', label='Smooth velocity', linewidth=2)
    ax2.set_xlabel('Parameter t')
    ax2.set_ylabel('Velocity')
    ax2.set_title('Path Velocities')
    ax2.legend()
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.show()
    
    print("\n🎯 Key Insight: Smooth paths have smooth velocity profiles!")

# ============================================================================
# WORKSHOP 4: The Interval and Cubical Operations
# ============================================================================

def workshop_4_interval():
    section("Workshop 4: The Interval I = [0,1] and De Morgan Algebra")
    
    print("The interval I is the foundation of cubical type theory.\n")
    
    print("📌 Basic operations on I:")
    print("   - 0, 1: endpoints")
    print("   - i ∧ j: meet (minimum)")
    print("   - i ∨ j: join (maximum)")  
    print("   - ¬i = 1-i: negation")
    
    pause()
    
    # Demonstrate operations
    print("\n📌 Let's compute some examples:")
    
    examples = [
        (0.3, 0.7, "∧", min),
        (0.3, 0.7, "∨", max),
        (0.3, None, "¬", lambda x, _: 1-x),
    ]
    
    for ex in examples:
        if ex[1] is None:
            result = ex[3](ex[0], None)
            print(f"   ¬{ex[0]} = {result:.1f}")
        else:
            result = ex[3](ex[0], ex[1])
            print(f"   {ex[0]} {ex[2]} {ex[1]} = {result:.1f}")
    
    pause()
    
    print("\n📌 De Morgan Laws (fundamental properties):")
    
    # Verify De Morgan laws numerically
    i, j = 0.3, 0.7
    
    # First law: ¬(i ∧ j) = ¬i ∨ ¬j
    left1 = 1 - min(i, j)
    right1 = max(1-i, 1-j)
    print(f"\n   ¬(i ∧ j) = ¬{min(i,j):.1f} = {left1:.1f}")
    print(f"   ¬i ∨ ¬j = {1-i:.1f} ∨ {1-j:.1f} = {right1:.1f}")
    print(f"   Equal? {abs(left1 - right1) < 0.001} ✓")
    
    # Second law: ¬(i ∨ j) = ¬i ∧ ¬j
    left2 = 1 - max(i, j)
    right2 = min(1-i, 1-j)
    print(f"\n   ¬(i ∨ j) = ¬{max(i,j):.1f} = {left2:.1f}")
    print(f"   ¬i ∧ ¬j = {1-i:.1f} ∧ {1-j:.1f} = {right2:.1f}")
    print(f"   Equal? {abs(left2 - right2) < 0.001} ✓")
    
    pause()
    
    print("\n📌 Building higher dimensions from I:")
    print("\n   I¹ = I = line segment [0,1]")
    print("   I² = I × I = square [0,1]²")
    print("   I³ = I × I × I = cube [0,1]³")
    print("   ...")
    
    # Visualize square from intervals
    fig = plt.figure(figsize=(10, 5))
    
    # 1D interval
    ax1 = fig.add_subplot(121)
    ax1.plot([0, 1], [0, 0], 'b-', linewidth=3)
    ax1.plot(0, 0, 'ro', markersize=10, label='0')
    ax1.plot(1, 0, 'go', markersize=10, label='1')
    ax1.set_xlim(-0.2, 1.2)
    ax1.set_ylim(-0.5, 0.5)
    ax1.set_title('I¹: The Interval')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # 2D square
    ax2 = fig.add_subplot(122)
    square = plt.Rectangle((0, 0), 1, 1, fill=False, edgecolor='b', linewidth=2)
    ax2.add_patch(square)
    ax2.plot([0, 1, 1, 0], [0, 0, 1, 1], 'ro', markersize=10)
    ax2.set_xlim(-0.2, 1.2)
    ax2.set_ylim(-0.2, 1.2)
    ax2.set_aspect('equal')
    ax2.set_title('I²: The Square')
    ax2.set_xlabel('i')
    ax2.set_ylabel('j')
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.show()
    
    print("\n🎯 Key Insight: Everything in cubical type theory is built from I!")
    print("   Paths, squares, cubes, ... all come from the interval.")

# ============================================================================
# WORKSHOP 5: The Coherence Challenge
# ============================================================================

def workshop_5_coherence():
    section("Workshop 5: The Coherence Challenge - Making Smooth + Cubical Work")
    
    print("This is the heart of SCTT: how do smooth and cubical interact?\n")
    
    t = sp.Symbol('t', real=True)
    
    print("📌 The Problem: Path composition might break smoothness")
    
    # Two smooth paths
    path1 = t**2  # From 0 to 1
    path2 = 1 + t*(2-1)  # From 1 to 2 (linear for simplicity)
    
    print(f"\n   Path 1: p₁(t) = {path1} from 0 to 1")
    print(f"   Path 2: p₂(t) = {path2} from 1 to 2")
    
    # Velocities
    v1 = sp.diff(path1, t)
    v2 = sp.diff(path2, t)
    
    print(f"\n   Velocity 1: v₁(t) = {v1}")
    print(f"   Velocity 2: v₂(t) = {v2}")
    
    # Check at connection point
    v1_end = v1.subs(t, 1)
    v2_start = v2.subs(t, 0)
    
    print(f"\n   At connection (t=1 for p₁, t=0 for p₂):")
    print(f"   v₁(1) = {v1_end}")
    print(f"   v₂(0) = {v2_start}")
    
    if v1_end != v2_start:
        print(f"\n   ❌ Velocities don't match! {v1_end} ≠ {v2_start}")
        print("   The composed path has a 'kink' - not smooth!")
    
    pause()
    
    print("\n📌 The Solution: Coherence conditions")
    print("\n   Option 1: Require matching derivatives at boundaries")
    print("   Option 2: Use reparametrization to smooth the connection")
    print("   Option 3: Define composition differently for smooth paths")
    
    pause()
    
    print("\n📌 Let's try a smooth composition:")
    
    # Smooth transition using cubic Hermite spline
    def smooth_compose(t):
        if t <= 0.5:
            # First half: traverse path1 with smooth slowdown
            s = 2*t  # Reparametrize to [0,1]
            # Apply smoothing: 3s² - 2s³
            s_smooth = 3*s**2 - 2*s**3
            return s_smooth  # Returns value in [0,1]
        else:
            # Second half: traverse path2 with smooth speedup
            s = 2*(t - 0.5)  # Reparametrize to [0,1]
            s_smooth = 3*s**2 - 2*s**3
            return 1 + s_smooth  # Returns value in [1,2]
    
    # Visualize
    t_vals = np.linspace(0, 1, 200)
    
    # Naive composition (with kink)
    naive = []
    for t_val in t_vals:
        if t_val <= 0.5:
            naive.append((2*t_val)**2)  # path1 sped up
        else:
            naive.append(1 + (2*(t_val - 0.5)))  # path2
    
    # Smooth composition
    smooth = [smooth_compose(t_val) for t_val in t_vals]
    
    # Velocities (numerical derivative)
    dt = t_vals[1] - t_vals[0]
    naive_vel = np.gradient(naive, dt)
    smooth_vel = np.gradient(smooth, dt)
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))
    
    ax1.plot(t_vals, naive, 'r-', label='Naive composition', linewidth=2)
    ax1.plot(t_vals, smooth, 'g-', label='Smooth composition', linewidth=2)
    ax1.set_xlabel('Parameter t')
    ax1.set_ylabel('Position')
    ax1.set_title('Path Composition: Naive vs Smooth')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    ax2.plot(t_vals, naive_vel, 'r-', label='Naive velocity', linewidth=2)
    ax2.plot(t_vals, smooth_vel, 'g-', label='Smooth velocity', linewidth=2)
    ax2.set_xlabel('Parameter t')
    ax2.set_ylabel('Velocity')
    ax2.set_title('Velocity Profiles (See the Kink?)')
    ax2.legend()
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.show()
    
    print("\n🎯 Key Insight: SCTT must ensure ALL operations preserve smoothness!")
    print("   This is the coherence challenge we need to solve.")

# ============================================================================
# WORKSHOP 6: Taylor Series and Infinite Information
# ============================================================================

def workshop_6_taylor():
    section("Workshop 6: Taylor Series - Encoding Infinite Information")
    
    print("Smooth functions contain infinite information (all derivatives).\n")
    print("How do we represent this computationally? Taylor series!\n")
    
    x = sp.Symbol('x', real=True)
    
    # Example function
    f = sp.sin(x)
    print(f"📌 Let's expand sin(x) around x = 0:")
    
    # Compute Taylor series
    print("\n   Taylor series: f(x) = Σ(n=0 to ∞) f^(n)(0)/n! · x^n")
    
    print("\n   Computing derivatives at 0:")
    derivatives = []
    for n in range(8):
        deriv = sp.diff(f, x, n)
        deriv_at_0 = deriv.subs(x, 0)
        derivatives.append(deriv_at_0)
        if deriv_at_0 != 0:
            print(f"   f^({n})(0) = {deriv_at_0}")
    
    pause()
    
    # Build Taylor series
    print("\n📌 Building the series:")
    series = 0
    factorials = [1, 1, 2, 6, 24, 120, 720, 5040]
    
    for n in range(8):
        if derivatives[n] != 0:
            term = derivatives[n] * x**n / factorials[n]
            series += term
            print(f"   Term {n}: {derivatives[n]}/{factorials[n]} · x^{n}")
    
    print(f"\n   sin(x) ≈ {series}")
    
    pause()
    
    # Visualize convergence
    print("\nLet's see how Taylor series converges to sin(x)...\n")
    
    x_vals = np.linspace(-np.pi, np.pi, 1000)
    true_sin = np.sin(x_vals)
    
    fig, axes = plt.subplots(2, 2, figsize=(12, 10))
    
    # Different order approximations
    orders = [1, 3, 5, 7]
    
    for idx, order in enumerate(orders):
        ax = axes[idx // 2, idx % 2]
        
        # Compute Taylor approximation
        approx = np.zeros_like(x_vals)
        for n in range(order + 1):
            deriv_n = sp.diff(f, x, n).subs(x, 0)
            if deriv_n != 0:
                approx += float(deriv_n) * x_vals**n / np.math.factorial(n)
        
        ax.plot(x_vals, true_sin, 'b-', label='sin(x)', linewidth=2)
        ax.plot(x_vals, approx, 'r--', label=f'Taylor order {order}', linewidth=2)
        ax.set_xlabel('x')
        ax.set_ylabel('y')
        ax.set_title(f'Taylor Series: Order {order}')
        ax.legend()
        ax.grid(True, alpha=0.3)
        ax.set_ylim(-2, 2)
    
    plt.suptitle('Taylor Series Convergence for sin(x)', fontsize=16)
    plt.tight_layout()
    plt.show()
    
    print("\n🎯 Key Insight: Smooth functions can be represented by their")
    print("   Taylor series - infinite information in a finite formula!")

# ============================================================================
# MAIN MENU
# ============================================================================

def main():
    print("""
    ╔════════════════════════════════════════════════════════════════╗
    ║         SCTT Math Workshop - Let's Learn Together!            ║
    ╚════════════════════════════════════════════════════════════════╝
    """)
    
    workshops = [
        ("Understanding Smoothness", workshop_1_smoothness),
        ("The Chain Rule and Composition", workshop_2_chain_rule),
        ("Paths and Homotopy", workshop_3_paths),
        ("The Interval and Cubical Operations", workshop_4_interval),
        ("The Coherence Challenge", workshop_5_coherence),
        ("Taylor Series and Infinite Information", workshop_6_taylor),
    ]
    
    while True:
        print("\n" + "="*70)
        print("  Available Workshops:")
        print("="*70)
        
        for i, (name, _) in enumerate(workshops, 1):
            print(f"  {i}. {name}")
        
        print(f"  {len(workshops)+1}. Run all workshops in sequence")
        print(f"  0. Exit")
        
        try:
            choice = int(input("\nSelect a workshop (0-7): "))
            
            if choice == 0:
                print("\nThanks for learning! Keep exploring SCTT! 🚀")
                break
            elif 1 <= choice <= len(workshops):
                workshops[choice-1][1]()
                pause()
            elif choice == len(workshops) + 1:
                print("\nRunning all workshops in sequence...\n")
                for name, workshop in workshops:
                    workshop()
                    pause()
            else:
                print("Invalid choice. Please try again.")
        except (ValueError, KeyboardInterrupt):
            print("\nExiting... Thanks for learning!")
            break

if __name__ == "__main__":
    main()