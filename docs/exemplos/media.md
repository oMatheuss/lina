# Exemplos Avançados

## Calcular a Média de uma Lista de Números
```plaintext
programa MediaLista

inteiro numeros[5] := {10, 20, 30, 40, 50}
inteiro soma := 0
inteiro i := 0

para i ate 5 repetir
    soma := soma + numeros[i]
fim

real media := soma / 5

saida := "A média é: " + media
````