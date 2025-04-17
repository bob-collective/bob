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
  "https://drive.google.com/drive/u/0/folders/1c30QDkyWgaV8xSEpCXFWJj1WQyUjSm7N";
const ONE_PAGER =
  "https://build-on-bitcoin.notion.site/BOB-Summary-23fad2d446ff467d8551b924eb3338fc";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "BOB - Build on Bitcoin",
  tagline: "A hybrid L2 powered by Bitcoin and Ethereum.",
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
    [
      "@docusaurus/plugin-client-redirects",
      {
        redirects: [
          // Introduction section
          {
            from: "/docs/learn/guides/faq",
            to: "/learn/user-guides/wallet-guide#faqs",
          },
          {
            from: "/learn/introduction/faq",
            to: "/learn/user-guides/wallet-guide#faqs",
          },
          {
            from: "/learn/user-guides/faq",
            to: "/learn/user-guides/wallet-guide#faqs",
          },
          {
            from: "/docs/learn/bob-stack/merged-mining",
            to: "/learn/introduction/roadmap",
          },
          {
            from: "/docs/learn/bob-stack/op-stack",
            to: "/learn/introduction/roadmap",
          },
          {
            from: "/docs/learn/bob-stack/roadmap",
            to: "/learn/introduction/roadmap",
          },

          // User Guides
          {
            from: "/docs/learn/guides/binance-wallet-gateway",
            to: "/learn/user-guides/binance-wallet-gateway",
          },
          {
            from: "/docs/learn/guides/bob-fusion",
            to: "/learn/user-guides/bob-fusion",
          },
          {
            from: "/docs/learn/guides/bob-gateway",
            to: "/learn/user-guides/onboard-to-bob/bob-gateway",
          },
          {
            from: "/docs/learn/guides/bob-pay",
            to: "/learn/user-guides/bob-pay",
          },
          {
            from: "/docs/learn/guides/bob-stake",
            to: "/learn/user-guides/stake-btc",
          },
          {
            from: "/docs/learn/guides/ethereum-bridge",
            to: "/learn/user-guides/onboard-to-bob/ethereum-bridge",
          },
          {
            from: "/docs/build/getting-started/networks",
            to: "/learn/user-guides/networks",
          },
          {
            from: "/docs/learn/guides/onboard-to-bob",
            to: "/learn/user-guides/onboard-to-bob",
          },

          // Builder Guides
          {
            from: "/docs/build/bob-sdk/bridged-btc-gas-fee/account-abstraction",
            to: "/learn/builder-guides/bridged-btc-gas-fee/account-abstraction",
          },
          {
            from: "/docs/build/bob-sdk/bridged-btc-gas-fee",
            to: "/learn/builder-guides/bridged-btc-gas-fee",
          },
          {
            from: "/docs/build/bob-sdk/bridged-btc-gas-fee/meta-transactions",
            to: "/learn/builder-guides/bridged-btc-gas-fee/meta-transactions",
          },
          {
            from: "/docs/build/how-to/full-node",
            to: "/learn/builder-guides/full-node",
          },
          {
            from: "/docs/build/bob-sdk/gateway",
            to: "/learn/builder-guides/gateway",
          },
          {
            from: "/docs/build/getting-started/helloworld",
            to: "/learn/builder-guides/hello-bitcoin",
          },
          {
            from: "/docs/build/getting-started/local-development",
            to: "/learn/builder-guides/local-development",
          },
          {
            from: "/docs/build/bob-sdk/relay",
            to: "/learn/builder-guides/relay",
          },
          {
            from: "/docs/build/bob-sdk/sats-wagmi",
            to: "/learn/builder-guides/sats-wagmi",
          },
          {
            from: "/docs/build/how-to/metamask-snap",
            to: "/learn/builder-guides/metamask-snap",
          },

          // Reference section
          {
            from: "/docs/learn/security/audits",
            to: "/learn/reference/audits",
          },
          {
            from: "/docs/build/bridged-token-addresses",
            to: "/learn/reference/bridged-token-addresses",
          },
          {
            from: "/docs/build/contracts",
            to: "/learn/reference/contracts",
          },
          {
            from: "/docs/learn/security/privileged-roles",
            to: "/learn/reference/privileged-roles",
          },
          {
            from: "/docs/build/examples/zkvm-taproot",
            to: "/learn/reference/tools/rust-zkvm",
          },
          {
            from: '/learn/user-guides/bob-stake',
            to: '/learn/user-guides/stake-btc',
          },
          {
            from: '/learn/user-guides/stake-with-ledger',
            to: '/learn/user-guides/stake-btc#ledger-guide',
          },
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
        additionalLanguages: ["solidity"],
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      // Docusaurus x Algolia documentation: https://docusaurus.io/docs/search#connecting-algolia
      algolia: {
        appId: "AO8XE1SP27",

        // Public API key (safe to commit)
        apiKey: "c9e55704810dd96a5013d44fb439186b",

        indexName: "BOB Docs",
        contextualSearch: true,

        // Optional: Replace parts of the item URLs from Algolia.
        // Useful when using the same search index for multiple deployments using a different baseUrl.
        // You can use regexp or string in the `from` param. For example: localhost:3000 vs myCompany.com/docs
        replaceSearchResultPathname: {
          from: "localhost:3000",
          to: "docs.gobob.xyz",
        },

        // Optional: path for dedicated search page (`false` to disable it)
        searchPagePath: "search",

        // Collect data on user events, such as search queries
        insights: true,

        // Other parameters: https://www.algolia.com/doc/api-reference/search-api-parameters/
        searchParameters: {},

        // Other options: https://docsearch.algolia.com/docs/api/
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
  themes: ["@docusaurus/theme-mermaid", "docusaurus-theme-github-codeblock"],
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
