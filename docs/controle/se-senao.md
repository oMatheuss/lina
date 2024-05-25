# Estruturas de Controle

## Condicional `se` com `senao`

A estrutura condicional `se` com `senao` permite definir um bloco de código alternativo que será executado se a expressão condicional for falsa.

### Exemplo

```plaintext
programa CondicionalSeSenao

inteiro numero := 15

se numero % 2 = 0 repetir
    saida := "O número é par."
senao
    saida := "O número é ímpar."
fim
```