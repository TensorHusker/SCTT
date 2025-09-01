#!/usr/bin/env python3
"""
Reference Implementation: SCTT Type Checker
This is a simplified but working type checker showing key ideas
"""

from dataclasses import dataclass
from typing import Optional, Dict, List, Union
from enum import Enum
import sympy as sp

# ============================================================================
# CORE TYPE SYSTEM
# ============================================================================

class TypeKind(Enum):
    """Different kinds of types in SCTT"""
    UNIVERSE = "Type"
    SMOOTH = "Smooth"
    PATH = "Path"
    FUNCTION = "→"
    PRODUCT = "×"
    INTERVAL = "I"

@dataclass
class Type:
    """Base class for all types"""
    kind: TypeKind
    
@dataclass 
class Universe(Type):
    """Type of types"""
    level: int = 0
    
    def __init__(self, level=0):
        super().__init__(TypeKind.UNIVERSE)
        self.level = level

@dataclass
class SmoothType(Type):
    """Smooth type - the key innovation"""
    base_type: Type
    smooth_structure: Optional[str] = None
    
    def __init__(self, base_type):
        super().__init__(TypeKind.SMOOTH)
        self.base_type = base_type

@dataclass
class PathType(Type):
    """Path type for cubical structure"""
    space: Type
    start: 'Term'
    end: 'Term'
    
    def __init__(self, space, start, end):
        super().__init__(TypeKind.PATH)
        self.space = space
        self.start = start
        self.end = end

@dataclass
class FunctionType(Type):
    """Function type A → B"""
    domain: Type
    codomain: Type
    is_smooth: bool = False
    
    def __init__(self, domain, codomain, is_smooth=False):
        super().__init__(TypeKind.FUNCTION)
        self.domain = domain
        self.codomain = codomain
        self.is_smooth = is_smooth

@dataclass
class IntervalType(Type):
    """The interval [0,1]"""
    def __init__(self):
        super().__init__(TypeKind.INTERVAL)

# ============================================================================
# TERMS
# ============================================================================

@dataclass
class Term:
    """Base class for all terms"""
    pass

@dataclass
class Variable(Term):
    name: str

@dataclass
class Lambda(Term):
    param: str
    param_type: Type
    body: Term
    is_smooth: bool = False

@dataclass
class Application(Term):
    func: Term
    arg: Term

@dataclass
class PathLambda(Term):
    """Path abstraction ⟨i⟩ body"""
    param: str  # Interval variable
    body: Term

@dataclass
class PathApp(Term):
    """Path application path @ r"""
    path: Term
    point: Term

@dataclass
class SmoothFunction(Term):
    """Smooth function with derivative data"""
    expr: str  # Symbolic expression
    var: str
    derivatives: List[str] = None

@dataclass
class Interval(Term):
    """Interval values"""
    value: float  # Between 0 and 1

# ============================================================================
# TYPE CHECKING CONTEXT
# ============================================================================

class Context:
    """Typing context Γ"""
    
    def __init__(self):
        self.bindings: Dict[str, Type] = {}
        self.smooth_vars: set = set()
        self.interval_vars: set = set()
    
    def extend(self, var: str, typ: Type) -> 'Context':
        """Extend context with new variable"""
        new_ctx = Context()
        new_ctx.bindings = self.bindings.copy()
        new_ctx.bindings[var] = typ
        new_ctx.smooth_vars = self.smooth_vars.copy()
        new_ctx.interval_vars = self.interval_vars.copy()
        
        if isinstance(typ, SmoothType):
            new_ctx.smooth_vars.add(var)
        elif isinstance(typ, IntervalType):
            new_ctx.interval_vars.add(var)
        
        return new_ctx
    
    def lookup(self, var: str) -> Optional[Type]:
        """Look up variable type"""
        return self.bindings.get(var)

# ============================================================================
# TYPE CHECKER
# ============================================================================

