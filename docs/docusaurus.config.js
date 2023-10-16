// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

const GITHUB_LINK = "https://github.com/bob-collective/bob";
const LANDING_PAGE = "https://gobob.xyz";
const DOCS_PAGE = "https://docs.gobob.xyz";
const DISCORD = "https://discordapp.com/invite/interlay";
const TWITTER = "https://twitter.com/build_on_bob";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "BOB - Build on Bitcoin",
  tagline:
    "A layer 2 stack empowering everyone to build and innovate on Bitcoin.",
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

  plugins: [
    () => ({
      name: "inject-tag",
      injectHtmlTags() {
        return {
          headTags: [
            {
              tagName: "script",
              attributes: {
                href: "https://cdn.usefathom.com/script.js",
                dataSite: "NBNJSTNS",
                defer: true,
              },
            },
          ],
        };
      },
    }),

    [
      'docusaurus-plugin-typedoc',
      // Plugin / TypeDoc options
      {
        entryPoints: ['../sdk/src/*'],
        tsconfig: 'tsconfig.json',
        sidebar: {
          collapsed: false,
          position: 0,
          fullNames: true,
        },
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
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
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
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "GitHub",
                href: GITHUB_LINK,
              },
            ],
          },
        ],
        copyright: `Built with ❤️ by the BOB Collective. ${new Date().getFullYear()}.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      announcementBar: {
        id: "sign_up",
        content:
          'Stay up to date with the BOB collective. <a target="_blank" rel="noopener noreferrer" href="https://interlay.typeform.com/to/qIBFgIZJ">Sign up</a>.',
        backgroundColor: "#f58b00",
        textColor: "#fff",
        isCloseable: true,
      },
    }),

  markdown: {
    mermaid: true,
  },
  themes: ["@docusaurus/theme-mermaid"],
};

module.exports = config;
