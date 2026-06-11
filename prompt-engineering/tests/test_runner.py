"""
Test Runner for Prompt Engineering Techniques
Executes each technique and collects results for evaluation
"""

import sys
import json
from pathlib import Path
from typing import Dict, List, Any

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from implementations.techniques import (
    PromptEngineeringFramework, 
    META_PATTERNS,
    create_test_suite
)

class PromptTechniqueTestRunner:
    """Runs comprehensive tests on all prompt engineering techniques"""
    
    def __init__(self):
        self.framework, self.test_problems = create_test_suite()
        self.test_results = []
        
    def run_individual_tests(self) -> List[Dict[str, Any]]:
        """Test each technique individually"""
        print("=" * 60)
        print("TESTING INDIVIDUAL TECHNIQUES")
        print("=" * 60)
        
        results = []
        for i, (key, technique) in enumerate(self.framework.techniques.items(), 1):
            print(f"\n[{i}/15] Testing: {technique.name}")
            print(f"Category: {technique.category}")
            print(f"Test scenario: {technique.test_scenario}")
            
            # Run test
            result = self.framework.test_technique(key)
            
            # Add test metrics
            result['test_id'] = f"individual_{key}"
            result['test_type'] = 'individual'
            result['technique_key'] = key
            
            results.append(result)
            print(f"✓ Test completed")
            
        return results
    
    def run_combination_tests(self) -> List[Dict[str, Any]]:
        """Test technique combinations for different purposes"""
        print("\n" + "=" * 60)
        print("TESTING TECHNIQUE COMBINATIONS")
        print("=" * 60)
        
        results = []
        for pattern_name, technique_keys in META_PATTERNS.items():
            print(f"\n Testing pattern: {pattern_name}")
            print(f" Techniques: {', '.join(technique_keys)}")
            
            # Select appropriate test problem
            if "creativity" in pattern_name:
                problem = self.test_problems["creativity"]
            elif "analysis" in pattern_name:
                problem = self.test_problems["complexity"]
            elif "solving" in pattern_name:
                problem = self.test_problems["optimization"]
            elif "code" in pattern_name:
                problem = self.test_problems["programming"]
            elif "theoretical" in pattern_name:
                problem = self.test_problems["mathematics"]
            else:
                problem = self.test_problems["ai_safety"]
            
            # Generate combined prompt
            combined_prompt = self.framework.combine_techniques(technique_keys, problem)
            
            result = {
                'test_id': f"combination_{pattern_name}",
                'test_type': 'combination',
                'pattern_name': pattern_name,
                'techniques_used': technique_keys,
                'problem': problem,
                'combined_prompt': combined_prompt,
                'evaluation_pending': True
            }
            
            results.append(result)
            print(f"✓ Combination test completed")
            
        return results
    
    def run_domain_tests(self) -> List[Dict[str, Any]]:
        """Test techniques across different problem domains"""
        print("\n" + "=" * 60)
        print("TESTING ACROSS DOMAINS")
        print("=" * 60)
        
        results = []
        
        # Select best techniques for each domain
        domain_technique_map = {
            "mathematics": ["smooth_reasoning", "metamathematical_compass", "dimensional_collapse"],
            "programming": ["alien_archaeology", "emergent_intelligence", "bootstrap_paradox"],
            "philosophy": ["consciousness_bridge", "godel_loop", "cognitive_parallax"],
            "physics": ["quantum_superposition", "smooth_reasoning", "hyperdimensional_projection"],
            "ai_safety": ["temporal_braiding", "reality_refactoring", "strange_attractor"],
            "creativity": ["reality_refactoring", "bootstrap_paradox", "quantum_superposition"],
            "optimization": ["smooth_reasoning", "strange_attractor", "dimensional_collapse"],
            "complexity": ["emergent_intelligence", "cognitive_parallax", "strange_attractor"]
        }
        
        for domain, problem in self.test_problems.items():
            print(f"\n Testing domain: {domain}")
            print(f" Problem: {problem}")
            
            techniques = domain_technique_map.get(domain, ["recursive_meta"])
            
            for technique_key in techniques:
                if technique_key in self.framework.techniques:
                    result = self.framework.test_technique(technique_key, problem)
                    result['test_id'] = f"domain_{domain}_{technique_key}"
                    result['test_type'] = 'domain'
                    result['domain'] = domain
                    results.append(result)
            
            print(f"✓ Domain tests completed")
            
        return results
    
    def generate_evaluation_metrics(self) -> Dict[str, Any]:
        """Generate metrics for Vote agent evaluation"""
        
        metrics = {
            "evaluation_criteria": self.framework.generate_evaluation_criteria(),
            "scoring_dimensions": {
                "effectiveness": {
                    "weight": 0.25,
                    "sub_criteria": ["problem_solving", "accuracy", "completeness"]
                },
                "innovation": {
                    "weight": 0.20,
                    "sub_criteria": ["novelty", "creativity", "unexpected_insights"]
                },
                "clarity": {
                    "weight": 0.20,
                    "sub_criteria": ["understandability", "structure", "coherence"]
                },
                "depth": {
                    "weight": 0.15,
                    "sub_criteria": ["thoroughness", "nuance", "consideration_of_edge_cases"]
                },
                "practicality": {
                    "weight": 0.10,
                    "sub_criteria": ["implementability", "real_world_applicability", "actionability"]
                },
                "elegance": {
                    "weight": 0.10,
                    "sub_criteria": ["simplicity", "beauty", "minimalism"]
                }
            },
            "comparison_framework": {
                "baseline": "standard_prompting",
                "improvement_threshold": 0.2,  # 20% improvement to be significant
                "statistical_significance": 0.05
            }
        }
        
        return metrics
    
    def run_all_tests(self) -> Dict[str, Any]:
        """Run complete test suite"""
        print("\n" + "=" * 60)
        print("PROMPT ENGINEERING TECHNIQUE TEST SUITE")
        print("=" * 60)
        
        # Run all test types
        individual_results = self.run_individual_tests()
        combination_results = self.run_combination_tests()
        domain_results = self.run_domain_tests()
        
        # Compile results
        all_results = {
            "test_suite": "prompt_engineering_techniques",
            "version": "1.0.0",
            "total_tests": len(individual_results) + len(combination_results) + len(domain_results),
            "individual_tests": individual_results,
            "combination_tests": combination_results,
            "domain_tests": domain_results,
            "evaluation_metrics": self.generate_evaluation_metrics(),
            "meta_patterns": META_PATTERNS,
            "test_problems": self.test_problems
        }
        
        return all_results
    
    def export_for_evaluation(self, results: Dict[str, Any], output_dir: str = "prompt-engineering/results"):
        """Export results for Vote agent evaluation"""
        
        # Create output directory
        Path(output_dir).mkdir(parents=True, exist_ok=True)
        
        # Export full results
        with open(f"{output_dir}/full_test_results.json", 'w') as f:
            json.dump(results, f, indent=2, default=str)
        
        # Export simplified version for Vote agent
        vote_format = {
            "techniques_to_evaluate": [],
            "evaluation_criteria": results["evaluation_metrics"]["evaluation_criteria"],
            "scoring_dimensions": results["evaluation_metrics"]["scoring_dimensions"]
        }
        
        # Add all techniques for evaluation
        for test in results["individual_tests"]:
            vote_format["techniques_to_evaluate"].append({
                "id": test["technique_key"],
                "name": test["technique"],
                "category": test["category"],
                "prompt": test["prompt"],
                "expected_qualities": test["expected_qualities"]
            })
        
        with open(f"{output_dir}/vote_evaluation_input.json", 'w') as f:
            json.dump(vote_format, f, indent=2)
        
        print(f"\n✓ Results exported to {output_dir}/")
        print(f"  - Full results: full_test_results.json")
        print(f"  - Vote agent input: vote_evaluation_input.json")

def main():
    """Main test execution"""
    runner = PromptTechniqueTestRunner()
    
    # Run all tests
    results = runner.run_all_tests()
    
    # Export for evaluation
    runner.export_for_evaluation(results)
    
    # Print summary
    print("\n" + "=" * 60)
    print("TEST SUMMARY")
    print("=" * 60)
    print(f"Total tests run: {results['total_tests']}")
    print(f"Individual technique tests: {len(results['individual_tests'])}")
    print(f"Combination tests: {len(results['combination_tests'])}")
    print(f"Domain-specific tests: {len(results['domain_tests'])}")
    print("\nReady for Vote agent evaluation!")

if __name__ == "__main__":
    main()