class TypeChecker:
    """Bidirectional type checker for SCTT"""
    
    def __init__(self):
        self.context = Context()
    
    def check(self, term: Term, expected_type: Type) -> bool:
        """Check that term has expected type"""
        
        if isinstance(term, Lambda) and isinstance(expected_type, FunctionType):
            # Check lambda against function type
            new_ctx = self.context.extend(term.param, expected_type.domain)
            checker = TypeChecker()
            checker.context = new_ctx
            
            # Check body has codomain type
            if checker.check(term.body, expected_type.codomain):
                # If function should be smooth, verify smoothness
                if expected_type.is_smooth:
                    return self.verify_smooth_lambda(term)
                return True
            return False
        
        elif isinstance(term, PathLambda) and isinstance(expected_type, PathType):
            # Check path lambda
            new_ctx = self.context.extend(term.param, IntervalType())
            checker = TypeChecker()
            checker.context = new_ctx
            
            # Check body has path space type
            if not checker.check(term.body, expected_type.space):
                return False
            
            # Verify boundary conditions
            body_at_0 = self.substitute(term.body, term.param, Interval(0))
            body_at_1 = self.substitute(term.body, term.param, Interval(1))
            
            return (self.equal_terms(body_at_0, expected_type.start) and
                   self.equal_terms(body_at_1, expected_type.end))
        
        else:
            # Try to infer type and check equality
            inferred = self.infer(term)
            return inferred and self.equal_types(inferred, expected_type)
    
    def infer(self, term: Term) -> Optional[Type]:
        """Infer the type of a term"""
        
        if isinstance(term, Variable):
            return self.context.lookup(term.name)
        
        elif isinstance(term, Application):
            # Infer function type
            func_type = self.infer(term.func)
            if not isinstance(func_type, FunctionType):
                return None
            
            # Check argument has domain type
            if self.check(term.arg, func_type.domain):
                return func_type.codomain
            return None
        
        elif isinstance(term, Lambda):
            # Can't infer lambda type without annotation
            return None
        
        elif isinstance(term, SmoothFunction):
            # Smooth functions have smooth type
            return SmoothType(FunctionType(
                SmoothType(Universe()),  # ℝ
                SmoothType(Universe())   # ℝ
            ))
        
        elif isinstance(term, Interval):
            return IntervalType()
        
        elif isinstance(term, PathApp):
            # Infer path type
            path_type = self.infer(term.path)
            if isinstance(path_type, PathType):
                # Check point is in interval
                if self.check(term.point, IntervalType()):
                    return path_type.space
        
        return None
    
    def verify_smooth_lambda(self, lam: Lambda) -> bool:
        """Verify that a lambda represents a smooth function"""
        if not isinstance(lam.body, SmoothFunction):
            # Try to extract smoothness from body
            return self.is_smooth_term(lam.body)
        
        # Check that all derivatives exist
        sf = lam.body
        try:
            # Verify first few derivatives symbolically
            x = sp.Symbol(sf.var)
            expr = sp.sympify(sf.expr)
            
            for i in range(5):  # Check first 5 derivatives
                expr = sp.diff(expr, x)
                if expr is sp.nan:
                    return False
            
            return True
        except:
            return False
    
    def is_smooth_term(self, term: Term) -> bool:
        """Check if a term represents a smooth value"""
        if isinstance(term, SmoothFunction):
            return True
        elif isinstance(term, Variable):
            return term.name in self.context.smooth_vars
        elif isinstance(term, Application):
            func_type = self.infer(term.func)
            return isinstance(func_type, FunctionType) and func_type.is_smooth
        return False
    
    def equal_types(self, t1: Type, t2: Type) -> bool:
        """Check type equality"""
        if type(t1) != type(t2):
            return False
        
        if isinstance(t1, Universe):
            return t1.level == t2.level
        elif isinstance(t1, SmoothType):
            return self.equal_types(t1.base_type, t2.base_type)
        elif isinstance(t1, FunctionType):
            return (self.equal_types(t1.domain, t2.domain) and
                   self.equal_types(t1.codomain, t2.codomain) and
                   t1.is_smooth == t2.is_smooth)
        elif isinstance(t1, PathType):
            return (self.equal_types(t1.space, t2.space) and
                   self.equal_terms(t1.start, t2.start) and
                   self.equal_terms(t1.end, t2.end))
        
        return True
    
    def equal_terms(self, t1: Term, t2: Term) -> bool:
        """Check term equality (simplified)"""
        # This is where smooth equality checking would go
        # For now, just structural equality
        if type(t1) != type(t2):
            return False
        
        if isinstance(t1, Variable):
            return t1.name == t2.name
        elif isinstance(t1, Interval):
            return abs(t1.value - t2.value) < 1e-10
        # ... more cases
        
        return str(t1) == str(t2)  # Fallback
    
    def substitute(self, term: Term, var: str, value: Term) -> Term:
        """Substitute value for variable in term"""
        if isinstance(term, Variable):
            return value if term.name == var else term
        elif isinstance(term, Lambda):
            if term.param == var:
                return term  # Variable is bound
            new_body = self.substitute(term.body, var, value)
            return Lambda(term.param, term.param_type, new_body, term.is_smooth)
        elif isinstance(term, Application):
            new_func = self.substitute(term.func, var, value)
            new_arg = self.substitute(term.arg, var, value)
            return Application(new_func, new_arg)
        # ... more cases
        
        return term

# ============================================================================
# NORMALIZATION (Simplified NbE)
# ============================================================================

class Value:
    """Runtime values for normalization"""
    pass

@dataclass
class VLambda(Value):
    """Lambda value (closure)"""
    param: str
    body: Term
    env: Dict[str, Value]

@dataclass
class VNeutral(Value):
    """Neutral value (blocked computation)"""
    term: Term

@dataclass
class VInterval(Value):
    """Interval value"""
    value: float

def normalize(term: Term, env: Dict[str, Value] = None) -> Term:
    """Normalize term by evaluation"""
    if env is None:
        env = {}
    
    # Evaluate to value
    val = eval_term(term, env)
    
    # Read back to normal form
    return readback(val)

