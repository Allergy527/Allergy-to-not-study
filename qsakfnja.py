nums1=[1,2]
nums2=[2,1]

if nums1[-1]==max(nums1) and max(nums2)==nums2[-1]:print(0)
elif max(nums1)!=nums1[-1] and max(nums2)!=nums2[-1]:
    print(-1)
elif   (max(nums1)==nums1[-1] and nums1[-1]<max(nums2)) or (max(nums2)==nums1[-1] and nums2[-1]<max(nums1)) :
    print(-1)
elif   max(nums1)==nums1[-1] and nums1[-1]>max(nums2):
    a=[i for i in range(len(nums1)-1) if nums1[i]>nums2[-1] and nums2[i]<=nums1[-1]]
elif max(nums2)==nums2[-1] and nums2[-1]>max(nums1):
    a=[i for i in range(len(nums2)-1) if nums2[i]>nums1[-1] and nums1[i]<=nums2[-1]]
