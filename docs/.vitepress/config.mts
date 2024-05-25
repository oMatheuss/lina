import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Lina",
  description:
    "Documentação para a linguagem de programação Lina. Uma linguagem voltada para o aprendizado, feito em português.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Introdução", link: "/introducao" },
    ],

    sidebar: [
      {
        text: "Introdução",
        link: "/introducao",
      },
      {
        text: "Estrutura Básica do Programa",
        link: "/estrutura-basica-do-programa",
      },
      {
        text: "Tipos de Variaveis",
        base: "/tipos",
        items: [
          { text: "Inteiros e Operações Básicas", link: "/numerico" },
          { text: "Texto e Concatenação", link: "/texto" },
          { text: "Booleano e Condicionais", link: "/booleano" },
        ],
      },
      {
        text: "Estruturas de Controle",
        base: "/controle",
        items: [
          { text: "Condicional se", link: "/se" },
          { text: "Condicional se com senao", link: "/se-senao" },
          { text: "Laço para", link: "/para" },
          { text: "Laço enquanto", link: "/enquanto" },
        ],
      },
      {
        text: "Exemplos Avançados",
        base: "/exemplos",
        items: [
          { 
            text: "Calcular a Média de uma Lista de Números", link: "/media" },
          {
            text: "Encontrar o Maior Número em uma Lista", link: "/maior-numero",
          },
          { 
            text: "Verificar se um Número é Primo", link: "/primo" 
          },
          {
            text: "Fibonacci", link: "/fibonacci"
          }
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/oMatheuss/lina" },
    ],
  },
});
