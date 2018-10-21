[![Build Status](https://travis-ci.com/marcio-diaz/spanish-numbers.svg?branch=master)](https://travis-ci.com/marcio-diaz/spanish-numbers)

# spanish-numbers

Program that translates positive integers to spanish words.

### Examples

Basic usage:
```
> spanish-numbers 281
doscientos ochenta y uno
```

Using long scale and new line format:
```
> spanish-numbers 289348729834792837492873492837492837987 -n
doscientos ochenta y nueve sextillones
trescientos cuarenta y ocho mil setecientos veintinueve quintillones
ochocientos treinta y cuatro mil setecientos noventa y dos cuatrillones
ochocientos treinta y siete mil cuatrocientos noventa y dos trillones
ochocientos setenta y tres mil cuatrocientos noventa y dos billones
ochocientos treinta y siete mil cuatrocientos noventa y dos millones
ochocientos treinta y siete mil novecientos ochenta y siete
```
Using short scale and new line format:
```
> spanish-numbers 289348729834792837492873492837492837987 -n -s
doscientos ochenta y nueve undecillones
trescientos cuarenta y ocho decillones
setecientos veintinueve nonillones
ochocientos treinta y cuatro octillones
setecientos noventa y dos septillones
ochocientos treinta y siete sextillones
cuatrocientos noventa y dos quintillones
ochocientos setenta y tres cuatrillones
cuatrocientos noventa y dos trillones
ochocientos treinta y siete billones
cuatrocientos noventa y dos millones
ochocientos treinta y siete mil novecientos ochenta y siete
```
