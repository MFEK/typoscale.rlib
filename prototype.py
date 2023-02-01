import math
typoscale = lambda i: 1 * 2**(i/5)
fifths=list("⅕⅖⅗⅘")
fifths=dict({i+1: fifths[i] for i in range(4)})
fifths[0]=''
tenths=["½" if i%5==0 else str(i)+"⁄10" if not i%2==0 else fifths[(i/2)] for i in range(1,10)]
tenths=dict({i+1: tenths[i] for i in range(9)})
tenths[0]=''
print(", ".join([f"{i} {tenths[int(j*10)]}" for i, j in [(math.floor(typoscale(i)), math.fmod(typoscale(i),1.0)) for i in range(0,64)]]))
