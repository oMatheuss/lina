# Exemplos Avançados

## Número de Euler

O número e é uma constante matemática, aproximadamente igual a 2,71828, que é a base dos logaritmos naturais. É o limite de ⁿ quando n se aproxima de infinito, uma expressão que provem da computação dos juros compostos. É o valor da função exponencial, usualmente denotada eˣ, quando avaliada em x = 1.

```lina
programa Euler

real euler

para i := 1 ate 11 repetir
    inteiro fatorial := 1
    inteiro j := 1
    enquanto (j += 1) < i repetir fatorial *= j fim
    euler += 1.0 / fatorial
fim

saida(euler)
```
