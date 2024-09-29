# How does polynomial division work? 
Division of two polynomials [[20240920124236]] works in 2 seperate ways. 

## 1. Long Division
Say we want to divide $x^2 - 4x + 2$ by $x-2$ how would be do that?
1. Take the first term (in this case $x^2$) and check what factor m you need to multiply the divisor ($x$) by in order for the to equal 0 when subtracted (in this case x)
2. Multiply the whole divisor ($x-2$) by m ($x*m-2*m$)
3. Subtract the result from the polynomial (or switch all signs and at, it's the same)
4. Write the next term of the polynomial into the next line (just like with normal long division)
5. Repeat step one until there are no terms left.

## 2. Synthetic Division
If the divisor is in the form $x-c$ then synthetic division is possible. 
Synthetic division works like the following:
| c | $a_n$     | $a_{n-1}$             | … | $a_0$          |
|---|-----------|-----------------------|---|----------------|
|   |           | $r_n*c$               |   | $r_1*c$        |
|   | r_n=a_n   | $r_{n-1}=r_n+a_{n-1}$ |   | $r_n=r_{n-1}*c |
|   | = coeff n | =coeff n-1            |   | = Remainder    |

This yields the new coefficients for the result (quotient) of the division.

#math #polynomials #division