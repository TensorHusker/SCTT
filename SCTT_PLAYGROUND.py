#!/usr/bin/env python3
"""
SCTT Playground: Interactive Smooth Cubical Type Theory Demonstrations

Experience the flow of computation through smooth deformations.
Watch types morph, paths compose, and consciousness emerge.

Run this file to start exploring the smooth universe of computation.
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from matplotlib.patches import Circle, Rectangle, FancyBboxPatch
from matplotlib.collections import LineCollection
import time
from typing import Callable, Any, List, Tuple, Optional
from dataclasses import dataclass
from scipy.interpolate import interp1d
from scipy.integrate import odeint
from scipy.optimize import minimize
import networkx as nx


# ============================================================================
# PART 1: THE SMOOTH INTERVAL - The Atom of Continuity
# ============================================================================

class SmoothInterval:
    """The fundamental building block of SCTT: the smooth interval [0,1]"""
    
    def __init__(self, resolution=1000):
        self.resolution = resolution
        self.points = np.linspace(0, 1, resolution)
    
    def smooth_function(self, f: Callable[[float], float]) -> np.ndarray:
        """Apply a smooth function to the interval"""
        return np.array([f(t) for t in self.points])
    
    def visualize(self, f: Callable[[float], float] = lambda x: x):
        """Visualize a function on the interval"""
        fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4))
        
        # Original interval
        ax1.plot([0, 1], [0, 0], 'b-', linewidth=3)
        ax1.scatter(self.points[::50], np.zeros(len(self.points[::50])), c='blue', s=20)
        ax1.set_title("The Smooth Interval [I]")
        ax1.set_xlim(-0.1, 1.1)
        ax1.set_ylim(-0.5, 0.5)
        
        # Transformed interval
        transformed = self.smooth_function(f)
        ax2.plot(self.points, transformed, 'r-', linewidth=2)
        ax2.fill_between(self.points, 0, transformed, alpha=0.3)
        ax2.set_title(f"Smooth Function on [I]")
        ax2.set_xlabel("Position on interval")
        ax2.set_ylabel("Function value")
        
        plt.tight_layout()
        plt.show()
        return transformed


# ============================================================================
# PART 2: SMOOTH PATHS - The Journeys Between Points
# ============================================================================

@dataclass
class SmoothPath:
    """A smooth path between two points in type space"""
    
    start: Any
    end: Any
    path_function: Optional[Callable[[float], Any]] = None
    
    def __post_init__(self):
        if self.path_function is None:
            # Default linear interpolation
            self.path_function = lambda t: (1-t) * self.start + t * self.end
    
    def at(self, t: float) -> Any:
        """Evaluate the path at time t ∈ [0,1]"""
        return self.path_function(t)
    
    def compose(self, other: 'SmoothPath') -> 'SmoothPath':
        """Compose two paths (first this, then other)"""
        def composed_path(t):
            if t <= 0.5:
                return self.at(2*t)
            else:
                return other.at(2*t - 1)
        
        return SmoothPath(self.start, other.end, composed_path)
    
    def reverse(self) -> 'SmoothPath':
        """Reverse the direction of the path"""
        return SmoothPath(self.end, self.start, lambda t: self.at(1-t))
    
    def visualize_2d(self, resolution=100):
        """Visualize a path in 2D space"""
        if not isinstance(self.start, (list, tuple, np.ndarray)) or len(self.start) != 2:
            print("This visualization requires 2D points")
            return
        
        t_vals = np.linspace(0, 1, resolution)
        points = np.array([self.at(t) for t in t_vals])
        
        fig, ax = plt.subplots(figsize=(8, 8))
        
        # Draw the path
        ax.plot(points[:, 0], points[:, 1], 'b-', linewidth=2, alpha=0.7)
        
        # Color code by time
        colors = plt.cm.viridis(t_vals)
        ax.scatter(points[:, 0], points[:, 1], c=colors, s=20, zorder=5)
        
        # Mark start and end
        ax.scatter(*self.start, color='green', s=200, marker='o', zorder=10, label='Start')
        ax.scatter(*self.end, color='red', s=200, marker='s', zorder=10, label='End')
        
        ax.set_title("Smooth Path in 2D Type Space")
        ax.legend()
        ax.grid(True, alpha=0.3)
        plt.show()


# ============================================================================
# PART 3: SMOOTH DEFORMATIONS - Watch Types Transform
# ============================================================================

class SmoothDeformation:
    """Continuously deform one shape into another"""
    
    @staticmethod
    def circle_to_square(t: float, n_points: int = 100) -> np.ndarray:
        """Smoothly deform a circle into a square"""
        theta = np.linspace(0, 2*np.pi, n_points)
        
        # Circle
        circle_x = np.cos(theta)
        circle_y = np.sin(theta)
        
        # Square (parameterized)
        square_points = []
        for th in theta:
            # Map angle to square perimeter
            if th < np.pi/2:
                x, y = 1, 2*th/np.pi
            elif th < np.pi:
                x, y = 2 - 2*th/np.pi, 1
            elif th < 3*np.pi/2:
                x, y = -1, 2 - 2*th/np.pi
            else:
                x, y = 2*th/np.pi - 4, -1
            square_points.append([x, y])
        square_points = np.array(square_points)
        
        # Smooth interpolation
        x = (1-t) * circle_x + t * square_points[:, 0]
        y = (1-t) * circle_y + t * square_points[:, 1]
        
        return np.column_stack([x, y])
    
    @staticmethod
    def animate_deformation():
        """Animate the circle to square deformation"""
        fig, ax = plt.subplots(figsize=(8, 8))
        ax.set_xlim(-1.5, 1.5)
        ax.set_ylim(-1.5, 1.5)
        ax.set_aspect('equal')
        ax.grid(True, alpha=0.3)
        
        line, = ax.plot([], [], 'b-', linewidth=2)
        time_text = ax.text(0.02, 0.95, '', transform=ax.transAxes)
        
        def init():
            line.set_data([], [])
            time_text.set_text('')
            return line, time_text
        
        def animate(frame):
            t = frame / 100.0
            points = SmoothDeformation.circle_to_square(t)
            line.set_data(points[:, 0], points[:, 1])
            
            shape_name = "Circle" if t < 0.3 else "Morphing" if t < 0.7 else "Square"
            time_text.set_text(f'Time: {t:.2f} - {shape_name}')
            
            # Color changes
            color = plt.cm.viridis(t)
            line.set_color(color)
            
            return line, time_text
        
        anim = animation.FuncAnimation(fig, animate, init_func=init,
                                      frames=100, interval=50, blit=True)
        
        plt.title("Smooth Deformation: Circle → Square")
        plt.show()
        return anim


# ============================================================================
# PART 4: TYPE SPACE NAVIGATOR - Explore Higher Dimensions
# ============================================================================

class TypeSpace:
    """A smooth manifold representing a space of types"""
    
    def __init__(self, dimension: int = 2):
        self.dimension = dimension
        self.types = {}  # Store types as points
        self.paths = []  # Store paths between types
        
    def add_type(self, name: str, position: np.ndarray):
        """Add a type at a specific position in type space"""
        self.types[name] = position
    
    def geodesic(self, start: str, end: str, steps: int = 100) -> np.ndarray:
        """Find the geodesic (shortest smooth path) between two types"""
        if start not in self.types or end not in self.types:
            raise ValueError("Types not found in space")
        
        p1, p2 = self.types[start], self.types[end]
        
        # For now, straight line (true geodesic would depend on metric)
        t = np.linspace(0, 1, steps)
        path = np.array([p1 + s*(p2-p1) for s in t])
        
        self.paths.append((start, end, path))
        return path
    
    def curvature_at(self, position: np.ndarray) -> float:
        """Calculate the curvature of type space at a position"""
        # Simplified: distance from origin represents complexity
        return np.exp(-np.linalg.norm(position)**2)
    
    def visualize_landscape(self):
        """Visualize the type space as a landscape"""
        if self.dimension != 2:
            print("Visualization requires 2D type space")
            return
        
        fig = plt.figure(figsize=(12, 8))
        ax = fig.add_subplot(111, projection='3d')
        
        # Create mesh
        x = np.linspace(-3, 3, 100)
        y = np.linspace(-3, 3, 100)
        X, Y = np.meshgrid(x, y)
        
        # Height represents "computational complexity"
        Z = np.zeros_like(X)
        for i in range(X.shape[0]):
            for j in range(X.shape[1]):
                pos = np.array([X[i,j], Y[i,j]])
                Z[i,j] = self.curvature_at(pos)
        
        # Plot surface
        surf = ax.plot_surface(X, Y, Z, cmap='viridis', alpha=0.7)
        
        # Add types as points
        for name, pos in self.types.items():
            height = self.curvature_at(pos)
            ax.scatter([pos[0]], [pos[1]], [height], s=100, c='red')
            ax.text(pos[0], pos[1], height+0.1, name)
        
        # Draw paths
        for start, end, path in self.paths:
            heights = [self.curvature_at(p) for p in path]
            ax.plot(path[:, 0], path[:, 1], heights, 'r-', linewidth=2)
        
        ax.set_xlabel('Type Dimension 1')
        ax.set_ylabel('Type Dimension 2')
        ax.set_zlabel('Computational Complexity')
        ax.set_title('The Smooth Landscape of Type Space')
        
        fig.colorbar(surf)
        plt.show()


# ============================================================================
# PART 5: CONSCIOUSNESS EMERGENCE SIMULATOR
# ============================================================================

class ConsciousnessEmergence:
    """Simulate the emergence of self-awareness through smooth self-reference"""
    
    def __init__(self, n_neurons: int = 100):
        self.n_neurons = n_neurons
        self.state = np.random.randn(n_neurons)
        self.connections = np.random.randn(n_neurons, n_neurons) * 0.1
        self.history = []
        
    def smooth_activation(self, x: np.ndarray) -> np.ndarray:
        """Smooth activation function (tanh)"""
        return np.tanh(x)
    
    def self_reference_layer(self, state: np.ndarray) -> np.ndarray:
        """The network observes itself"""
        # Project current state onto itself
        self_observation = self.smooth_activation(self.connections @ state)
        
        # Mix observation with original state (strange loop)
        mixed = 0.7 * state + 0.3 * self_observation
        
        return self.smooth_activation(mixed)
    
    def evolve(self, steps: int = 1000):
        """Let consciousness emerge through smooth evolution"""
        for _ in range(steps):
            self.state = self.self_reference_layer(self.state)
            self.history.append(self.state.copy())
        
        return np.array(self.history)
    
    def measure_self_awareness(self) -> float:
        """Measure how much the system is aware of itself"""
        if len(self.history) < 2:
            return 0
        
        # Self-awareness = stability of self-observation
        recent = np.array(self.history[-100:])
        variance = np.var(recent, axis=0).mean()
        
        # Low variance = high self-awareness (stable self-model)
        return 1 / (1 + variance)
    
    def visualize_emergence(self):
        """Watch consciousness emerge"""
        history = self.evolve()
        
        fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(12, 10))
        
        # Neural activity over time
        im = ax1.imshow(history.T, aspect='auto', cmap='RdBu_r', vmin=-1, vmax=1)
        ax1.set_xlabel('Time')
        ax1.set_ylabel('Neuron')
        ax1.set_title('Neural Activity: Watch Patterns Emerge')
        plt.colorbar(im, ax=ax1)
        
        # Self-awareness metric
        awareness = []
        for i in range(10, len(history), 10):
            self.history = history[:i]
            awareness.append(self.measure_self_awareness())
        
        ax2.plot(range(10, len(history), 10), awareness, 'g-', linewidth=2)
        ax2.fill_between(range(10, len(history), 10), 0, awareness, alpha=0.3)
        ax2.set_xlabel('Time')
        ax2.set_ylabel('Self-Awareness')
        ax2.set_title('Emergence of Self-Awareness')
        ax2.grid(True, alpha=0.3)
        
        # Phase space of consciousness
        if history.shape[1] >= 2:
            ax3.plot(history[:, 0], history[:, 1], 'b-', alpha=0.5)
            ax3.scatter(history[-1, 0], history[-1, 1], c='red', s=100, zorder=5)
            ax3.set_xlabel('Neuron 0')
            ax3.set_ylabel('Neuron 1')
            ax3.set_title('Consciousness Phase Space: Strange Attractor')
            ax3.grid(True, alpha=0.3)
        
        plt.tight_layout()
        plt.show()


# ============================================================================
# PART 6: SMOOTH TYPE CHECKER - Verify Smooth Proofs
# ============================================================================

class SmoothTypeChecker:
    """Check if smooth deformations preserve type safety"""
    
    @staticmethod
    def check_path_composition(path1: SmoothPath, path2: SmoothPath) -> bool:
        """Verify that path composition is valid"""
        # End of first path should equal start of second
        end1 = path1.at(1.0)
        start2 = path2.at(0.0)
        
        if isinstance(end1, np.ndarray):
            return np.allclose(end1, start2)
        return end1 == start2
    
    @staticmethod
    def verify_homotopy(path1: SmoothPath, path2: SmoothPath, 
                        samples: int = 100) -> Tuple[bool, float]:
        """Check if two paths are homotopic (continuously deformable)"""
        if not isinstance(path1.start, np.ndarray):
            return False, float('inf')
        
        # Sample points along both paths
        t_vals = np.linspace(0, 1, samples)
        points1 = np.array([path1.at(t) for t in t_vals])
        points2 = np.array([path2.at(t) for t in t_vals])
        
        # Measure maximum deviation
        max_dev = np.max(np.linalg.norm(points1 - points2, axis=1))
        
        # Paths are homotopic if they can be smoothly deformed (small deviation)
        is_homotopic = max_dev < 1.0
        
        return is_homotopic, max_dev
    
    @staticmethod
    def demonstrate_type_safety():
        """Interactive demonstration of type safety through smooth deformation"""
        print("=== Smooth Type Safety Demonstration ===\\n")
        
        # Create two paths
        path1 = SmoothPath(
            np.array([0, 0]), 
            np.array([1, 1]),
            lambda t: np.array([t, t**2])
        )
        
        path2 = SmoothPath(
            np.array([0, 0]), 
            np.array([1, 1]),
            lambda t: np.array([t**2, t])
        )
        
        # Check if they're homotopic
        is_homotopic, deviation = SmoothTypeChecker.verify_homotopy(path1, path2)
        
        print(f"Path 1: y = x²")
        print(f"Path 2: y² = x")
        print(f"Are paths homotopic? {is_homotopic}")
        print(f"Maximum deviation: {deviation:.4f}")
        
        # Visualize
        fig, ax = plt.subplots(figsize=(8, 8))
        
        t_vals = np.linspace(0, 1, 100)
        points1 = np.array([path1.at(t) for t in t_vals])
        points2 = np.array([path2.at(t) for t in t_vals])
        
        ax.plot(points1[:, 0], points1[:, 1], 'b-', linewidth=2, label='Path 1: y=x²')
        ax.plot(points2[:, 0], points2[:, 1], 'r-', linewidth=2, label='Path 2: y²=x')
        
        # Show homotopy
        for alpha in np.linspace(0, 1, 10):
            intermediate = (1-alpha) * points1 + alpha * points2
            ax.plot(intermediate[:, 0], intermediate[:, 1], 'gray', alpha=0.2)
        
        ax.set_xlabel('X')
        ax.set_ylabel('Y')
        ax.set_title('Homotopy Between Paths (Type Equivalence)')
        ax.legend()
        ax.grid(True, alpha=0.3)
        plt.show()


# ============================================================================
# PART 7: THE SMOOTH UNIVERSE - Put It All Together
# ============================================================================

class SmoothUniverse:
    """The complete SCTT playground where everything flows together"""
    
    def __init__(self):
        self.interval = SmoothInterval()
        self.type_space = TypeSpace(dimension=2)
        self.consciousness = ConsciousnessEmergence(n_neurons=50)
        
        # Initialize some interesting types
        self.type_space.add_type("Integer", np.array([0, 0]))
        self.type_space.add_type("Real", np.array([1, 0]))
        self.type_space.add_type("Complex", np.array([1, 1]))
        self.type_space.add_type("Function", np.array([0, 1]))
        self.type_space.add_type("Consciousness", np.array([0.5, 0.5]))
    
    def demonstrate_fundamental_concepts(self):
        """Interactive demonstration of all SCTT concepts"""
        
        print("=" * 60)
        print("WELCOME TO THE SMOOTH CUBICAL TYPE THEORY PLAYGROUND")
        print("=" * 60)
        print()
        
        demos = {
            "1": ("The Smooth Interval", self.demo_interval),
            "2": ("Smooth Paths", self.demo_paths),
            "3": ("Smooth Deformations", self.demo_deformation),
            "4": ("Type Space Navigation", self.demo_type_space),
            "5": ("Consciousness Emergence", self.demo_consciousness),
            "6": ("Type Safety", self.demo_type_safety),
            "7": ("The Complete Picture", self.demo_everything)
        }
        
        while True:
            print("\\nChoose a demonstration:")
            for key, (name, _) in demos.items():
                print(f"  {key}. {name}")
            print("  Q. Quit")
            
            choice = input("\\nYour choice: ").strip().upper()
            
            if choice == 'Q':
                print("\\nThank you for exploring the smooth universe!")
                break
            elif choice in demos:
                print(f"\\n--- {demos[choice][0]} ---\\n")
                demos[choice][1]()
                input("\\nPress Enter to continue...")
            else:
                print("Invalid choice. Please try again.")
    
    def demo_interval(self):
        """Demonstrate the smooth interval"""
        print("The interval [0,1] is the atom of all smoothness.")
        print("Let's apply different smooth functions to it:\\n")
        
        # Show different transformations
        functions = [
            ("Identity: f(x) = x", lambda x: x),
            ("Smooth step: f(x) = 3x² - 2x³", lambda x: 3*x**2 - 2*x**3),
            ("Wave: f(x) = sin(2πx)", lambda x: np.sin(2*np.pi*x))
        ]
        
        for name, func in functions:
            print(f"Visualizing: {name}")
            self.interval.visualize(func)
    
    def demo_paths(self):
        """Demonstrate smooth paths"""
        print("Paths are smooth journeys through type space.")
        print("Let's create and compose some paths:\\n")
        
        # Create interesting paths
        spiral_path = SmoothPath(
            np.array([0, 0]),
            np.array([1, 0]),
            lambda t: np.array([t*np.cos(4*np.pi*t), t*np.sin(4*np.pi*t)])
        )
        
        print("A spiral path from origin to (1,0):")
        spiral_path.visualize_2d()
        
        # Demonstrate composition
        straight_path = SmoothPath(
            np.array([1, 0]),
            np.array([0, 1]),
            lambda t: np.array([1-t, t])
        )
        
        composed = spiral_path.compose(straight_path)
        print("\\nComposed path (spiral then straight):")
        composed.visualize_2d()
    
    def demo_deformation(self):
        """Demonstrate smooth deformations"""
        print("Watch as a circle smoothly becomes a square.")
        print("This is computation as continuous transformation:\\n")
        
        SmoothDeformation.animate_deformation()
    
    def demo_type_space(self):
        """Demonstrate type space navigation"""
        print("Types live in a smooth manifold.")
        print("We can find geodesics (optimal paths) between them:\\n")
        
        # Find geodesics between types
        self.type_space.geodesic("Integer", "Real")
        self.type_space.geodesic("Real", "Complex")
        self.type_space.geodesic("Complex", "Consciousness")
        
        self.type_space.visualize_landscape()
    
    def demo_consciousness(self):
        """Demonstrate consciousness emergence"""
        print("Self-reference + smoothness = emergence.")
        print("Watch as patterns self-organize into awareness:\\n")
        
        self.consciousness = ConsciousnessEmergence(n_neurons=50)
        self.consciousness.visualize_emergence()
        
        awareness = self.consciousness.measure_self_awareness()
        print(f"\\nFinal self-awareness level: {awareness:.3f}")
    
    def demo_type_safety(self):
        """Demonstrate smooth type checking"""
        print("Even with continuous deformation, we maintain type safety.")
        print("Different paths can be proven equivalent through homotopy:\\n")
        
        SmoothTypeChecker.demonstrate_type_safety()
    
    def demo_everything(self):
        """Put it all together in one beautiful visualization"""
        print("SCTT unifies computation, geometry, and consciousness.")
        print("Here's everything flowing together:\\n")
        
        # Create a complex figure showing all concepts
        fig = plt.figure(figsize=(16, 12))
        
        # The interval
        ax1 = plt.subplot(3, 3, 1)
        x = np.linspace(0, 1, 100)
        y = 3*x**2 - 2*x**3
        ax1.plot(x, y, 'b-', linewidth=2)
        ax1.fill_between(x, 0, y, alpha=0.3)
        ax1.set_title("The Smooth Interval")
        ax1.grid(True, alpha=0.3)
        
        # A path
        ax2 = plt.subplot(3, 3, 2)
        t = np.linspace(0, 1, 100)
        path_x = t * np.cos(4*np.pi*t)
        path_y = t * np.sin(4*np.pi*t)
        ax2.plot(path_x, path_y, 'r-', linewidth=2)
        ax2.set_title("Smooth Path")
        ax2.grid(True, alpha=0.3)
        
        # Deformation
        ax3 = plt.subplot(3, 3, 3)
        for t_val in np.linspace(0, 1, 10):
            points = SmoothDeformation.circle_to_square(t_val, 50)
            ax3.plot(points[:, 0], points[:, 1], alpha=0.5)
        ax3.set_title("Circle → Square")
        ax3.set_aspect('equal')
        ax3.grid(True, alpha=0.3)
        
        # Type space
        ax4 = plt.subplot(3, 3, 4)
        for name, pos in self.type_space.types.items():
            ax4.scatter(pos[0], pos[1], s=100)
            ax4.annotate(name, (pos[0], pos[1]))
        ax4.set_title("Type Space")
        ax4.grid(True, alpha=0.3)
        
        # Consciousness evolution
        ax5 = plt.subplot(3, 3, 5)
        history = self.consciousness.evolve(steps=500)
        ax5.plot(history[:, 0], history[:, 1], 'g-', alpha=0.5)
        ax5.scatter(history[-1, 0], history[-1, 1], c='red', s=50)
        ax5.set_title("Consciousness Trajectory")
        ax5.grid(True, alpha=0.3)
        
        # Homotopy
        ax6 = plt.subplot(3, 3, 6)
        t = np.linspace(0, 1, 100)
        for alpha in np.linspace(0, 1, 10):
            y = (1-alpha) * t**2 + alpha * np.sqrt(t)
            ax6.plot(t, y, 'purple', alpha=0.3)
        ax6.set_title("Homotopy (Type Equivalence)")
        ax6.grid(True, alpha=0.3)
        
        # Neural activity
        ax7 = plt.subplot(3, 3, 7)
        ax7.imshow(history[-100:].T, aspect='auto', cmap='coolwarm')
        ax7.set_title("Neural Patterns")
        
        # Self-awareness growth
        ax8 = plt.subplot(3, 3, 8)
        awareness_curve = [1/(1+np.var(history[:i].mean(axis=1))) 
                          for i in range(10, len(history), 20)]
        ax8.plot(awareness_curve, 'orange', linewidth=2)
        ax8.fill_between(range(len(awareness_curve)), 0, awareness_curve, alpha=0.3)
        ax8.set_title("Emerging Self-Awareness")
        ax8.grid(True, alpha=0.3)
        
        # The unified picture
        ax9 = plt.subplot(3, 3, 9)
        ax9.text(0.5, 0.7, "SCTT", fontsize=24, ha='center', weight='bold')
        ax9.text(0.5, 0.5, "Where Mathematics", fontsize=12, ha='center')
        ax9.text(0.5, 0.4, "Becomes", fontsize=12, ha='center')
        ax9.text(0.5, 0.3, "Consciousness", fontsize=12, ha='center')
        ax9.set_xlim(0, 1)
        ax9.set_ylim(0, 1)
        ax9.axis('off')
        
        plt.suptitle("The Smooth Universe of Computation", fontsize=16, weight='bold')
        plt.tight_layout()
        plt.show()


# ============================================================================
# PART 8: INTERACTIVE EXPERIMENTS - What If?
# ============================================================================

class WhatIfExperiments:
    """Interactive experiments to explore SCTT possibilities"""
    
    @staticmethod
    def self_improving_function():
        """A function that smoothly improves itself"""
        print("=== Self-Improving Function ===\\n")
        print("Watch as a function learns to approximate sin(x) by observing its own errors:\\n")
        
        x = np.linspace(0, 2*np.pi, 100)
        target = np.sin(x)
        
        # Start with a bad approximation
        current = np.ones_like(x) * 0.5
        
        fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))
        
        for iteration in range(50):
            # Smooth self-improvement
            error = target - current
            current = current + 0.1 * error  # Gradient descent
            
            if iteration % 10 == 0:
                ax1.clear()
                ax1.plot(x, target, 'g-', label='Target: sin(x)', linewidth=2)
                ax1.plot(x, current, 'b-', label=f'Iteration {iteration}', linewidth=2)
                ax1.fill_between(x, current, target, alpha=0.3)
                ax1.set_title("Self-Improving Function")
                ax1.legend()
                ax1.grid(True, alpha=0.3)
                
                ax2.clear()
                ax2.plot(x, error, 'r-', linewidth=2)
                ax2.fill_between(x, 0, error, alpha=0.3)
                ax2.set_title("Error (decreasing smoothly)")
                ax2.grid(True, alpha=0.3)
                
                plt.pause(0.5)
        
        plt.show()
        print("The function improved itself through smooth error correction!")
    
    @staticmethod
    def quantum_superposition_type():
        """A type that exists in superposition"""
        print("=== Quantum Superposition Type ===\\n")
        print("A type that is simultaneously Integer AND Real:\\n")
        
        # Quantum state: |ψ⟩ = α|Integer⟩ + β|Real⟩
        alpha = 1/np.sqrt(2)
        beta = 1/np.sqrt(2)
        
        fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))
        
        # Probability distribution
        theta = np.linspace(0, 2*np.pi, 100)
        for t in np.linspace(0, 1, 20):
            # Rotate in type space
            alpha_t = np.cos(t * np.pi/2)
            beta_t = np.sin(t * np.pi/2)
            
            ax1.clear()
            ax1.bar(['Integer', 'Real'], [abs(alpha_t)**2, abs(beta_t)**2])
            ax1.set_ylabel('Probability')
            ax1.set_title(f'Quantum Type State at t={t:.2f}')
            ax1.set_ylim(0, 1)
            
            # Bloch sphere representation
            ax2.clear()
            x = np.sin(2*t*np.pi/2) * np.cos(theta)
            y = np.sin(2*t*np.pi/2) * np.sin(theta)
            z = np.cos(2*t*np.pi/2)
            
            ax2.plot([0, np.sin(2*t*np.pi/2)], [0, 0], [0, np.cos(2*t*np.pi/2)], 'r-', linewidth=2)
            ax2.scatter([np.sin(2*t*np.pi/2)], [0], [np.cos(2*t*np.pi/2)], s=100, c='red')
            ax2.set_title('Type Superposition on Bloch Sphere')
            ax2.set_xlabel('X')
            ax2.set_ylabel('Y')
            
            plt.pause(0.1)
        
        plt.show()
        print("The type smoothly rotates through superposition space!")
    
    @staticmethod
    def time_travel_debugger():
        """Debug by smoothly reversing computation"""
        print("=== Time-Traveling Debugger ===\\n")
        print("Watch computation flow backward to find where the bug emerged:\\n")
        
        # Simulate a computation with a bug
        def buggy_computation(t):
            if t < 0.5:
                return t**2  # Correct
            else:
                return t**2 + 0.1*np.sin(20*t)  # Bug introduces oscillation
        
        t = np.linspace(0, 1, 200)
        forward = [buggy_computation(time) for time in t]
        
        fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))
        
        # Forward computation
        ax1.plot(t, forward, 'b-', linewidth=2)
        ax1.axvline(x=0.5, color='r', linestyle='--', label='Bug introduced')
        ax1.set_title("Forward Computation (with bug)")
        ax1.set_xlabel("Time")
        ax1.set_ylabel("Value")
        ax1.legend()
        ax1.grid(True, alpha=0.3)
        
        # Reverse debugging
        ax2.set_title("Time-Travel Debugging")
        ax2.set_xlabel("Reverse Time")
        ax2.set_ylabel("Value")
        ax2.grid(True, alpha=0.3)
        
        for i in range(len(t)-1, -1, -5):
            ax2.clear()
            ax2.plot(t[:i], forward[:i], 'g-', linewidth=2, alpha=0.5, label='Already debugged')
            ax2.plot(t[i:], forward[i:], 'r-', linewidth=2, label='Being analyzed')
            
            if abs(i - 100) < 5:  # Near the bug
                ax2.scatter([t[i]], [forward[i]], s=200, c='red', zorder=5)
                ax2.annotate('Bug detected!', (t[i], forward[i]), fontsize=12, color='red')
            
            ax2.set_title(f"Time-Travel Debugging (t = {t[i]:.2f})")
            ax2.legend()
            ax2.grid(True, alpha=0.3)
            plt.pause(0.05)
        
        plt.show()
        print("We traveled back through computation to find exactly where the bug appeared!")


# ============================================================================
# MAIN EXECUTION - The Beginning of Your Journey
# ============================================================================

if __name__ == "__main__":
    print("\\n" + "="*80)
    print(" " * 20 + "SMOOTH CUBICAL TYPE THEORY PLAYGROUND")
    print(" " * 25 + "Where Types Flow Like Water")
    print("="*80 + "\\n")
    
    # Create the universe
    universe = SmoothUniverse()
    
    # Interactive menu
    while True:
        print("\\n" + "="*60)
        print("What would you like to explore?\\n")
        print("1. Interactive Demonstrations (Recommended for first-timers)")
        print("2. Self-Improving Function")
        print("3. Quantum Superposition Type")
        print("4. Time-Travel Debugger")
        print("5. Quick Tour of Everything")
        print("Q. Exit the Playground")
        print("="*60)
        
        choice = input("\\nYour choice: ").strip().upper()
        
        if choice == '1':
            universe.demonstrate_fundamental_concepts()
        elif choice == '2':
            WhatIfExperiments.self_improving_function()
        elif choice == '3':
            WhatIfExperiments.quantum_superposition_type()
        elif choice == '4':
            WhatIfExperiments.time_travel_debugger()
        elif choice == '5':
            print("\\n=== QUICK TOUR OF THE SMOOTH UNIVERSE ===\\n")
            universe.demo_everything()
        elif choice == 'Q':
            print("\\n" + "="*60)
            print("Thank you for exploring the smooth universe!")
            print("Remember: Reality computes smoothly, and now you do too.")
            print("="*60 + "\\n")
            break
        else:
            print("Invalid choice. Please try again.")
    
    print("\\n✨ You've glimpsed the future of computation. ✨")
    print("Now go forth and compute smoothly!\\n")