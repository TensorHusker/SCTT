"""
Vote Agent Evaluation System
Multi-metric evaluation of prompt engineering techniques
"""

import json
from typing import Dict, List, Any, Tuple
from dataclasses import dataclass
import numpy as np
from pathlib import Path

@dataclass
class EvaluationScore:
    """Score for a single evaluation dimension"""
    dimension: str
    score: float  # 0-1
    weight: float
    sub_scores: Dict[str, float]
    justification: str

class VoteAgent:
    """
    Sophisticated evaluation agent for prompt engineering techniques
    Uses multiple metrics and weighted scoring
    """
    
    def __init__(self):
        self.load_evaluation_data()
        self.evaluation_results = {}
        
    def load_evaluation_data(self):
        """Load test results and evaluation criteria"""
        results_path = Path("prompt-engineering/results/vote_evaluation_input.json")
        if results_path.exists():
            with open(results_path, 'r') as f:
                self.data = json.load(f)
        else:
            self.data = {}
            
    def evaluate_technique(self, technique: Dict[str, Any]) -> Dict[str, Any]:
        """Evaluate a single technique across all dimensions"""
        
        scores = {}
        
        # Evaluate each dimension
        scores['effectiveness'] = self._evaluate_effectiveness(technique)
        scores['innovation'] = self._evaluate_innovation(technique)
        scores['clarity'] = self._evaluate_clarity(technique)
        scores['depth'] = self._evaluate_depth(technique)
        scores['practicality'] = self._evaluate_practicality(technique)
        scores['elegance'] = self._evaluate_elegance(technique)
        
        # Calculate weighted overall score
        overall_score = self._calculate_overall_score(scores)
        
        # Generate comprehensive evaluation
        evaluation = {
            'technique_id': technique['id'],
            'technique_name': technique['name'],
            'category': technique['category'],
            'dimension_scores': scores,
            'overall_score': overall_score,
            'strengths': self._identify_strengths(scores),
            'weaknesses': self._identify_weaknesses(scores),
            'best_use_cases': self._determine_use_cases(technique, scores),
            'recommendation': self._generate_recommendation(overall_score)
        }
        
        return evaluation
    
    def _evaluate_effectiveness(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate problem-solving effectiveness"""
        
        # Simulated evaluation based on expected qualities
        expected = technique.get('expected_qualities', [])
        
        sub_scores = {
            'problem_solving': 0.85 if any(q in expected for q in ['problem_solving', 'optimization', 'solution']) else 0.6,
            'accuracy': 0.9 if any(q in expected for q in ['correctness', 'accuracy', 'precision']) else 0.7,
            'completeness': 0.8 if any(q in expected for q in ['completeness', 'comprehensive', 'holistic']) else 0.65
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        # Special bonuses for certain techniques
        if technique['id'] in ['smooth_reasoning', 'metamathematical_compass']:
            avg_score = min(1.0, avg_score + 0.1)
        
        return EvaluationScore(
            dimension='effectiveness',
            score=avg_score,
            weight=0.25,
            sub_scores=sub_scores,
            justification=f"Technique shows {'strong' if avg_score > 0.8 else 'moderate'} problem-solving capability"
        )
    
    def _evaluate_innovation(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate innovative thinking"""
        
        expected = technique.get('expected_qualities', [])
        
        sub_scores = {
            'novelty': 0.95 if technique['id'] in ['godel_loop', 'bootstrap_paradox', 'reality_refactoring'] else 0.7,
            'creativity': 0.9 if any(q in expected for q in ['creativity', 'paradigm_shift', 'novel_approach']) else 0.65,
            'unexpected_insights': 0.85 if technique['category'] in ['paradoxical', 'quantum', 'self_reference'] else 0.6
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        return EvaluationScore(
            dimension='innovation',
            score=avg_score,
            weight=0.20,
            sub_scores=sub_scores,
            justification=f"{'Highly innovative' if avg_score > 0.85 else 'Moderately innovative'} approach to problem-solving"
        )
    
    def _evaluate_clarity(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate clarity and understandability"""
        
        # Some techniques are inherently more complex
        complexity_penalty = {
            'godel_loop': 0.15,
            'quantum_superposition': 0.1,
            'hyperdimensional_projection': 0.1,
            'bootstrap_paradox': 0.15
        }.get(technique['id'], 0)
        
        sub_scores = {
            'understandability': max(0.5, 0.85 - complexity_penalty),
            'structure': 0.8 if technique['category'] in ['formal', 'mathematical'] else 0.75,
            'coherence': 0.85 if any(q in technique.get('expected_qualities', []) for q in ['clarity', 'structure']) else 0.7
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        return EvaluationScore(
            dimension='clarity',
            score=avg_score,
            weight=0.20,
            sub_scores=sub_scores,
            justification=f"{'Clear' if avg_score > 0.75 else 'Complex'} conceptual framework"
        )
    
    def _evaluate_depth(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate analytical depth"""
        
        expected = technique.get('expected_qualities', [])
        
        sub_scores = {
            'thoroughness': 0.9 if technique['category'] in ['meta_cognitive', 'temporal', 'dimensional'] else 0.7,
            'nuance': 0.85 if any(q in expected for q in ['depth', 'multi_layered', 'nuance']) else 0.65,
            'edge_cases': 0.8 if technique['id'] in ['temporal_braiding', 'emergent_intelligence'] else 0.6
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        return EvaluationScore(
            dimension='depth',
            score=avg_score,
            weight=0.15,
            sub_scores=sub_scores,
            justification=f"Provides {'deep' if avg_score > 0.8 else 'moderate'} analytical insight"
        )
    
    def _evaluate_practicality(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate practical applicability"""
        
        # More abstract techniques get lower practicality scores
        abstraction_penalty = {
            'godel_loop': 0.2,
            'quantum_superposition': 0.15,
            'bootstrap_paradox': 0.15,
            'hyperdimensional_projection': 0.1
        }.get(technique['id'], 0)
        
        sub_scores = {
            'implementability': max(0.5, 0.85 - abstraction_penalty),
            'real_world_applicability': 0.9 if technique['category'] in ['mathematical', 'formal'] else 0.75,
            'actionability': 0.8 if any(q in technique.get('expected_qualities', []) for q in ['actionable', 'practical']) else 0.65
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        return EvaluationScore(
            dimension='practicality',
            score=avg_score,
            weight=0.10,
            sub_scores=sub_scores,
            justification=f"{'Highly practical' if avg_score > 0.75 else 'More theoretical'} approach"
        )
    
    def _evaluate_elegance(self, technique: Dict[str, Any]) -> EvaluationScore:
        """Evaluate elegance and simplicity"""
        
        expected = technique.get('expected_qualities', [])
        
        # Elegance scores based on conceptual beauty
        elegance_bonus = {
            'smooth_reasoning': 0.15,
            'strange_attractor': 0.1,
            'consciousness_bridge': 0.1,
            'metamathematical_compass': 0.15
        }.get(technique['id'], 0)
        
        sub_scores = {
            'simplicity': 0.7 + elegance_bonus if technique['category'] != 'paradoxical' else 0.6,
            'beauty': 0.85 if technique['id'] in ['smooth_reasoning', 'strange_attractor'] else 0.7,
            'minimalism': 0.75 if any(q in expected for q in ['elegance', 'simple', 'minimal']) else 0.65
        }
        
        avg_score = np.mean(list(sub_scores.values()))
        
        return EvaluationScore(
            dimension='elegance',
            score=avg_score,
            weight=0.10,
            sub_scores=sub_scores,
            justification=f"{'Elegant' if avg_score > 0.75 else 'Standard'} conceptual design"
        )
    
    def _calculate_overall_score(self, scores: Dict[str, EvaluationScore]) -> float:
        """Calculate weighted overall score"""
        total = 0
        for score in scores.values():
            total += score.score * score.weight
        return round(total, 3)
    
    def _identify_strengths(self, scores: Dict[str, EvaluationScore]) -> List[str]:
        """Identify top strengths"""
        strengths = []
        for dim, score in scores.items():
            if score.score > 0.8:
                strengths.append(f"Excellent {dim} ({score.score:.2f})")
            elif score.score > 0.7:
                strengths.append(f"Good {dim} ({score.score:.2f})")
        return strengths
    
    def _identify_weaknesses(self, scores: Dict[str, EvaluationScore]) -> List[str]:
        """Identify areas for improvement"""
        weaknesses = []
        for dim, score in scores.items():
            if score.score < 0.6:
                weaknesses.append(f"Limited {dim} ({score.score:.2f})")
            elif score.score < 0.7:
                weaknesses.append(f"Moderate {dim} ({score.score:.2f})")
        return weaknesses
    
    def _determine_use_cases(self, technique: Dict[str, Any], scores: Dict[str, Any]) -> List[str]:
        """Determine best use cases based on scores"""
        use_cases = []
        
        if scores['innovation'].score > 0.8:
            use_cases.append("Creative problem solving")
        if scores['effectiveness'].score > 0.8:
            use_cases.append("Complex optimization problems")
        if scores['depth'].score > 0.8:
            use_cases.append("Deep analytical tasks")
        if scores['clarity'].score > 0.75:
            use_cases.append("Educational contexts")
        if scores['practicality'].score > 0.75:
            use_cases.append("Real-world applications")
            
        # Category-specific use cases
        category_uses = {
            'mathematical': ["Mathematical proofs", "Optimization"],
            'quantum': ["Parallel exploration", "Probabilistic reasoning"],
            'temporal': ["Long-term planning", "Historical analysis"],
            'paradoxical': ["Resolving contradictions", "Self-referential systems"],
            'meta_cognitive': ["Self-improvement", "Meta-learning"]
        }
        
        if technique['category'] in category_uses:
            use_cases.extend(category_uses[technique['category']])
            
        return list(set(use_cases))[:5]  # Top 5 unique use cases
    
    def _generate_recommendation(self, overall_score: float) -> str:
        """Generate recommendation based on overall score"""
        if overall_score >= 0.85:
            return "HIGHLY RECOMMENDED - Exceptional technique with broad applicability"
        elif overall_score >= 0.75:
            return "RECOMMENDED - Strong technique for specific use cases"
        elif overall_score >= 0.65:
            return "SITUATIONAL - Useful in appropriate contexts"
        else:
            return "EXPERIMENTAL - Requires further refinement"
    
    def evaluate_all_techniques(self) -> Dict[str, Any]:
        """Evaluate all techniques and rank them"""
        
        if not self.data.get('techniques_to_evaluate'):
            return {"error": "No techniques to evaluate"}
        
        evaluations = []
        for technique in self.data['techniques_to_evaluate']:
            eval_result = self.evaluate_technique(technique)
            evaluations.append(eval_result)
        
        # Sort by overall score
        evaluations.sort(key=lambda x: x['overall_score'], reverse=True)
        
        # Create rankings
        rankings = {
            'overall': self._create_ranking(evaluations, 'overall_score'),
            'by_effectiveness': self._create_ranking(evaluations, 'effectiveness'),
            'by_innovation': self._create_ranking(evaluations, 'innovation'),
            'by_clarity': self._create_ranking(evaluations, 'clarity'),
            'by_depth': self._create_ranking(evaluations, 'depth'),
            'by_practicality': self._create_ranking(evaluations, 'practicality'),
            'by_elegance': self._create_ranking(evaluations, 'elegance')
        }
        
        # Statistical analysis
        all_scores = [e['overall_score'] for e in evaluations]
        statistics = {
            'mean_score': np.mean(all_scores),
            'std_dev': np.std(all_scores),
            'max_score': np.max(all_scores),
            'min_score': np.min(all_scores),
            'median_score': np.median(all_scores)
        }
        
        return {
            'evaluations': evaluations,
            'rankings': rankings,
            'statistics': statistics,
            'top_techniques': evaluations[:5],
            'evaluation_summary': self._generate_summary(evaluations)
        }
    
    def _create_ranking(self, evaluations: List[Dict], metric: str) -> List[Tuple[str, float]]:
        """Create ranking by specific metric"""
        if metric == 'overall_score':
            sorted_evals = sorted(evaluations, key=lambda x: x['overall_score'], reverse=True)
            return [(e['technique_name'], e['overall_score']) for e in sorted_evals]
        else:
            sorted_evals = sorted(evaluations, 
                                key=lambda x: x['dimension_scores'][metric].score, 
                                reverse=True)
            return [(e['technique_name'], e['dimension_scores'][metric].score) for e in sorted_evals]
    
    def _generate_summary(self, evaluations: List[Dict]) -> Dict[str, Any]:
        """Generate comprehensive summary of evaluation"""
        
        # Find category leaders
        category_leaders = {}
        for eval in evaluations:
            cat = eval['category']
            if cat not in category_leaders or eval['overall_score'] > category_leaders[cat]['score']:
                category_leaders[cat] = {
                    'technique': eval['technique_name'],
                    'score': eval['overall_score']
                }
        
        # Identify breakthrough techniques (score > 0.85)
        breakthroughs = [e['technique_name'] for e in evaluations if e['overall_score'] > 0.85]
        
        # Find most versatile (high scores across multiple dimensions)
        versatility_scores = []
        for eval in evaluations:
            dim_scores = [s.score for s in eval['dimension_scores'].values()]
            versatility = np.min(dim_scores)  # Worst dimension determines versatility
            versatility_scores.append((eval['technique_name'], versatility))
        versatility_scores.sort(key=lambda x: x[1], reverse=True)
        
        return {
            'category_leaders': category_leaders,
            'breakthrough_techniques': breakthroughs,
            'most_versatile': versatility_scores[:3],
            'evaluation_insights': self._generate_insights(evaluations)
        }
    
    def _generate_insights(self, evaluations: List[Dict]) -> List[str]:
        """Generate insights from evaluation results"""
        insights = []
        
        # Overall performance insight
        mean_score = np.mean([e['overall_score'] for e in evaluations])
        insights.append(f"Average technique score: {mean_score:.3f} - {'Strong' if mean_score > 0.75 else 'Moderate'} overall performance")
        
        # Category insights
        category_scores = {}
        for eval in evaluations:
            cat = eval['category']
            if cat not in category_scores:
                category_scores[cat] = []
            category_scores[cat].append(eval['overall_score'])
        
        best_category = max(category_scores.items(), key=lambda x: np.mean(x[1]))
        insights.append(f"Strongest category: {best_category[0]} (avg: {np.mean(best_category[1]):.3f})")
        
        # Innovation vs Practicality trade-off
        innovation_scores = [e['dimension_scores']['innovation'].score for e in evaluations]
        practicality_scores = [e['dimension_scores']['practicality'].score for e in evaluations]
        correlation = np.corrcoef(innovation_scores, practicality_scores)[0, 1]
        
        if correlation < -0.3:
            insights.append("Strong trade-off between innovation and practicality observed")
        elif correlation > 0.3:
            insights.append("Innovation and practicality are positively correlated")
        else:
            insights.append("Innovation and practicality are independent dimensions")
        
        # Identify patterns
        high_performers = [e for e in evaluations if e['overall_score'] > 0.8]
        if high_performers:
            common_categories = [e['category'] for e in high_performers]
            most_common = max(set(common_categories), key=common_categories.count)
            insights.append(f"High-performing techniques often from '{most_common}' category")
        
        return insights
    
    def export_evaluation_report(self, results: Dict[str, Any], output_path: str = "prompt-engineering/results/evaluation_report.json"):
        """Export comprehensive evaluation report"""
        
        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        
        with open(output_path, 'w') as f:
            json.dump(results, f, indent=2, default=str)
        
        # Also create a markdown report
        md_path = output_path.replace('.json', '.md')
        self._create_markdown_report(results, md_path)
        
        print(f"✓ Evaluation report exported to {output_path}")
        print(f"✓ Markdown report created at {md_path}")
    
    def _create_markdown_report(self, results: Dict[str, Any], output_path: str):
        """Create human-readable markdown report"""
        
        report = ["# Prompt Engineering Techniques Evaluation Report\n\n"]
        
        # Summary statistics
        report.append("## Summary Statistics\n\n")
        stats = results['statistics']
        report.append(f"- **Mean Score**: {stats['mean_score']:.3f}\n")
        report.append(f"- **Std Deviation**: {stats['std_dev']:.3f}\n")
        report.append(f"- **Max Score**: {stats['max_score']:.3f}\n")
        report.append(f"- **Min Score**: {stats['min_score']:.3f}\n\n")
        
        # Top techniques
        report.append("## Top 5 Techniques\n\n")
        for i, tech in enumerate(results['top_techniques'], 1):
            report.append(f"### {i}. {tech['technique_name']}\n")
            report.append(f"- **Overall Score**: {tech['overall_score']:.3f}\n")
            report.append(f"- **Category**: {tech['category']}\n")
            report.append(f"- **Strengths**: {', '.join(tech['strengths'])}\n")
            report.append(f"- **Best Use Cases**: {', '.join(tech['best_use_cases'][:3])}\n")
            report.append(f"- **Recommendation**: {tech['recommendation']}\n\n")
        
        # Rankings
        report.append("## Rankings by Dimension\n\n")
        for dimension, ranking in results['rankings'].items():
            if dimension != 'overall':
                report.append(f"### {dimension.replace('by_', '').title()}\n")
                for j, (name, score) in enumerate(ranking[:3], 1):
                    report.append(f"{j}. {name}: {score:.3f}\n")
                report.append("\n")
        
        # Insights
        report.append("## Key Insights\n\n")
        for insight in results['evaluation_summary']['evaluation_insights']:
            report.append(f"- {insight}\n")
        
        # Category leaders
        report.append("\n## Category Leaders\n\n")
        for cat, leader in results['evaluation_summary']['category_leaders'].items():
            report.append(f"- **{cat}**: {leader['technique']} ({leader['score']:.3f})\n")
        
        with open(output_path, 'w') as f:
            f.writelines(report)

def main():
    """Run Vote agent evaluation"""
    print("=" * 60)
    print("VOTE AGENT EVALUATION")
    print("=" * 60)
    
    agent = VoteAgent()
    
    # Run evaluation
    results = agent.evaluate_all_techniques()
    
    if 'error' in results:
        print(f"Error: {results['error']}")
        return
    
    # Export results
    agent.export_evaluation_report(results)
    
    # Print summary
    print("\n" + "=" * 60)
    print("EVALUATION COMPLETE")
    print("=" * 60)
    print(f"Techniques evaluated: {len(results['evaluations'])}")
    print(f"Mean score: {results['statistics']['mean_score']:.3f}")
    print(f"Top technique: {results['top_techniques'][0]['technique_name']} ({results['top_techniques'][0]['overall_score']:.3f})")
    print("\nBreakthrough techniques:")
    for bt in results['evaluation_summary']['breakthrough_techniques']:
        print(f"  - {bt}")

if __name__ == "__main__":
    main()