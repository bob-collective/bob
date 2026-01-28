// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const { themes } = require("prism-react-renderer");
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

const GITHUB_LINK = "https://github.com/bob-collective/bob";
const LANDING_PAGE = "https://gobob.xyz";
const DOCS_PAGE = "https://docs.gobob.xyz";
const STATUS_PAGE = "https://conduit-bob.checkly-dashboards.com/";
const DISCORD = "https://discord.gg/gobob";
const TWITTER = "https://x.com/build_on_bob";
const TELEGRAM = "https://t.me/+CyIcLW2nfaFlNDc1";
const FORUM = "https://forum.gobob.xyz";
const PRESS_KIT =
  "https://build-on-bitcoin.notion.site/BOB-Press-Kit-1be66c38713d480eab01000bdd164206";
const BRAND_KIT =
  "https://build-on-bitcoin.notion.site/Brand-Assets-2163a8aad3b48072afe8e134fc04e720";
const ONE_PAGER =
  "https://build-on-bitcoin.notion.site/BOB-Summary-23fad2d446ff467d8551b924eb3338fc";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "BOB - Build on Bitcoin",
  tagline: "A Hybrid Chain powered by Bitcoin and Ethereum.",
  favicon: "img/favicon.ico",
  url: DOCS_PAGE,
  baseUrl: "/",

  organizationName: "bob-collective",
  projectName: "bob",

  onBrokenLinks: "throw",
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
    ['docusaurus-plugin-llms', { pathTransformation: { ignorePaths: ['docs'] } }],
    [
      'docusaurus-plugin-openapi-docs',
      {
        id: 'gateway-api',
        docsPluginId: 'classic',
        config: {
          gateway: {
            specPath: 'https://gateway-api-staging.gobob.xyz/api-doc.json',
            outputDir: 'docs/docs/gateway/api',
            version: '1.0.0',
            hideSendButton: false,
            sidebarOptions: {
              groupPathsBy: 'tag',
            },
            showExtensions: true,
            downloadUrl: 'https://gateway-api-staging.gobob.xyz/api-doc.json',
          },
        },
      },
    ],
    [
      "@docusaurus/plugin-client-redirects",
      {
        redirects: [
          // Folder-level redirects for docs migration
          {
            from: "/learn/introduction",
            to: "/docs/reference",
          },
          {
            from: "/learn/user-guides",
            to: "/docs/user-hub",
          },
          {
            from: "/learn/builder-guides/full-node",
            to: "/docs/bob-chain/full-node",
          },
          {
            from: "/learn/builder-guides/gateway",
            to: "/docs/gateway",
          },
          {
            from: "/learn/builder-guides/create-strategy",
            to: "/docs/gateway",
          },
          {
            from: "/learn/builder-guides/bob-chain",
            to: "/docs/bob-chain",
          },
          {
            from: "/learn/reference/tools",
            to: "/docs/tools/",
          },
          {
            from: "/learn/reference",
            to: "/docs/reference",
          },
          // Deprecated features
          {
            from: "/learn/user-guides/bob-pay",
            to: "/docs/deprecated/bob-pay",
          },
          // PDF redirects
          {
            from: "/docs/reference/mica-whitepaper",
            to: "https://www.notion.so/build-on-bitcoin/BOB-MiCA-White-Paper-2c33a8aad3b48057a35afb9451211fe9"
          }
        ],
      },
    ],
    [
      "@docusaurus/plugin-google-gtag",
      {
        trackingID: "G-VX9XQWCCC5",
        anonymizeIP: true,
      },
    ],
    [
      "@docusaurus/plugin-google-tag-manager",
      {
        containerId: "GTM-NSJLJ9D5",
      },
    ],
  ],
  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        pages: false,
        blog: false,
        docs: {
          routeBasePath: "/",
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: `${GITHUB_LINK}/tree/master/docs/`,
          docItemComponent: "@theme/ApiItem",
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
      api: {
        authPersistance: 'localStorage',
        proxy: undefined,
      },
      languageTabs: [
        {
          language: "curl",
        },
        {
          language: "nodejs",
        },
        {
          language: "rust",
        },
      ],
      docs: {
        sidebar: {
          hideable: true,
        },
      },
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
            "bitcoin, layer 2, scaling, rollup, bitvm, rust, smart contracts, evm, solidity, collective, open source, blockchain",
        },
      ],
      navbar: {
        title: "",
        logo: {
          alt: "BOB Logo",
          src: "img/bob.png",
          srcDark: "img/bob-light.png",
        },
        items: [
          {
            type: "docSidebar",
            sidebarId: "docsSidebar",
            position: "left",
            label: "Docs",
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
          {
            href: STATUS_PAGE,
            label: "Status",
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
        copyright: `Built with 🧡 by the BOB Collective. ${new Date().getFullYear()}.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: [
          "rust",
          "json",
          "bash",
          "solidity",
        ],
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      algolia: {
        appId: "AO8XE1SP27",
        apiKey: "c9e55704810dd96a5013d44fb439186b",
        indexName: "BOB Docs",
        contextualSearch: true,
        replaceSearchResultPathname: {
          from: "localhost:3000",
          to: "docs.gobob.xyz",
        },
        searchPagePath: "search",
        insights: true,
        searchParameters: {},
      },
    }),

  markdown: {
    mermaid: true,
  },
  themes: [
    "@docusaurus/theme-mermaid",
    "docusaurus-theme-github-codeblock",
    "docusaurus-theme-openapi-docs",
  ],
  scripts: [
    {
      src: "https://cdn.usefathom.com/script.js",
      site: "NBNJSTNS",
      defer: true,
    },
  ],
  headTags: [
    {
      tagName: "link",
      attributes: {
        rel: "preload",
        href: "/fonts/IBMPlexSans-Regular.ttf",
        as: "font",
        type: "font/ttf",
        crossorigin: "anonymous",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "preload",
        href: "/fonts/Inter_18pt-Regular.ttf",
        as: "font",
        type: "font/ttf",
        crossorigin: "anonymous",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "preload",
        href: "/fonts/Inter_28pt-SemiBold.ttf",
        as: "font",
        type: "font/ttf",
        crossorigin: "anonymous",
      },
    },
  ],
  staticDirectories: ["static"],
};

module.exports = config;
