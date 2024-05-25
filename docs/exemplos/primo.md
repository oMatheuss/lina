# Exemplos Avançados

## Verificar se um Número é Primo

```plaintext
programa VerificarPrimo

inteiro numero := 29
booleano primo := verdadeiro
inteiro i := 2

enquanto i < numero repetir
    se numero % i = 0 repetir
        primo := falso
        interromper
    fim
    i := i + 1
fim

se primo repetir
    saida := "O número " + numero + " é primo."
senao
    saida := "O número " + numero + " não é primo."
fim
```