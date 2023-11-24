input()
a=list(map(int,input().split()))
b=[]
SUM=0
for i in a:
    SUM+=i
    b.append(SUM)
print(b.count(b[-1])-1)