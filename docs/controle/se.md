# Extruturas de Controle

## Condicional `se`

A estrutura condicional `se` executa um bloco de código apenas se uma expressão condicional for verdadeira. A sintaxe é a seguinte:

```lina
se expressao entao
    # corpo condicional
fim
```

### Exemplo

```lina
programa Exemplo

inteiro x := 5
se x > 0 entao
    saida("x é de fato maior que zero :)")
fim
```
