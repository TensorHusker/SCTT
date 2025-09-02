#!/bin/bash

# Create a directory for papers if it doesn't exist
mkdir -p papers

echo "Downloading freely available papers on Cubical Type Theory and related topics..."

# Core Cubical Type Theory papers
echo "Downloading Cubical Type Theory papers..."
curl -L "https://arxiv.org/pdf/1611.02108.pdf" -o "papers/cubical_type_theory_univalence.pdf"
curl -L "https://arxiv.org/pdf/1902.06572.pdf" -o "papers/canonicity_homotopy_canonicity.pdf"
curl -L "https://www.cs.cmu.edu/~rwh/papers/uniform/uniform.pdf" -o "papers/cartesian_cubical_type_theory.pdf"

# HoTT Book
echo "Downloading HoTT Book..."
curl -L "https://homotopytypetheory.org/book/hott-online-13-g2e736d1.pdf" -o "papers/hott_book.pdf"

# Modal Type Theory
echo "Downloading Modal Type Theory papers..."
curl -L "http://www.danielgratzer.com/papers/multimodal-dependent-type-theory.pdf" -o "papers/multimodal_dependent_type_theory.pdf"
curl -L "https://arxiv.org/pdf/2303.02572.pdf" -o "papers/semantics_multimodal_adjoint.pdf"

# Synthetic Differential Geometry
echo "Downloading Synthetic Differential Geometry papers..."
curl -L "https://arxiv.org/pdf/1610.00286.pdf" -o "papers/new_methods_old_spaces_sdg.pdf"
curl -L "https://arxiv.org/pdf/gr-qc/9608013.pdf" -o "papers/sdg_general_relativity.pdf"

# Modalities in HoTT
echo "Downloading Modalities papers..."
curl -L "https://arxiv.org/pdf/1706.07526.pdf" -o "papers/modalities_in_hott.pdf"

# Cubical Agda paper
echo "Downloading Cubical Agda paper..."
curl -L "https://arxiv.org/pdf/1901.05072.pdf" -o "papers/cubical_agda.pdf"

# Cubical Synthetic Homotopy Theory
echo "Downloading Cubical Synthetic Homotopy Theory..."
curl -L "https://arxiv.org/pdf/1904.04117.pdf" -o "papers/cubical_synthetic_homotopy.pdf"

echo "Download complete! Papers saved in ./papers directory"
echo ""
echo "Note: Some books are copyrighted and not freely available:"
echo "- Kock's 'Synthetic Differential Geometry' (Cambridge Press)"
echo "- Moerdijk & Reyes 'Models for Smooth Infinitesimal Analysis' (Springer)"
echo "- Johnstone's 'Sketches of an Elephant' (Oxford)"
echo "- Awodey's 'Category Theory' (Oxford)"
echo ""
echo "These must be purchased or accessed through academic libraries."