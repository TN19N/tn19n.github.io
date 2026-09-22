# Mustapha Annouaoui — portfolio

Static Astro portfolio hosted on GitHub Pages. The site presents professional work, three selected projects, education, skills, and a downloadable one-page CV.

## Development

Use Node.js 24 or later.

```sh
npm ci
npm run dev
npm run build
npm run preview
```

`npm run build` runs Astro type checks and generates the static site in `dist/`.

## Content

- `src/data/profile.json`: shared professional facts, project descriptions, skills, and links.
- `src/pages/index.astro`: page structure and introductory copy.
- `src/styles/global.css`: responsive design, print styles, and reduced-motion support.
- `public/Mustapha_Annouaoui_CV.pdf`: current CV download.

When updating employment, contact details, or projects, update both the shared data and CV. Review the LinkedIn and GitHub profile wording at the same time.

## Deployment

Pull requests run the production build. Pushes to `main` build and deploy to GitHub Pages through `.github/workflows/cd.yml`. GitHub Pages should use GitHub Actions as its source. No server, database, or client-side framework is required. Fonts are served locally.

The former blog and Dioxus application have been removed. Old routes receive a real 404 page linking to the homepage.
