# include <stdio.h>
int main()
{
    int bazi[10][10]={{1,1,1,1,1,1,1,1,1,1},{1,2,2,2,2,2,2,2,2,1},
	{1,2,3,3,3,3,3,3,2,1},{1,2,3,4,4,4,4,3,2,1},{1,2,3,4,5,5,4,3,2,1},{1,2,3,4,5,5,4,3,2,1},
	{1,2,3,4,4,4,4,3,2,1},{1,2,3,3,3,3,3,3,2,1},{1,2,2,2,2,2,2,2,2,1},{1,1,1,1,1,1,1,1,1,1}};
    int t,i,j,k;
    char str[11];
    int result=0;
    scanf("%d",&t);
    for (i=0;i<t;i++){
		result=0;
    	for (j=0;j<10;j++){
    		scanf("%s",str);
    		for (k=0;k<10;k++){
    			if (str[k]=='X'){
    				result+=bazi[j][k];
				} 
			}
		}
		printf("%d\n",result);
	}
    getchar();getchar();
	return 0;
}