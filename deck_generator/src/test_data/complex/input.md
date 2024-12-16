# What rules are there for working with complex numbers?
[[20240910112146]]
## Addition
Addition is pretty simple. You just add the real parts and the imaginary parts together like so:
$z_1+z_2 = (a_1 + a_2)+i(b_1+b_2)$

## Multiplication
Multiplication is a bit harder, but can be figured out by using the distributive property.
$z_1*z_2= (a_1 + ib_1)*(a_2 + ib_2)$
$=a_1*a_2 + a_1* ib_2 + a_2*ib_1 + ib_1*ib_2$
$=(a_1 a_2 - b_1 b_2) + i(a_1 b_2 + a_2 b_1)$

$z_1 * z_2 = [r_1, \varphi_1] * [r_2, \varphi_2] = [r_1 r_2, \varphi_1 + \varphi_2]$

## Addition of arguments
The arguments of complex numbers also add.
$arg(z_1)+arg(z_2) = arg(z_1 * z_2) \mod 2\pi$

## Reciprocals
Every complex number $a + ib \not = 0$ has a reciprocal
$$\frac{1}{z} = \frac{a}{a^2+b^2} - i\frac{b}{a^2+b^2}$$
which means that the complex numbers allow unrestricted division through numbers $\not = 0$. This already uses a rule for working with conjugated complex numbers [[20240910122857]] [[20240910123008]].

### Conjugated complex number
Through the use of the conjugated complex number $\overline{z} = a - bi$ the reciprocal can be defined even neater as:
$$\frac{1}{z} = \frac{\overline{z}}{z*z} = \frac{\overline{z}}{|z|^2}$$
## Division
$$\frac{z_1}{z_2} = \frac{z_1}{z_2}*\frac{\overline{z_2}}{\overline{z_2}}=\frac{1}{z_2 \overline{z_2}} * (z_1 * \overline{z_2})$$
## Roots
What is really nice about complex numbers is solving square roots.
When looking at a complex number $z = [R, \psi]$, because of the rules for multiplication the complex number $w = [\sqrt{R}, \frac{\psi}{2}]$ 
fills the condition $w^2=z$ or $w = \sqrt{z}$. This means when searching the n-th root of a number $w_j = [\sqrt[n]{R}, \frac{\psi}{n}+ \frac{2 \pi j }{n}], j \in \{0,1..., n-1\}$

#math #rules #numbers #complexNumbers
