# Extruturas de Controle

## Laço `enquanto`

O laço `enquanto` repete um bloco de código enquanto uma expressão condicional for verdadeira. A sintaxe é a seguinte:

```plaintext
enquanto [expressao] repetir
    # corpo do laço
fim
```

### Exemplo

```plaintext
programa Exemplo

inteiro i := 0
enquanto i < 10 repetir
    i := i + 1
    saida("i = ", i)
fim
```
