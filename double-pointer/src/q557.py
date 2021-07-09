class Solution(object):



    def reverseWords(self, s):

        return " ".join(x[::-1] for x in s.split(" "))
            


