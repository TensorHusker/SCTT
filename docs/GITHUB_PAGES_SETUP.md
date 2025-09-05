# GitHub Pages Setup for SCTT

This document explains the GitHub Pages configuration for the SCTT (Smooth Cubical Type Theory) website.

## Overview

The repository is configured to automatically deploy to GitHub Pages when changes are pushed to the `main` branch. The website will be available at:

- **Custom Domain**: https://sctt.space (configured via CNAME file)
- **GitHub Pages URL**: https://tensorhusker.github.io/SCTT

## Site Structure

The GitHub Pages deployment includes:

- **Homepage** (`static/index.html`): Landing page with project overview
- **Documentation** (`docs/`): Comprehensive guides and tutorials
- **Book** (`book/`): The complete SCTT book chapters
- **SEO Files**: `robots.txt`, `sitemap.xml`, `404.html`

## Workflow

The site is built and deployed using the GitHub Actions workflow in `.github/workflows/pages.yml`, which:

1. Copies static content from `static/` directory
2. Organizes documentation from `docs/` directory
3. Includes book content from `book/` directory
4. Creates navigation index pages for docs and book sections
5. Deploys to GitHub Pages

## Local Development

To test the site locally:

```bash
# Navigate to the repository root
cd /path/to/SCTT

# Create a test build
mkdir -p _site
cp -r static/* _site/
cp CNAME _site/
mkdir -p _site/docs && cp -r docs/* _site/docs/
mkdir -p _site/book && cp -r book/* _site/book/

# Serve locally
cd _site && python3 -m http.server 8000

# Visit http://localhost:8000
```

## Configuration

### Custom Domain

The custom domain `sctt.space` is configured via the `CNAME` file in the repository root.

### GitHub Pages Settings

In the repository settings:
1. Go to Settings → Pages
2. Source should be set to "GitHub Actions"
3. Custom domain should show `sctt.space`

## Content Management

### Adding Documentation

1. Add new markdown files to the `docs/guides/` directory
2. Update the docs index page links in the workflow if needed
3. Commit and push - the site will automatically update

### Adding Book Chapters

1. Add new markdown files to the `book/` directory
2. Update the book index page links in the workflow if needed
3. Update `book/SUMMARY.md` if using a book generator

### Updating the Homepage

Edit `static/index.html` directly to modify the landing page.

## Troubleshooting

### Site Not Updating

- Check the Actions tab for workflow run status
- Ensure the workflow completed successfully
- GitHub Pages can take a few minutes to reflect changes

### Custom Domain Issues

- Verify the CNAME file contains only `sctt.space`
- Check DNS settings for the domain
- Ensure GitHub Pages is configured to use the custom domain

### 404 Errors

- The `static/404.html` file handles page not found errors
- Ensure internal links use the correct paths
- Check that file names match the links exactly (case sensitive)

## Security

- The workflow only has the minimum required permissions
- No secrets or sensitive data are exposed in the static site
- All content is publicly accessible as intended for documentation