import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Lina",
  base: process.env.BASE_PATH,
  outDir: "../dist",
  description:
    "Documentação para a linguagem de programação Lina. Uma linguagem voltada para o aprendizado, feito em português.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Introdução", link: "/introducao" },
      { text: "Playground", target: "_self", link: "/playground/" },
    ],

    sidebar: [
      {
        text: "Introdução",
        link: "/introducao",
      },
      {
        text: "Estrutura Básica do Programa",
        link: "/estrutura-basica",
      },
      {
        text: "Tipos de Variaveis",
        base: "/tipos",
        items: [
          { text: "Inteiros e Reais", link: "/numerico" },
          { text: "Texto e Concatenação", link: "/texto" },
          { text: "Booleano e Condicionais", link: "/booleano" },
        ],
      },
      {
        text: "Estruturas de Controle",
        base: "/controle",
        items: [
          { text: "Condicional se", link: "/se" },
          { text: "Laço Para", link: "/para" },
          { text: "Laço Enquanto", link: "/enquanto" },
        ],
      },
      {
        text: "Exemplos Avançados",
        base: "/exemplos",
        items: [
          {
            text: "Número de Euler",
            link: "/euler",
          },
          {
            text: "Sequência de Fibonacci",
            link: "/fibonacci",
          },
          {
            text: "Série Gregoriana",
            link: "/gregory",
          },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/oMatheuss/lina" },
    ],
  },
});
