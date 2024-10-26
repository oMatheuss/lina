# Exemplos Avançados

## Série Gregoriana

Em matemática, a fórmula de Leibniz para π, que leva o nome de Gottfried Wilhelm Leibniz. Visualização: [Wolfram](https://mathworld.wolfram.com/GregorySeries.html).

```lina
programa GregorySeries

real sum := 0.0
real neg := 0.0-1.0
real flip := neg

para i := 1 ate 1000000000 repetir
    flip *= neg
    sum += flip / (2*i - 1)
fim

saida(sum * 4.0)
```

*Quando executar esse exemplo no browser usar números menores (< 10000), ou a execução leverá muito tempo.*