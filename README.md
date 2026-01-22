Under benchmark så ger koden resultaten:

test tests::bench_image_simd   ... bench:   4,991,293.75 ns/iter (+/- 208,855.04)
test tests::bench_image_simple ... bench:  14,752,133.30 ns/iter (+/- 899,621.98)
test tests::bench_simd_calc    ... bench:          15.65 ns/iter (+/- 1.88)
test tests::bench_simple_calc  ... bench:           0.32 ns/iter (+/- 0.70)

vilket innebär att simple(skalär) kod är snabbare på att beräkna men tar mer tid för att generera bilden utav datan.
Om ni vill testa på erat system så borde det bara vara att skriva "cargo bench"