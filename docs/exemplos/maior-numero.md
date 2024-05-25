# Exemplos Avançados

## Encontrar o Maior Número em uma Lista

```plaintext
programa MaiorNumeroLista

inteiro numeros[5] := {15, 42, 33, 74, 56}
inteiro maior := numeros[0]
inteiro i := 1

para i ate 5 repetir
    se numeros[i] > maior repetir
        maior := numeros[i]
    fim
fim

saida := "O maior número é: " + maior
```