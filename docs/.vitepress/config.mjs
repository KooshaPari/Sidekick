import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'Sidekick',
  description: 'Sidekick — Agent Utility Collection for the Phenotype ecosystem',
  cleanUrls: true,
  lastUpdated: true,

  // Only the docs-site pages should be treated as source content.
  // Other markdown files under docs/ are existing repo documentation
  // (boundary/intent/journeys/security/operations/worklogs and similar)
  // and must NOT be picked up by VitePress — they contain template-style
  // markup that the Vue markdown compiler would otherwise choke on.
  srcExclude: [
    'boundary/**',
    'intent/**',
    'journeys/**',
    'operations/**',
    'security/**',
    'worklogs/**',
    'FUNCTIONAL_REQUIREMENTS.md',
    'consolidation_notes.md',
    'slsa.md',
  ],

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Repository', link: 'https://github.com/KooshaPari/Sidekick' },
    ],
    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Getting Started', link: '/getting-started' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/Sidekick' },
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright (c) Koosha Pari and contributors.',
    },
  },
})
