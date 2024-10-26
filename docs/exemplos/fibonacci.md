# Exemplos Avançados

## Sequência de Fibonacci

Na matemática, a sucessão de Fibonacci, é uma sequência de números inteiros, começando normalmente por 0 e 1, na qual cada termo subsequente corresponde à soma dos dois anteriores.

```lina
programa Fibonacci

inteiro x := 0
inteiro y := 1

saida("fib(0) = ", x)

para i := 1 ate 30 repetir
    inteiro z := x + y
    x := y
    y := z
    saida("fib(" + i + ") = ", x)
fim
```
