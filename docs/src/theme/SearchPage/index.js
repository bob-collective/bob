import React from "react";
import Head from "@docusaurus/Head";
import SearchPage from "@theme-original/SearchPage";

// The theme's own search page marks itself noindex with `property="robots"`,
// which crawlers ignore (and which Docusaurus's sitemap plugin does not read,
// so the page also landed in sitemap.xml). `name` is the attribute that counts.
export default function SearchPageWrapper(props) {
  return (
    <>
      <Head>
        <meta name="robots" content="noindex, follow" />
      </Head>
      <SearchPage {...props} />
    </>
  );
}