def eval_term(term: Term, env: Dict[str, Value]) -> Value:
    """Evaluate term to value"""
    if isinstance(term, Variable):
        return env.get(term.name, VNeutral(term))
    
    elif isinstance(term, Lambda):
        return VLambda(term.param, term.body, env)
    
    elif isinstance(term, Application):
        func_val = eval_term(term.func, env)
        arg_val = eval_term(term.arg, env)
        
        if isinstance(func_val, VLambda):
            # Beta reduction
            new_env = func_val.env.copy()
            new_env[func_val.param] = arg_val
            return eval_term(func_val.body, new_env)
        else:
            # Neutral application
            return VNeutral(Application(readback(func_val), readback(arg_val)))
    
    elif isinstance(term, Interval):
        return VInterval(term.value)
    
    return VNeutral(term)

def readback(value: Value) -> Term:
    """Read back value to term"""
    if isinstance(value, VLambda):
        # Generate fresh variable
        var = f"x_{id(value)}"
        body_val = eval_term(value.body, {**value.env, value.param: VNeutral(Variable(var))})
        return Lambda(var, Universe(), readback(body_val))
    
    elif isinstance(value, VNeutral):
        return value.term
    
    elif isinstance(value, VInterval):
        return Interval(value.value)
    
    return Variable("?")  # Shouldn't happen

# ============================================================================
# COHERENCE CHECKING
# ============================================================================

class CoherenceChecker:
    """Check smooth-cubical coherence"""
    
    @staticmethod
    def check_composition_smooth(f: SmoothFunction, g: SmoothFunction) -> bool:
        """Check if composition preserves smoothness"""
        try:
            # Symbolic composition
            x = sp.Symbol(g.var)
            g_expr = sp.sympify(g.expr)
            
            y = sp.Symbol(f.var)
            f_expr = sp.sympify(f.expr)
            
            # Compose
            composed = f_expr.subs(y, g_expr)
            
            # Check if result is smooth (has all derivatives)
            for i in range(5):
                composed = sp.diff(composed, x)
                if composed is sp.nan:
                    return False
            
            return True
        except:
            return False
    
    @staticmethod
    def check_transport_smooth(path: PathLambda, smooth_val: SmoothFunction) -> bool:
        """Check if transport preserves smoothness"""
        # In full SCTT, this would verify that transporting smooth values
        # along paths preserves smooth structure
        
        # For now, check if path is smooth
        if not isinstance(path.body, SmoothFunction):
            return False
        
        return True

# ============================================================================
# EXAMPLE USAGE
# ============================================================================

def example_type_checking():
    """Demonstrate the type checker"""
    print("=" * 60)
    print("SCTT Type Checker Demo")
    print("=" * 60)
    
    checker = TypeChecker()
    
    # Example 1: Smooth function type checking
    print("\n1. Checking smooth function:")
    
    # Create a smooth function sin(x)
    sin_func = SmoothFunction("sin(x)", "x")
    smooth_type = SmoothType(FunctionType(
        SmoothType(Universe()),
        SmoothType(Universe()),
        is_smooth=True
    ))
    
    result = checker.infer(sin_func)
    print(f"   sin(x) : {result.kind if result else 'Failed'}")
    
    # Example 2: Path type checking
    print("\n2. Checking path:")
    
    # Create a path from 0 to 1
    path = PathLambda("t", SmoothFunction("t**2", "t"))
    path_type = PathType(
        SmoothType(Universe()),
        Interval(0),
        Interval(1)
    )
    
    # This should fail because t² doesn't satisfy boundary conditions
    result = checker.check(path, path_type)
    print(f"   Path t² from 0 to 1: {'Valid' if result else 'Invalid (wrong boundaries)'}")
    
    # Example 3: Composition preservation
    print("\n3. Checking composition preserves smoothness:")
    
    f = SmoothFunction("sin(x)", "x")
    g = SmoothFunction("x**2", "x")
    
    coherence = CoherenceChecker()
    result = coherence.check_composition_smooth(f, g)
    print(f"   sin ∘ x² preserves smoothness: {result}")
    
    # Example 4: Lambda with smooth body
    print("\n4. Checking smooth lambda:")
    
    smooth_lambda = Lambda("x", SmoothType(Universe()), 
                          SmoothFunction("exp(x)", "x"), 
                          is_smooth=True)
    func_type = FunctionType(
        SmoothType(Universe()),
        SmoothType(Universe()),
        is_smooth=True
    )
    
    result = checker.check(smooth_lambda, func_type)
    print(f"   λx. exp(x) is smooth: {result}")

if __name__ == "__main__":
    example_type_checking()
    
    print("\n" + "=" * 60)
    print("This is a simplified reference implementation.")
    print("A full SCTT type checker would include:")
    print("- Complete cubical operations (composition, transport)")
    print("- Full smooth equality checking")
    print("- Proper universe levels")
    print("- Dependent types")
    print("- Performance optimizations")
    print("=" * 60)