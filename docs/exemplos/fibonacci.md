# Exemplos Avançados

## Fibonacci

```plaintext
programa Fibonacci

inteiro x := 0
inteiro y := 1

saida := "fib(0) = " + x

para i := 1 ate 30 repetir
    inteiro z := x + y
    x := y
    y := z
    saida := "fib(" + i + ") = " + x
fim
```