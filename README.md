# streemap - Set of tree map algorithms

Algorithms:

- [x] Slice and Dice 😵.
- [x] Binary.
- [x] [Squarified](https://www.win.tue.nl/~vanwijk/stm.pdf) by Bruls, Mark; Huizing, Kees; van Wijk, Jarke J. (2000).
- [ ] [Ordered](http://cvs.cs.umd.edu/~ben/papers/Shneiderman2001Ordered.pdf) by Shneiderman, Ben (2001).
- [ ] [Strip](http://www.cs.umd.edu/hcil/trs/2001-18/2001-18.pdf) by Benjamin, Bederson; Shneiderman, Ben; Wattenberg, Martin (2002).
- [ ] [Quantum](http://www.cs.umd.edu/hcil/trs/2001-18/2001-18.pdf) by Benjamin, Bederson; Shneiderman, Ben; Wattenberg, Martin (2002). _Quantized_ variant of other algorithms.

## Example: depth 1

```
[6, 6, 4, 3, 2, 2, 1]
```

**Binary**:

<svg viewBox="0 0 6 4" width="300" height="200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <radialGradient id="g" cx="0.5" cy="0.5" r="0.5"
    fx="0.75" fy="0.6" fr="5%" gradientTransform="scale(2) translate(-0.25, -0.25)">
      <stop offset="0%" stop-color="white"/>
      <stop offset="100%" stop-color="darkseagreen"/>
    </radialGradient>
  </defs>
  <rect x="0" y="0" width="3" height="2" fill="url(#g)" />
  <rect x="0" y="2" width="3" height="2" fill="url(#g)" />
  <rect x="3" y="0" width="3" height="1.3333334" fill="url(#g)" />
  <rect x="3" y="1.3333334" width="1.125" height="2.6666665" fill="url(#g)" />
  <rect x="4.125" y="1.3333334" width="1.875" height="1.0666667" fill="url(#g)" />
  <rect x="4.125" y="2.4" width="1.25" height="1.5999999" fill="url(#g)" />
  <rect x="5.375" y="2.4" width="0.625" height="1.5999999" fill="url(#g)" />
</svg>

**Squarified**:

<svg viewBox="0 0 6 4" width="300" height="200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <radialGradient id="g" cx="0.5" cy="0.5" r="0.5"
    fx="0.75" fy="0.6" fr="5%" gradientTransform="scale(2) translate(-0.25, -0.25)">
      <stop offset="0%" stop-color="white"/>
      <stop offset="100%" stop-color="darkseagreen"/>
    </radialGradient>
  </defs>
  <rect x="0" y="0" width="3" height="2" fill="url(#g)" />
  <rect x="0" y="2" width="3" height="2" fill="url(#g)" />
  <rect x="3" y="0" width="1.7142857" height="2.3333333" fill="url(#g)" />
  <rect x="4.714286" y="0" width="1.2857141" height="2.3333333" fill="url(#g)" />
  <rect x="3" y="2.3333333" width="1.1999999" height="1.6666667" fill="url(#g)" />
  <rect x="4.2" y="2.3333333" width="1.1999999" height="1.6666667" fill="url(#g)" />
  <rect x="5.3999996" y="2.3333333" width="0.60000014" height="1.6666667" fill="url(#g)" />  
</svg>
