// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import {themes as prismThemes} from 'prism-react-renderer';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'CLN Plugin Tutorial',
  tagline: 'This tutorial will guide you through building your own Core Lightning (CLN) plugin.',
  url: 'https://chrisguida.github.io',
  baseUrl: '/cln-plugin-tutorial/',
  organizationName: 'chrisguida', // Usually your GitHub org/user name.
  projectName: 'cln-plugin-tutorial', // Usually your repo name.
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          routeBasePath: '/',
          sidebarPath: './sidebars.js',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/social-card.jpg',
      colorMode: {
        defaultMode: 'dark',
        disableSwitch: false,
        respectPrefersColorScheme: false,
      },
      navbar: {
        title: 'Tiny Lightning Node Tutorial',
        items: [
          {
            href: 'https://github.com/chrisguida/cln-plugin-tutorial/',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Introduction',
                to: '/',
              },
              {
                label: 'Python Tutorial',
                to: '/python-tutorial',
              }
            ],
          },
          {
            title: 'Social Links',
            items: [
              {
                label: 'X (twitter)',
                href: 'https://x.com/cguida6',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/chrisguida/cln-plugin-tutorial/',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} CLN Plugin Tutorial. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
      },
    }),
};

export default config;
