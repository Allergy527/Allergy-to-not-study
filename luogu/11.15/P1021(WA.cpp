#include<bits/stdc++.h>

using namespace std;

int main(){
    int n,k,i,j;
    cin>>n>>k;
    long long ans=n;
    cout<<1<<' ';
    for(i=1;i<k;++i){
        cout<<ans<<' ';
        ans=2*ans+1;
    }
    cout<<"\nMAX="<<ans;
    
    getchar();getchar();
    return 0;
}