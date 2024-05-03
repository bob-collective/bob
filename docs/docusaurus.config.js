// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const {themes} = require('prism-react-renderer');
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

const GITHUB_LINK = "https://github.com/bob-collective/bob";
const LANDING_PAGE = "https://gobob.xyz";
const DOCS_PAGE = "https://docs.gobob.xyz";
const DISCORD = "https://discord.gg/gobob";
const TWITTER = "https://twitter.com/build_on_bob";
const TELEGRAM = "https://t.me/+CyIcLW2nfaFlNDc1";
const FORUM = "https://forum.gobob.xyz";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "BOB - Build on Bitcoin",
  tagline:
    "A hybrid layer-2 powered by Bitcoin and Ethereum.",
  favicon: "img/favicon.ico",
  url: DOCS_PAGE,
  baseUrl: "/",

  organizationName: "bob-collective",
  projectName: "bob",

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "throw",

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: `${GITHUB_LINK}/tree/master/docs/`,
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: "img/bob-social-card.png",
      metadata: [
        {
          name: "keywords",
          content:
            "bitcoin, layer 2, scaling, rollup, rust, smart contracts, evm, solidity, collective, open source, blockchain",
        },
      ],
      navbar: {
        title: "",
        logo: {
          alt: "BOB Logo",
          src: "img/logo.svg",
          srcDark: "img/logo-light.svg",
        },
        items: [
          {
            type: "docSidebar",
            sidebarId: "learnSidebar",
            position: "left",
            label: "Learn",
          },
          {
            type: "docSidebar",
            sidebarId: "buildSidebar",
            position: "left",
            label: "Build",
          },
          // {
          //   type: "docSidebar",
          //   sidebarId: "apiSidebar",
          //   position: "left",
          //   label: "API",
          // },
          {
            type: "docSidebar",
            sidebarId: "contractSidebar",
            position: "left",
            label: "Contracts",
          },
          {
            href: DISCORD,
            label: "Discord",
            position: "right",
          },
          {
            href: GITHUB_LINK,
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Community",
            items: [
              {
                label: "Discord",
                href: DISCORD,
              },
              {
                label: "Twitter",
                href: TWITTER,
              },
              // {
              //   label: "Forum",
              //   href: FORUM,
              // },
            ],
          },
          {
            title: "Builders",
            items: [
              {
                label: "Telegram",
                href: TELEGRAM,
              },
              {
                label: "GitHub",
                href: GITHUB_LINK,
              },
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "BOB Homepage",
                href: LANDING_PAGE,
              },
            ],
          },
        ],
        copyright: `Built with ❤️ by the BOB Collective. ${new Date().getFullYear()}.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['solidity'],
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      announcementBar: {
        id: "sign_up",
        content:
          'Missing something? Want to know more? <a target="_blank" rel="noopener noreferrer" href="https://forms.gle/etYqChR3aahUFuEZ9">Provide feedback</a>.',
        backgroundColor: "#f58b00",
        textColor: "#fff",
        isCloseable: true,
      },
    }),

  markdown: {
    mermaid: true,
  },
  themes: [
    "@docusaurus/theme-mermaid",
    [
      "@easyops-cn/docusaurus-search-local",
      {
        indexBlog: false,
        indexDocs: true,
        indexPages: false,
        hashed: true,
        highlightSearchTermsOnTargetPage: true,
        language: ["en"],
      },
    ],
  ],
  scripts: [
    {
      src: "https://cdn.usefathom.com/script.js",
      site: "NBNJSTNS",
      defer: true,
    },
  ],
};

module.exports = config;
