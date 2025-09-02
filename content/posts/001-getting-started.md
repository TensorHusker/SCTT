---
title: Getting Started with SCTT
date: 2025-01-01T00:00:00Z
tags: introduction, tutorial
---

# Getting Started with Smooth Cubical Type Theory

Welcome to SCTT! This post will guide you through the basics.

## What is SCTT?

Smooth Cubical Type Theory extends cubical type theory with smooth structures, enabling us to reason about differential geometry within type theory.

## Your First SCTT Program

```sctt
-- Define a smooth real function
smooth f : C∞(ℝ, ℝ) 
f x = x² + sin(x)

-- Compute its derivative
df : C∞(ℝ, ℝ)
df x = 2*x + cos(x)
```

## Key Concepts

1. **Smooth Types**: Types with differential structure
2. **Path Types**: Representing equalities and homotopies
3. **Cubical Structure**: Higher-dimensional paths

## Try It Yourself

Visit our [playground](/playground) to experiment with SCTT code interactively!