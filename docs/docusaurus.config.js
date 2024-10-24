// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const { themes } = require("prism-react-renderer");
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

const GITHUB_LINK = "https://github.com/bob-collective/bob";
const LANDING_PAGE = "https://gobob.xyz";
const DOCS_PAGE = "https://docs.gobob.xyz";
const DISCORD = "https://discord.gg/gobob";
const TWITTER = "https://twitter.com/build_on_bob";
const TELEGRAM = "https://t.me/+CyIcLW2nfaFlNDc1";
const FORUM = "https://forum.gobob.xyz";
const PRESS_KIT =
  "https://build-on-bitcoin.notion.site/BOB-Press-Kit-1be66c38713d480eab01000bdd164206";
const BRAND_KIT =
  "https://drive.google.com/drive/u/0/folders/1c30QDkyWgaV8xSEpCXFWJj1WQyUjSm7N";
const ONE_PAGER =
  "https://build-on-bitcoin.notion.site/BOB-Summary-23fad2d446ff467d8551b924eb3338fc";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "BOB - Build on Bitcoin",
  tagline: "A hybrid layer-2 powered by Bitcoin and Ethereum.",
  favicon: "img/favicon.ico",
  url: DOCS_PAGE,
  baseUrl: "/",

  organizationName: "bob-collective",
  projectName: "bob",

  onBrokenLinks: "warn",
  onBrokenAnchors: "throw",
  onBrokenMarkdownLinks: "throw",
  onDuplicateRoutes: "throw",

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  plugins: [
    [
      "@docusaurus/plugin-client-redirects",
      {
        redirects: [
          {
            from: "/",
            to: "/docs",
          },
        ],
      },
    ],
  ],

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: `${GITHUB_LINK}/tree/master/docs/`,
          remarkPlugins: [
            [require("@docusaurus/remark-plugin-npm2yarn"), { sync: true }],
          ],
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
      codeblock: {
        showGithubLink: true,
        githubLinkLabel: "View on GitHub",
        showRunmeLink: false,
        runmeLinkLabel: "Checkout via Runme",
      },

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
          // {
          //   type: "docSidebar",
          //   sidebarId: "buildSidebar",
          //   position: "left",
          //   label: "Build",
          // },
          // {
          //   type: "docSidebar",
          //   sidebarId: "apiSidebar",
          //   position: "left",
          //   label: "API",
          // },
          // {
          //   type: "docSidebar",
          //   sidebarId: "contractSidebar",
          //   position: "left",
          //   label: "Contracts",
          // },
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
              {
                label: "Telegram",
                href: TELEGRAM,
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
              {
                label: "Press Kit",
                href: PRESS_KIT,
              },
              {
                label: "Brand Kit",
                href: BRAND_KIT,
              },
              {
                label: "One Pager",
                href: ONE_PAGER,
              },
            ],
          },
        ],
        copyright: `Built with ❤️ by the BOB Collective. ${new Date().getFullYear()}.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ["solidity"],
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      // announcementBar: {
      //   id: "sign_up",
      //   content:
      //     'Missing something? Want to know more? <a target="_blank" rel="noopener noreferrer" href="https://forms.gle/etYqChR3aahUFuEZ9">Provide feedback</a>.',
      //   backgroundColor: "#f58b00",
      //   textColor: "#fff",
      //   isCloseable: true,
      // },
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
    "docusaurus-theme-github-codeblock",
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
