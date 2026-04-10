#!/usr/bin/env python3
"""ARC-AGI submission pipeline wrapping the SCTT Rust solver.

Usage:
    python submission_pipeline.py <challenges_json> <output_json> [solutions_json]

If solutions_json is provided, evaluates accuracy after solving.
"""

import json
import subprocess
import sys
import os
from pathlib import Path


def find_solver_binary() -> str:
    """Locate the sctt-arc-solve binary."""
    # Check common locations (crate root may be one or two levels up)
    script_dir = Path(__file__).resolve().parent
    crate_dir = script_dir.parent
    # Also check workspace-level target dir (when crate is inside a workspace)
    workspace_dir = crate_dir.parent
    candidates = [
        crate_dir / "target" / "release" / "sctt-arc-solve",
        crate_dir / "target" / "debug" / "sctt-arc-solve",
        workspace_dir / "target" / "release" / "sctt-arc-solve",
        workspace_dir / "target" / "debug" / "sctt-arc-solve",
        Path("sctt-arc-solve"),  # PATH
    ]
    for p in candidates:
        if p.exists():
            return str(p)

    # Try building it
    print("Solver binary not found, building...", file=sys.stderr)
    crate_dir = Path(__file__).parent.parent
    result = subprocess.run(
        ["cargo", "build", "--release", "--features", "arc-solver",
         "--bin", "sctt-arc-solve"],
        cwd=str(crate_dir),
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print(f"Build failed: {result.stderr}", file=sys.stderr)
        sys.exit(1)

    binary = crate_dir / "target" / "release" / "sctt-arc-solve"
    if binary.exists():
        return str(binary)

    print("Could not find solver binary after build", file=sys.stderr)
    sys.exit(1)


def solve_arc(challenges_path: str, output_path: str,
              solutions_path: str = None) -> dict:
    """Call the Rust solver binary and produce Kaggle submission."""
    solver = find_solver_binary()

    cmd = [solver, "--input", challenges_path, "--output", output_path]
    if solutions_path:
        cmd.extend(["--solutions", solutions_path])

    print(f"Running: {' '.join(cmd)}", file=sys.stderr)
    result = subprocess.run(cmd, capture_output=True, text=True)

    # Print solver stderr (progress + results)
    if result.stderr:
        print(result.stderr, file=sys.stderr)

    if result.returncode != 0:
        print(f"Solver failed with code {result.returncode}", file=sys.stderr)
        # Write empty submission as fallback
        with open(output_path, 'w') as f:
            json.dump({}, f)
        return {}

    with open(output_path) as f:
        return json.load(f)


def evaluate_submission(submission_path: str, solutions_path: str) -> tuple:
    """Evaluate submission against solutions. Returns (correct, total)."""
    with open(submission_path) as f:
        submission = json.load(f)
    with open(solutions_path) as f:
        solutions = json.load(f)

    correct = 0
    total = 0

    for task_id, expected in solutions.items():
        total += 1
        submitted = submission.get(task_id, [])

        if not isinstance(expected, list) or not isinstance(submitted, list):
            continue
        if len(submitted) != len(expected):
            continue

        task_ok = True
        for sub, exp in zip(submitted, expected):
            a1 = sub.get("attempt_1")
            a2 = sub.get("attempt_2")
            if a1 != exp and a2 != exp:
                task_ok = False
                break

        if task_ok:
            correct += 1

    return correct, total


def main():
    if len(sys.argv) < 3:
        print("Usage: submission_pipeline.py <challenges.json> <output.json> [solutions.json]",
              file=sys.stderr)
        sys.exit(1)

    challenges_path = sys.argv[1]
    output_path = sys.argv[2]
    solutions_path = sys.argv[3] if len(sys.argv) > 3 else None

    submission = solve_arc(challenges_path, output_path, solutions_path)

    if solutions_path:
        correct, total = evaluate_submission(output_path, solutions_path)
        print(f"\nPython evaluation: {correct}/{total} correct "
              f"({100*correct/max(total,1):.1f}%)")


if __name__ == "__main__":
    main()
