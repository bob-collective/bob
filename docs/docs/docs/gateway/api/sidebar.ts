import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebar: SidebarsConfig = {
  apisidebar: [
    {
      type: "doc",
      id: "bob-gateway-api",
    },
    {
      type: "category",
      label: "v1",
      items: [
        {
          type: "doc",
          id: "create-order",
          label: "Create a new gateway order.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "get-order",
          label: "Get all orders for a specific ID.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "get-orders",
          label: "Get all user orders.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "get-quote",
          label: "Get a gateway quote.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "get-routes",
          label: "Get all supported routes.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "register-tx",
          label: "Register a tx for a request.",
          className: "api-method patch",
        },
      ],
    },
  ],
};

export default sidebar.apisidebar;
